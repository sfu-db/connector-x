use chrono::NaiveDate;
use connectorx::Result;
use postgres::{binary_copy::BinaryCopyOutIter, fallible_iterator::FallibleIterator, types::Type};
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use rust_decimal::Decimal;
use std::time::Instant;

fn main() -> Result<()> {
    let manager = PostgresConnectionManager::new(
        "postgresql://postgres:postgres@localhost:5432/tpch".parse()?,
        NoTls,
    );
    let pool = Pool::builder().max_size(5).build(manager)?;

    let mut conn = pool.get()?;
    let pg_schema: Vec<_> = vec![
        Type::INT4,
        Type::INT4,
        Type::INT4,
        Type::INT4,
        Type::NUMERIC,
        Type::NUMERIC,
        Type::NUMERIC,
        Type::NUMERIC,
        Type::TEXT,
        Type::TEXT,
        Type::DATE,
        Type::DATE,
        Type::DATE,
        Type::TEXT,
        Type::TEXT,
        Type::TEXT,
    ];

    println!("run copy_out");
    let reader =
        conn.copy_out("COPY (SELECT * FROM lineitem limit 100000) TO STDOUT WITH BINARY")?;
    let mut iter = BinaryCopyOutIter::new(reader, &pg_schema);

    let start_stmp = Instant::now();
    println!("start traverse!");
    while let Some(row) = iter.next()? {
        let _l_orderkey: i32 = row.try_get(0)?;
        let _l_partkey: i32 = row.try_get(1)?;
        let _l_suppkey: i32 = row.try_get(2)?;
        let _l_linenumber: i32 = row.try_get(3)?;
        let _l_quantity: Decimal = row.try_get(4)?;
        let _l_extendedprice: Decimal = row.try_get(5)?;
        let _l_discount: Decimal = row.try_get(6)?;
        let _l_tax: Decimal = row.try_get(7)?;
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
