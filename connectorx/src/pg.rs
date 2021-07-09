use anyhow::Error;
use arrow::csv::reader::ReaderBuilder;
use arrow::datatypes::{Schema, SchemaRef};
use arrow::ffi::{FFI_ArrowArray, FFI_ArrowSchema};
use arrow::record_batch::RecordBatch;
use fehler::throws;
use futures::stream::{FuturesOrdered, StreamExt};
use log::debug;
use postgres::{Client, NoTls};
use serde_json::{from_str, Value};
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::sync::Arc;
use std::time::Instant;
use tokio::task::spawn_blocking;

#[throws(Error)]
pub async fn read_pg<S>(
    conn: &str,
    sqls: &[S],
    schema: &str,
) -> HashMap<String, Vec<(*const FFI_ArrowArray, *const FFI_ArrowSchema)>>
where
    S: AsRef<str>,
{
    let schema = Arc::new(Schema::from(&from_str::<Value>(schema)?)?);
    let mut futs: FuturesOrdered<_> = sqls
        .iter()
        .map(|sql| read_sql_as_batch(conn, sql, schema.clone()))
        .collect();
    let mut table = HashMap::new();
    debug!("start queries");
    let start = Instant::now();
    while let Some(rb) = futs.next().await {
        if let Some(batches) = rb? {
            for batch in batches {
                for (i, f) in batch.schema().fields().iter().enumerate() {
                    use arrow::datatypes::DataType::*;
                    match f.data_type() {
                        Null | Boolean | Int8 | Int16 | Int32 | Int64 | UInt8 | UInt16 | UInt32
                        | UInt64 | Float16 | Float32 | Float64 | Utf8 | LargeUtf8 | Binary
                        | LargeBinary => {}
                        _ => continue,
                    }
                    table
                        .entry(f.name().clone())
                        .or_insert_with(Vec::new)
                        .push(batch.column(i).to_raw()?)
                }
            }
        }
    }
    debug!("finish to arrow table: {:?}", start.elapsed());
    table
}

#[throws(Error)]
pub async fn read_sql_as_batch<S>(
    conn: &str,
    sql: &S,
    schema: SchemaRef,
) -> Option<Vec<RecordBatch>>
where
    S: AsRef<str>,
{
    let query = format!("COPY ({}) TO STDOUT WITH CSV", sql.as_ref());
    let conn = conn.to_string();
    let batches = spawn_blocking(move || -> Result<_, Error> {
        let start = Instant::now();
        let mut buf = vec![];
        let mut client = Client::connect(&conn, NoTls)?;
        client.copy_out(&*query)?.read_to_end(&mut buf)?;
        let t_copy = start.elapsed();

        let mut batches = vec![];
        let reader = ReaderBuilder::new()
            .with_schema(schema)
            .with_delimiter(b',')
            .build(Cursor::new(&buf[..]))?;

        for rb in reader {
            batches.push(rb?);
        }
        let t_arrow = start.elapsed();
        debug!("copy: {:?} batch: {:?}", t_copy, t_arrow);
        Ok(batches)
    })
    .await??;

    Some(batches)
}
