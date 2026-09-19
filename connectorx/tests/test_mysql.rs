#![cfg(all(feature = "src_mysql", feature = "dst_arrow"))]

mod test_db;

use arrow::{
    array::{
        Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, Int8Array, StringArray,
        UInt16Array, UInt32Array, UInt64Array, UInt8Array,
    },
    datatypes::DataType,
    record_batch::RecordBatch,
};
use connectorx::{
    destinations::arrow::ArrowDestination,
    prelude::*,
    sources::mysql::{BinaryProtocol, MySQLSource, TextProtocol},
    sql::CXQuery,
    transports::MySQLArrowTransport,
};

#[test]
fn test_mysql() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = test_db::mysql_url();

    let queries = [
        CXQuery::naked("select * from test_table where test_int <= 2"),
        CXQuery::naked("select * from test_table where test_int > 2"),
    ];

    let builder = MySQLSource::<BinaryProtocol>::new(&dburl, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, MySQLArrowTransport<BinaryProtocol>>::new(
        builder,
        &mut destination,
        &queries,
        Some(String::from("select * from test_table")),
    );
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    verify_arrow_results(result);
}

#[test]
fn test_mysql_text() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = test_db::mysql_url();

    let queries = [
        CXQuery::naked("select * from test_table where test_int <= 2"),
        CXQuery::naked("select * from test_table where test_int > 2"),
    ];

    let builder = MySQLSource::<TextProtocol>::new(&dburl, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, MySQLArrowTransport<TextProtocol>>::new(
        builder,
        &mut destination,
        &queries,
        None,
    );
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    verify_arrow_results(result);
}

#[test]
fn test_mysql_pre_execution_queries() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = test_db::mysql_url();

    let queries = [CXQuery::naked(
        "SELECT @@SESSION.max_execution_time, @@SESSION.wait_timeout",
    )];

    let pre_execution_queries = [
        String::from("SET SESSION max_execution_time = 2151"),
        String::from("SET SESSION wait_timeout = 2252"),
    ];

    let builder = MySQLSource::<BinaryProtocol>::new(&dburl, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let mut dispatcher = Dispatcher::<_, _, MySQLArrowTransport<BinaryProtocol>>::new(
        builder,
        &mut destination,
        &queries,
        None,
    );
    dispatcher.set_pre_execution_queries(Some(&pre_execution_queries));
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();

    assert_eq!(result.len(), 1);

    assert!(result[0]
        .column(0)
        .as_any()
        .downcast_ref::<UInt64Array>()
        .unwrap()
        .eq(&UInt64Array::from(vec![2151])));

    assert!(result[0]
        .column(1)
        .as_any()
        .downcast_ref::<UInt64Array>()
        .unwrap()
        .eq(&UInt64Array::from(vec![2252])));
}

#[test]
fn test_mysql_partitioned_pre_execution_queries() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = test_db::mysql_url();

    let queries = [
        CXQuery::naked(
            "SELECT 'max_execution_time' AS name, @@SESSION.max_execution_time AS setting",
        ),
        CXQuery::naked("SELECT 'wait_timeout' AS name, @@SESSION.wait_timeout AS setting"),
    ];

    let pre_execution_queries = [
        String::from("SET SESSION max_execution_time = 2151"),
        String::from("SET SESSION wait_timeout = 2252"),
    ];

    let builder = MySQLSource::<BinaryProtocol>::new(&dburl, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let mut dispatcher = Dispatcher::<_, _, MySQLArrowTransport<BinaryProtocol>>::new(
        builder,
        &mut destination,
        &queries,
        None,
    );
    dispatcher.set_pre_execution_queries(Some(&pre_execution_queries));
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();

    assert!(result.len() == 2);

    let mut result_map = std::collections::HashMap::new();
    for record_batch in result {
        let name = record_batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .value(0)
            .to_string();
        let setting = record_batch
            .column(1)
            .as_any()
            .downcast_ref::<UInt64Array>()
            .unwrap()
            .value(0);
        result_map.insert(name, setting);
    }

    assert_eq!(result_map.get("max_execution_time"), Some(&2151u64));
    assert_eq!(result_map.get("wait_timeout"), Some(&2252u64));
}

pub fn verify_arrow_results(result: Vec<RecordBatch>) {
    assert!(result.len() == 2);

    for r in result {
        match r.num_rows() {
            2 => {
                assert!(r
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![1, 2])));
                assert!(r
                    .column(1)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![1.1, 2.2])));
                assert!(r
                    .column(2)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec!["odd", "even"])));
                assert!(r
                    .column(3)
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![None, None])));
            }
            4 => {
                assert!(r
                    .column(0)
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![3, 4, 5, 6])));
                assert!(r
                    .column(1)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .unwrap()
                    .eq(&Float64Array::from(vec![3.3, 4.4, 5.5, 6.6])));
                assert!(r
                    .column(2)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .eq(&StringArray::from(vec!["odd", "even", "odd", "even"])));
                assert!(r
                    .column(3)
                    .as_any()
                    .downcast_ref::<Int32Array>()
                    .unwrap()
                    .eq(&Int32Array::from(vec![None, None, None, None])));
            }
            _ => {
                println!("got {} rows in a record batch!", r.num_rows());
                unreachable!()
            }
        }
    }
}

// Unit tests for MySQL binary-vs-text type detection (no DB required).
// Detection keys on the "binary" charset (collation id 63), not BINARY_FLAG:
// charset 63 => binary data (BINARY/VARBINARY/BLOB), any other id => text.
// Charset ids used below: 63 = binary, 45 = utf8mb4_general_ci, 46 = utf8mb4_bin, 65 = ascii_bin.

use connectorx::sources::mysql::MySQLTypeSystem;
use r2d2_mysql::mysql::consts::{ColumnFlags, ColumnType};

/// BINARY shares its type code with CHAR (MYSQL_TYPE_STRING). The binary charset
/// (63) is what marks it as binary, so it must map to TinyBlob, not Char.
#[test]
fn test_binary_is_binary() {
    let mut f = ColumnFlags::empty();
    f.insert(ColumnFlags::BINARY_FLAG);
    match MySQLTypeSystem::from((&ColumnType::MYSQL_TYPE_STRING, &f, 63)) {
        MySQLTypeSystem::TinyBlob(true) => {}
        o => panic!("BINARY should be TinyBlob, got {:?}", o),
    }
}

/// CHAR uses a real (non-binary) charset, so MYSQL_TYPE_STRING maps to Char.
#[test]
fn test_char_is_text() {
    match MySQLTypeSystem::from((&ColumnType::MYSQL_TYPE_STRING, &ColumnFlags::empty(), 45)) {
        MySQLTypeSystem::Char(true) => {}
        o => panic!("CHAR should be Char, got {:?}", o),
    }
}

/// VARBINARY shares its type code with VARCHAR (MYSQL_TYPE_VAR_STRING). The binary
/// charset (63) marks it as binary, so it must map to Blob, not VarChar.
#[test]
fn test_varbinary_is_binary() {
    let mut f = ColumnFlags::empty();
    f.insert(ColumnFlags::BINARY_FLAG);
    match MySQLTypeSystem::from((&ColumnType::MYSQL_TYPE_VAR_STRING, &f, 63)) {
        MySQLTypeSystem::Blob(true) => {}
        o => panic!("VARBINARY should be Blob, got {:?}", o),
    }
}

/// VARCHAR uses a real (non-binary) charset, so MYSQL_TYPE_VAR_STRING maps to VarChar.
#[test]
fn test_varchar_is_text() {
    match MySQLTypeSystem::from((
        &ColumnType::MYSQL_TYPE_VAR_STRING,
        &ColumnFlags::empty(),
        45,
    )) {
        MySQLTypeSystem::VarChar(true) => {}
        o => panic!("VARCHAR should be VarChar, got {:?}", o),
    }
}

/// Regression: a *_bin collation sets BINARY_FLAG but the column is still text.
/// charset != 63 must win over the flag, so VARCHAR ... COLLATE *_bin => VarChar.
#[test]
fn test_varchar_bin_collation_is_text() {
    let mut f = ColumnFlags::empty();
    f.insert(ColumnFlags::BINARY_FLAG); // *_bin collations DO set BINARY_FLAG
    match MySQLTypeSystem::from((&ColumnType::MYSQL_TYPE_VAR_STRING, &f, 65)) {
        MySQLTypeSystem::VarChar(true) => {}
        o => panic!("VARCHAR COLLATE *_bin must be VarChar, got {:?}", o),
    }
}

/// CHAR ... COLLATE *_bin is still text (charset != 63), despite BINARY_FLAG.
#[test]
fn test_char_bin_collation_is_text() {
    let mut f = ColumnFlags::empty();
    f.insert(ColumnFlags::BINARY_FLAG);
    match MySQLTypeSystem::from((&ColumnType::MYSQL_TYPE_STRING, &f, 46)) {
        MySQLTypeSystem::Char(true) => {}
        o => panic!("CHAR COLLATE *_bin must be Char, got {:?}", o),
    }
}

/// TEXT shares its type code with BLOB; a *_bin collation (charset != 63) is still text.
#[test]
fn test_text_bin_collation_is_text() {
    let mut f = ColumnFlags::empty();
    f.insert(ColumnFlags::BINARY_FLAG);
    match MySQLTypeSystem::from((&ColumnType::MYSQL_TYPE_BLOB, &f, 46)) {
        MySQLTypeSystem::VarChar(true) => {}
        o => panic!("TEXT COLLATE *_bin must be text, got {:?}", o),
    }
}

/// BLOB has the binary charset (63), so it must map to Blob.
#[test]
fn test_blob_is_binary() {
    match MySQLTypeSystem::from((&ColumnType::MYSQL_TYPE_BLOB, &ColumnFlags::empty(), 63)) {
        MySQLTypeSystem::Blob(true) => {}
        o => panic!("BLOB should be Blob, got {:?}", o),
    }
}

#[test]
fn test_mysql_tinyint_not_bool() {
    let result = run_mysql_query(
        "SELECT test_tiny FROM test_types WHERE test_tiny IS NOT NULL ORDER BY test_tiny",
        true,
    );
    assert_eq!(result.len(), 1);

    // TINYINT values must be preserved as Int8, not collapsed to Boolean.
    // Before this fix, -128 and 127 would both become `true`.
    assert!(result[0]
        .column(0)
        .as_any()
        .downcast_ref::<Int8Array>()
        .unwrap()
        .eq(&Int8Array::from(vec![-128i8, 127])));
}

#[test]
fn test_mysql_tinyint_not_bool_text() {
    let result = run_mysql_query(
        "SELECT test_tiny FROM test_types WHERE test_tiny IS NOT NULL ORDER BY test_tiny",
        false,
    );
    assert_eq!(result.len(), 1);

    assert!(result[0]
        .column(0)
        .as_any()
        .downcast_ref::<Int8Array>()
        .unwrap()
        .eq(&Int8Array::from(vec![-128i8, 127])));
}

#[test]
fn test_mysql_bigint_unsigned_not_float() {
    let result = run_mysql_query(
        "SELECT test_longlong_unsigned FROM test_types WHERE test_longlong_unsigned IS NOT NULL ORDER BY test_longlong_unsigned",
        true,
    );
    assert_eq!(result.len(), 1);

    // BIGINT UNSIGNED must be UInt64, not Float64.
    // Float64 loses precision for values exceeding 2^53 (see #890).
    let col = result[0]
        .column(0)
        .as_any()
        .downcast_ref::<UInt64Array>()
        .unwrap();

    assert_eq!(col.value(0), 0u64);
}

#[test]
fn test_mysql_bigint_unsigned_not_float_text() {
    let result = run_mysql_query(
        "SELECT test_longlong_unsigned FROM test_types WHERE test_longlong_unsigned IS NOT NULL ORDER BY test_longlong_unsigned",
        false,
    );
    assert_eq!(result.len(), 1);

    let col = result[0]
        .column(0)
        .as_any()
        .downcast_ref::<UInt64Array>()
        .unwrap();

    assert_eq!(col.value(0), 0u64);
}

fn assert_str_collation_schema(result: &[RecordBatch]) {
    let schema = result[0].schema();

    // Text columns, including *_bin collations, must be UTF-8 strings (not binary).
    for col in ["vc_general", "vc_bin", "vc_ascii", "txt_general", "txt_bin"] {
        let dt = schema.field_with_name(col).unwrap().data_type();
        assert!(
            matches!(dt, DataType::Utf8 | DataType::LargeUtf8),
            "{} should be a string type, got {:?}",
            col,
            dt
        );
    }

    // Genuine binary types must stay binary.
    for col in ["vb", "bin_col", "bl"] {
        let dt = schema.field_with_name(col).unwrap().data_type();
        assert!(
            matches!(dt, DataType::Binary | DataType::LargeBinary),
            "{} should be a binary type, got {:?}",
            col,
            dt
        );
    }
}

/// Run a single naked query through the Arrow transport using either the
/// binary or text protocol.
fn run_mysql_query(query: &str, binary: bool) -> Vec<RecordBatch> {
    let _ = env_logger::builder().is_test(true).try_init();
    let dburl = test_db::mysql_url();
    let queries = [CXQuery::naked(query)];
    let mut destination = ArrowDestination::new();

    if binary {
        let builder = MySQLSource::<BinaryProtocol>::new(&dburl, 1).unwrap();
        Dispatcher::<_, _, MySQLArrowTransport<BinaryProtocol>>::new(
            builder,
            &mut destination,
            &queries,
            None,
        )
        .run()
        .unwrap();
    } else {
        let builder = MySQLSource::<TextProtocol>::new(&dburl, 1).unwrap();
        Dispatcher::<_, _, MySQLArrowTransport<TextProtocol>>::new(
            builder,
            &mut destination,
            &queries,
            None,
        )
        .run()
        .unwrap();
    }

    destination.arrow().unwrap()
}

#[test]
fn test_mysql_year() {
    // YEAR is mapped to Int16. The value 2155 exceeds i8::MAX (127), so this
    // test immediately catches any regression that narrows YEAR to Int8.
    for binary in [true, false] {
        let result = run_mysql_query(
            "SELECT test_year FROM test_types WHERE test_year IS NOT NULL ORDER BY test_year",
            binary,
        );
        assert_eq!(result.len(), 1);
        assert!(result[0]
            .column(0)
            .as_any()
            .downcast_ref::<Int16Array>()
            .unwrap()
            .eq(&Int16Array::from(vec![1901i16, 2155])));
    }
}

#[test]
fn test_mysql_string_collation_types() {
    let _ = env_logger::builder().is_test(true).try_init();
    let dburl = test_db::mysql_url();

    let queries = [CXQuery::naked(
        "SELECT vc_general, vc_bin, vc_ascii, txt_general, txt_bin, vb, bin_col, bl \
         FROM test_str_collation ORDER BY test_id",
    )];

    let builder = MySQLSource::<BinaryProtocol>::new(&dburl, 1).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, MySQLArrowTransport<BinaryProtocol>>::new(
        builder,
        &mut destination,
        &queries,
        None,
    );
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    assert_eq!(result.len(), 1);

    // Text columns (including *_bin collations) must map to strings; only the
    // binary charset (charsetnr == 63) is binary. Guards against the regression
    // where BINARY_FLAG-based detection turned *_bin text into LargeBinary.
    assert_str_collation_schema(&result);
}

#[test]
fn test_mysql_string_collation_types_text() {
    let _ = env_logger::builder().is_test(true).try_init();
    let dburl = test_db::mysql_url();

    let queries = [CXQuery::naked(
        "SELECT vc_general, vc_bin, vc_ascii, txt_general, txt_bin, vb, bin_col, bl \
         FROM test_str_collation ORDER BY test_id",
    )];

    let builder = MySQLSource::<TextProtocol>::new(&dburl, 1).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher = Dispatcher::<_, _, MySQLArrowTransport<TextProtocol>>::new(
        builder,
        &mut destination,
        &queries,
        None,
    );
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    assert_eq!(result.len(), 1);

    assert_str_collation_schema(&result);
}

#[test]
fn test_mysql_tiny_unsigned() {
    // TINYINT UNSIGNED -> UInt8. 255 = u8::MAX, so any accidental narrowing to
    // i8 or signed conversion makes this test fail.
    for binary in [true, false] {
        let result = run_mysql_query(
            "SELECT test_tiny_unsigned FROM test_types \
             WHERE test_tiny_unsigned IS NOT NULL ORDER BY test_tiny_unsigned",
            binary,
        );
        assert_eq!(result.len(), 1);
        assert!(result[0]
            .column(0)
            .as_any()
            .downcast_ref::<UInt8Array>()
            .unwrap()
            .eq(&UInt8Array::from(vec![0u8, 255])));
    }
}

#[test]
fn test_mysql_float() {
    // FLOAT -> Float32. Compare with f32 literals so both sides are parsed at
    // the same single precision.
    for binary in [true, false] {
        let result = run_mysql_query(
            "SELECT test_float FROM test_types WHERE test_float IS NOT NULL ORDER BY test_float",
            binary,
        );
        assert_eq!(result.len(), 1);
        assert!(result[0]
            .column(0)
            .as_any()
            .downcast_ref::<Float32Array>()
            .unwrap()
            .eq(&Float32Array::from(vec![-1.1E-38f32, 3.4E38f32])));
    }
}

#[test]
fn test_mysql_short() {
    for binary in [true, false] {
        let result = run_mysql_query(
            "SELECT test_short FROM test_types WHERE test_short IS NOT NULL ORDER BY test_short",
            binary,
        );
        assert_eq!(result.len(), 1);
        assert!(result[0]
            .column(0)
            .as_any()
            .downcast_ref::<Int16Array>()
            .unwrap()
            .eq(&Int16Array::from(vec![-32768i16, 32767])));
    }
}

#[test]
fn test_mysql_int24() {
    // MEDIUMINT has no i24 representation, so it is mapped to Int32.
    for binary in [true, false] {
        let result = run_mysql_query(
            "SELECT test_int24 FROM test_types WHERE test_int24 IS NOT NULL ORDER BY test_int24",
            binary,
        );
        assert_eq!(result.len(), 1);
        assert!(result[0]
            .column(0)
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap()
            .eq(&Int32Array::from(vec![-8388608i32, 8388607])));
    }
}

#[test]
fn test_mysql_long() {
    for binary in [true, false] {
        let result = run_mysql_query(
            "SELECT test_long FROM test_types WHERE test_long IS NOT NULL ORDER BY test_long",
            binary,
        );
        assert_eq!(result.len(), 1);
        assert!(result[0]
            .column(0)
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap()
            .eq(&Int32Array::from(vec![-2147483648i32, 2147483647])));
    }
}

#[test]
fn test_mysql_longlong() {
    for binary in [true, false] {
        let result = run_mysql_query(
            "SELECT test_longlong FROM test_types \
             WHERE test_longlong IS NOT NULL ORDER BY test_longlong",
            binary,
        );
        assert_eq!(result.len(), 1);
        assert!(result[0]
            .column(0)
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap()
            .eq(&Int64Array::from(vec![
                -9223372036854775808i64,
                9223372036854775807,
            ])));
    }
}

#[test]
fn test_mysql_short_unsigned() {
    for binary in [true, false] {
        let result = run_mysql_query(
            "SELECT test_short_unsigned FROM test_types \
             WHERE test_short_unsigned IS NOT NULL ORDER BY test_short_unsigned",
            binary,
        );
        assert_eq!(result.len(), 1);
        assert!(result[0]
            .column(0)
            .as_any()
            .downcast_ref::<UInt16Array>()
            .unwrap()
            .eq(&UInt16Array::from(vec![0u16, 65535])));
    }
}

#[test]
fn test_mysql_int24_unsigned() {
    for binary in [true, false] {
        let result = run_mysql_query(
            "SELECT test_int24_unsigned FROM test_types \
             WHERE test_int24_unsigned IS NOT NULL ORDER BY test_int24_unsigned",
            binary,
        );
        assert_eq!(result.len(), 1);
        assert!(result[0]
            .column(0)
            .as_any()
            .downcast_ref::<UInt32Array>()
            .unwrap()
            .eq(&UInt32Array::from(vec![0u32, 16777215])));
    }
}

#[test]
fn test_mysql_long_unsigned() {
    for binary in [true, false] {
        let result = run_mysql_query(
            "SELECT test_long_unsigned FROM test_types \
             WHERE test_long_unsigned IS NOT NULL ORDER BY test_long_unsigned",
            binary,
        );
        assert_eq!(result.len(), 1);
        assert!(result[0]
            .column(0)
            .as_any()
            .downcast_ref::<UInt32Array>()
            .unwrap()
            .eq(&UInt32Array::from(vec![0u32, 4294967295])));
    }
}
