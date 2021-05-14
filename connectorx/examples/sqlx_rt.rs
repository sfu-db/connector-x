use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use futures::TryStreamExt;
use sqlx::{postgres::PgPoolOptions, Row};
use std::time::Instant;
use tokio::runtime;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

async fn test_postgres() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/tpch")
        .await?;

    println!("run fetch");
    let mut rows = sqlx::query("SELECT * FROM lineitem limit 100000").fetch(&pool);

    let start_stmp = Instant::now();
    println!("start traverse!");
    while let Some(row) = rows.try_next().await? {
        // map the row into a user-defined domain type

        let _l_orderkey: i32 = row.try_get(0)?;
        let _l_partkey: i32 = row.try_get(1)?;
        let _l_suppkey: i32 = row.try_get(2)?;
        let _l_linenumber: i32 = row.try_get(3)?;
        let _l_quantity: BigDecimal = row.try_get(4)?;
        let _l_extendedprice: BigDecimal = row.try_get(5)?;
        let _l_discount: BigDecimal = row.try_get(6)?;
        let _l_tax: BigDecimal = row.try_get(7)?;
        let _l_returnflag: &str = row.try_get(8)?;
        let _l_linestatus: &str = row.try_get(9)?;
        let _l_shipdate: NaiveDate = row.try_get(10)?;
        let _l_commitdate: NaiveDate = row.try_get(11)?;
        let _l_receiptdate: NaiveDate = row.try_get(12)?;
        let _l_shipinstruct: &str = row.try_get(13)?;
        let _l_shipmode: &str = row.try_get(14)?;
        let _l_comment: &str = row.try_get(15)?;

        // println!(
        //     "{},{},{},{},{},{},{},{},{},{},{:?},{:?},{:?},{},{},{}",
        //     _l_orderkey,
        //     _l_partkey,
        //     _l_suppkey,
        //     _l_linenumber,
        //     _l_quantity,
        //     _l_extendedprice,
        //     _l_discount,
        //     _l_tax,
        //     _l_returnflag,
        //     _l_linestatus,
        //     _l_shipdate,
        //     _l_commitdate,
        //     _l_receiptdate,
        //     _l_shipinstruct,
        //     _l_shipmode,
        //     _l_comment,
        // );
    }

    println!("Traverse finished, took {:?}", start_stmp.elapsed());
    Ok(())
}

// #[async_std::main]
fn main() -> Result<(), sqlx::Error> {
    // build runtime
    let runtime = runtime::Runtime::new()?;

    runtime.block_on(test_postgres())?;

    Ok(())
}
