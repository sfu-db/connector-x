mod plan;

use arrow::ffi::{FFI_ArrowArray, FFI_ArrowSchema};
use connectorx::prelude::*;
use libc::c_char;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::ffi::{CStr, CString};
use std::sync::Arc;

#[repr(C)]
pub struct CXSlice<T> {
    ptr: *const T,
    len: usize,
    capacity: usize,
}

impl<T> CXSlice<T> {
    pub fn new_from_vec(v: Vec<T>) -> Self {
        // If `Vec::into_raw_parts` becomes stable, can directly change to:
        // let (ptr, len, capacity) = v.into_raw_parts();
        // Self {ptr, len, capacity}

        let slice = Self {
            ptr: v.as_ptr(),
            len: v.len(),
            capacity: v.capacity(),
        };
        std::mem::forget(v);
        slice
    }
}

#[repr(C)]
pub struct CXTable {
    name: *const c_char,
    columns: CXSlice<*const c_char>,
}

#[repr(C)]
pub struct CXConnectionInfo {
    name: *const c_char,
    conn: *const c_char,
    schema: CXSlice<CXTable>,
    is_local: bool,
    jdbc_url: *const c_char,
    jdbc_driver: *const c_char,
}

#[repr(C)]
pub struct CXFederatedPlan {
    db_name: *const c_char,
    db_alias: *const c_char,
    sql: *const c_char,
    cardinality: usize,
}

#[cfg(feature = "federation")]
#[no_mangle]
pub unsafe extern "C" fn free_plans(res: *const CXSlice<CXFederatedPlan>) {
    let plans = get_vec::<_>((*res).ptr, (*res).len, (*res).capacity);
    plans.into_iter().for_each(|plan| {
        free_str(plan.db_name);
        free_str(plan.db_alias);
        free_str(plan.sql);
    });
}

#[no_mangle]
pub unsafe extern "C" fn connectorx_rewrite(
    conn_list: *const CXSlice<CXConnectionInfo>,
    query: *const c_char,
) -> CXSlice<CXFederatedPlan> {
    let mut db_map = HashMap::new();
    let conn_slice = unsafe { std::slice::from_raw_parts((*conn_list).ptr, (*conn_list).len) };
    for p in conn_slice {
        let name = unsafe { CStr::from_ptr(p.name) }.to_str().unwrap();
        if p.conn.is_null() {
            let mut table_map: HashMap<String, Vec<String>> = HashMap::new();
            let table_slice = unsafe { std::slice::from_raw_parts(p.schema.ptr, p.schema.len) };
            for t in table_slice {
                let table_name = unsafe { CStr::from_ptr(t.name) }.to_str().unwrap();
                // println!("raw table name: {:?}", table_name);
                let column_slice =
                    unsafe { std::slice::from_raw_parts(t.columns.ptr, t.columns.len) };

                let mut column_names = vec![];
                for &c in column_slice {
                    let column_name = unsafe { CStr::from_ptr(c).to_str().unwrap() };
                    column_names.push(column_name.to_string());
                }
                table_map.insert(table_name.to_string(), column_names);
            }
            let source_info =
                FederatedDataSourceInfo::new_from_manual_schema(table_map, p.is_local);
            db_map.insert(name.to_string(), source_info);
        } else {
            let conn = unsafe { CStr::from_ptr(p.conn) }.to_str().unwrap();
            let jdbc_url = match p.jdbc_url.is_null() {
                true => "",
                false => unsafe { CStr::from_ptr(p.jdbc_url) }.to_str().unwrap(),
            };
            let jdbc_driver = match p.jdbc_driver.is_null() {
                true => "",
                false => unsafe { CStr::from_ptr(p.jdbc_driver) }.to_str().unwrap(),
            };
            // println!("name: {:?}, conn: {:?}", name, conn);
            let source_info = FederatedDataSourceInfo::new_from_conn_str(
                SourceConn::try_from(conn).unwrap(),
                p.is_local,
                jdbc_url,
                jdbc_driver,
            );
            db_map.insert(name.to_string(), source_info);
        }
    }

    let query_str = unsafe { CStr::from_ptr(query) }.to_str().unwrap();
    let j4rs_base = match env::var("CX_LIB_PATH") {
        Ok(val) => Some(val),
        Err(_) => None,
    };
    // println!("j4rs_base: {:?}", j4rs_base);
    let fed_plan: Vec<CXFederatedPlan> = rewrite_sql(query_str, &db_map, j4rs_base.as_deref())
        .unwrap()
        .into_iter()
        .map(|p| p.into())
        .collect();

    CXSlice::<_>::new_from_vec(fed_plan)
}

#[repr(C)]
pub struct CXArray {
    array: *const FFI_ArrowArray,
    schema: *const FFI_ArrowSchema,
}

#[repr(C)]
pub struct CXResult {
    data: CXSlice<CXSlice<CXArray>>,
    header: CXSlice<*const c_char>,
}

pub unsafe fn get_vec<T>(ptr: *const T, len: usize, capacity: usize) -> Vec<T> {
    Vec::from_raw_parts(ptr as *mut T, len, capacity)
}

pub unsafe fn free_str(ptr: *const c_char) {
    let _ = CString::from_raw(ptr as *mut _);
}

#[no_mangle]
pub unsafe extern "C" fn free_result(res: *const CXResult) {
    let header = get_vec::<_>((*res).header.ptr, (*res).header.len, (*res).header.capacity);
    header.into_iter().for_each(|col| free_str(col));

    let rbs = get_vec::<_>((*res).data.ptr, (*res).data.len, (*res).data.capacity);
    rbs.into_iter().for_each(|rb| {
        get_vec::<_>(rb.ptr, rb.len, rb.capacity)
            .into_iter()
            .for_each(|a| {
                // Otherwise memory leak
                std::sync::Arc::from_raw(a.array);
                std::sync::Arc::from_raw(a.schema);
            })
    });
}

#[no_mangle]
pub unsafe extern "C" fn connectorx_scan(conn: *const c_char, query: *const c_char) -> CXResult {
    let conn_str = unsafe { CStr::from_ptr(conn) }.to_str().unwrap();
    let query_str = unsafe { CStr::from_ptr(query) }.to_str().unwrap();
    let source_conn = SourceConn::try_from(conn_str).unwrap();
    let record_batches = get_arrow(&source_conn, None, &[CXQuery::from(query_str)])
        .unwrap()
        .arrow()
        .unwrap();

    // arrow::util::pretty::print_batches(&record_batches[..]).unwrap();

    let names: Vec<*const c_char> = record_batches[0]
        .schema()
        .fields()
        .iter()
        .map(|f| {
            CString::new(f.name().as_str())
                .expect("new CString error")
                .into_raw() as *const c_char
        })
        .collect();

    let mut result = vec![];
    for rb in record_batches {
        let mut cols = vec![];

        for array in rb.columns() {
            let data = array.to_data();
            let array = Arc::new(FFI_ArrowArray::new(&data));
            let schema = Arc::new(
                arrow::ffi::FFI_ArrowSchema::try_from(data.data_type()).expect("export schema c"),
            );
            let array_ptr = Arc::into_raw(array);
            let schema_ptr = Arc::into_raw(schema);

            let cx_array = CXArray {
                array: array_ptr,
                schema: schema_ptr,
            };
            cols.push(cx_array);
        }

        let cx_rb = CXSlice::<CXArray>::new_from_vec(cols);
        result.push(cx_rb);
    }

    let res = CXResult {
        data: CXSlice::<_>::new_from_vec(result),
        header: CXSlice::<_>::new_from_vec(names),
    };

    res
}

#[repr(C)]
pub struct CXSchema {
    types: CXSlice<CXArray>,
    headers: CXSlice<*const c_char>,
}

#[no_mangle]
pub unsafe extern "C" fn free_iter(iter: *mut Box<dyn RecordBatchIterator>) {
    let _ = Box::from_raw(iter);
}

#[no_mangle]
pub unsafe extern "C" fn free_schema(schema: *mut CXSchema) {
    let res = Box::from_raw(schema);

    let header = get_vec::<_>(res.headers.ptr, res.headers.len, res.headers.capacity);
    header.into_iter().for_each(|col| free_str(col));

    get_vec::<_>(res.types.ptr, res.types.len, res.types.capacity)
        .into_iter()
        .for_each(|a| {
            std::sync::Arc::from_raw(a.array);
            std::sync::Arc::from_raw(a.schema);
        });
}

#[no_mangle]
pub unsafe extern "C" fn free_record_batch(rb: *mut CXSlice<CXArray>) {
    let slice = Box::from_raw(rb);
    get_vec::<_>(slice.ptr, slice.len, slice.capacity)
        .into_iter()
        .for_each(|a| {
            std::sync::Arc::from_raw(a.array);
            std::sync::Arc::from_raw(a.schema);
        })
}

#[no_mangle]
pub unsafe extern "C" fn connectorx_scan_iter(
    conn: *const c_char,
    queries: *const CXSlice<*const c_char>,
    batch_size: usize,
) -> *mut Box<dyn RecordBatchIterator> {
    let conn_str = unsafe { CStr::from_ptr(conn) }.to_str().unwrap();
    let source_conn = SourceConn::try_from(conn_str).unwrap();

    let query_slice = unsafe { std::slice::from_raw_parts((*queries).ptr, (*queries).len) };

    let mut query_vec = vec![];
    for &q in query_slice {
        let query = unsafe { CStr::from_ptr(q).to_str().unwrap() };
        query_vec.push(CXQuery::from(query));
    }

    let arrow_iter: Box<dyn RecordBatchIterator> =
        new_record_batch_iter(&source_conn, None, query_vec.as_slice(), batch_size);

    Box::into_raw(Box::new(arrow_iter))
}

#[no_mangle]
pub unsafe extern "C" fn connectorx_get_schema(
    iter: *mut Box<dyn RecordBatchIterator>,
) -> *mut CXSchema {
    let arrow_iter = unsafe { &*iter };
    let (empty_batch, names) = arrow_iter.get_schema();
    let mut cols = vec![];
    for array in empty_batch.columns() {
        let data = array.to_data();
        let array = Arc::new(arrow::ffi::FFI_ArrowArray::new(&data));
        let schema = Arc::new(
            arrow::ffi::FFI_ArrowSchema::try_from(data.data_type()).expect("export schema c"),
        );
        let array_ptr = Arc::into_raw(array);
        let schema_ptr = Arc::into_raw(schema);
        let cx_array = CXArray {
            array: array_ptr,
            schema: schema_ptr,
        };
        cols.push(cx_array);
    }

    let names: Vec<*const c_char> = names
        .iter()
        .map(|name| {
            CString::new(name.as_str())
                .expect("new CString error")
                .into_raw() as *const c_char
        })
        .collect();

    let res = Box::new(CXSchema {
        types: CXSlice::<_>::new_from_vec(cols),
        headers: CXSlice::<_>::new_from_vec(names),
    });

    Box::into_raw(res)
}

#[no_mangle]
pub unsafe extern "C" fn connectorx_prepare(iter: *mut Box<dyn RecordBatchIterator>) {
    let arrow_iter = unsafe { &mut *iter };
    arrow_iter.prepare();
}

#[no_mangle]
pub unsafe extern "C" fn connectorx_iter_next(
    iter: *mut Box<dyn RecordBatchIterator>,
) -> *mut CXSlice<CXArray> {
    let arrow_iter = unsafe { &mut *iter };
    match arrow_iter.next_batch() {
        Some(rb) => {
            let mut cols = vec![];

            for array in rb.columns() {
                let data = array.to_data();
                let array = Arc::new(arrow::ffi::FFI_ArrowArray::new(&data));
                let schema =
                    Arc::new(FFI_ArrowSchema::try_from(data.data_type()).expect("export schema c"));
                let array_ptr = Arc::into_raw(array);
                let schema_ptr = Arc::into_raw(schema);

                let cx_array = CXArray {
                    array: array_ptr,
                    schema: schema_ptr,
                };
                cols.push(cx_array);
            }

            let cx_rb = Box::new(CXSlice::<CXArray>::new_from_vec(cols));
            Box::into_raw(cx_rb)
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn connectorx_set_thread_num(num: usize) {
    set_global_num_thread(num);
}
