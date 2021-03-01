use anyhow::Error;
use arrow::datatypes::{Schema, SchemaRef};
use arrow::ffi::{FFI_ArrowArray, FFI_ArrowSchema};
use arrow::json::reader::ReaderBuilder;
use arrow::record_batch::RecordBatch;
use fehler::throws;
use flate2::read::GzDecoder;
use futures::stream::{FuturesOrdered, StreamExt};
use futures::TryFutureExt;
use rusoto_core::Region;
use rusoto_s3::{GetObjectOutput, GetObjectRequest, S3Client, S3};
use serde_json::{from_str, Value};
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::sync::Arc;
use strum::EnumString;
use tokio::io::AsyncReadExt;
use tokio::task::spawn_blocking;

#[derive(Debug, Clone, Copy, EnumString)]
pub enum JsonFormat {
    JsonL,
    Array,
}

#[throws(Error)]
pub async fn read_s3<S>(
    bucket: &str,
    objects: &[S],
    schema: &str,
    json_format: JsonFormat,
) -> HashMap<String, Vec<(*const FFI_ArrowArray, *const FFI_ArrowSchema)>>
where
    S: AsRef<str>,
{
    let client = S3Client::new(Region::UsWest2);

    let schema = Arc::new(Schema::from(&from_str::<Value>(schema)?)?);
    let mut futs: FuturesOrdered<_> = objects
        .iter()
        .map(|obj| {
            client
                .get_object(GetObjectRequest {
                    bucket: bucket.into(),
                    key: obj.as_ref().to_string(),
                    ..Default::default()
                })
                .err_into()
                .and_then(|resp| read_as_record_batch(resp, schema.clone(), json_format))
        })
        .collect();

    let mut table = HashMap::new();

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
                        .or_insert_with(|| vec![])
                        .push(batch.column(i).to_raw()?)
                }
            }
        }
    }

    table
}

#[throws(Error)]
async fn read_as_record_batch(
    payload: GetObjectOutput,
    schema: SchemaRef,
    json_format: JsonFormat,
) -> Option<Vec<RecordBatch>> {
    if let None = payload.body.as_ref() {
        return None;
    }

    let mut buf = vec![];
    payload
        .body
        .unwrap()
        .into_async_read()
        .read_to_end(&mut buf)
        .await?;

    let batches = spawn_blocking(move || -> Result<_, Error> {
        let mut rawjson = vec![];
        GzDecoder::new(&*buf).read_to_end(&mut rawjson)?;

        let mut reader = match json_format {
            JsonFormat::Array => {
                array_to_jsonl(rawjson.as_mut());
                ReaderBuilder::new()
                    .with_schema(schema.clone())
                    .build(Cursor::new(&rawjson[1..rawjson.len() - 1]))?
            }
            JsonFormat::JsonL => ReaderBuilder::new()
                .with_schema(schema.clone())
                .build(Cursor::new(&rawjson[..]))?,
        };

        let mut batches = vec![];
        while let Some(rb) = reader.next()? {
            batches.push(rb);
        }
        Ok(batches)
    })
    .await??;

    Some(batches)
}

fn array_to_jsonl(data: &mut [u8]) {
    let mut indent = 0;
    let n = data.len();
    for i in 0..n {
        if data[i] == b',' && indent == 0 {
            data[i] = b'\n';
        } else if data[i] == b'{' {
            indent += 1;
        } else if data[i] == b'}' {
            indent -= 1;
        } else if i < n - 6 && &data[i..i + 6] == b"[\"new\"" {
            data[i..i + 6].copy_from_slice(b"[10001");
        } else if i < n - 9 && &data[i..i + 9] == b"[\"change\"" {
            data[i..i + 9].copy_from_slice(b"[10000002");
        } else if i < n - 9 && &data[i..i + 9] == b"[\"delete\"" {
            data[i..i + 9].copy_from_slice(b"[10000004");
        }
    }
}
