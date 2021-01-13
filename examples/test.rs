use arrow::datatypes::{DataType, Field, Schema};
use connector_agent::s3;
use failure::Fallible;
use serde_json::to_string;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Fallible<()> {
    let schema = Arc::new(Schema::new(vec![
        Field::new(
            "asks",
            DataType::List(Box::new(Field::new(
                "entry",
                DataType::Struct(vec![
                    Field::new("type", DataType::UInt64, false),
                    Field::new("price", DataType::Float64, false),
                    Field::new("qty", DataType::Float64, false),
                ]),
                false,
            ))),
            false,
        ),
        Field::new(
            "bids",
            DataType::List(Box::new(Field::new(
                "entry",
                DataType::Struct(vec![
                    Field::new("type", DataType::UInt64, false),
                    Field::new("price", DataType::Float64, false),
                    Field::new("qty", DataType::Float64, false),
                ]),
                false,
            ))),
            false,
        ),
        Field::new("change_id", DataType::UInt64, false),
        Field::new("instrument_name", DataType::Utf8, false),
        Field::new("timestamp", DataType::UInt64, false),
        Field::new("type", DataType::Utf8, false),
        Field::new("prev_change_id", DataType::UInt64, false),
    ]));

    s3::read_s3(
        "dovahcrow",
        &[
            "raw/book.BTC-PERPETUAL.raw/[2020-11-30 21:45:06.698628639 UTC]~[2020-11-30 23:55:00.711449067 UTC].first.json.gz",
            "raw/book.BTC-PERPETUAL.raw/[2020-11-30 23:55:00.711458648 UTC]~[2020-12-01 01:20:00.753265335 UTC].json.gz",
            "raw/book.BTC-PERPETUAL.raw/[2020-12-01 01:20:00.753274983 UTC]~[2020-12-01 03:15:00.733890875 UTC].json.gz",
            "raw/book.BTC-PERPETUAL.raw/[2020-12-01 03:15:00.733900406 UTC]~[2020-12-01 05:12:00.723458713 UTC].json.gz",
            "raw/book.BTC-PERPETUAL.raw/[2020-12-01 05:12:00.723468470 UTC]~[2020-12-01 07:37:00.735621952 UTC].json.gz",
            "raw/book.BTC-PERPETUAL.raw/[2020-12-01 07:37:00.735632789 UTC]~[2020-12-01 09:54:00.722288899 UTC].json.gz",
            "raw/book.BTC-PERPETUAL.raw/[2020-12-01 09:54:00.722299282 UTC]~[2020-12-01 11:21:39.762742203 UTC].json.gz",
        ],
        &to_string(&schema.to_json())?,
        "JsonL".parse()?,
    )
    .await?;
    Ok(())
}
