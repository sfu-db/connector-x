# connector-agent ![CI](https://github.com/dovahcrow/treerite/workflows/CI/badge.svg)

## Environment Setup

* Install rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
* Install Just: `cargo install just`.
* Install nightly rust: `rustup toolchain install nightly-2021-01-01`.
* Set nightly rust as the project compiler, run this in the project root: `rustup override set nightly-2021-01-01`.

## Commands (refer to Justfile)
* Build rust: `just build-rust`
* Run test on PostgreSQL: `just pg_*`
  * `just pg_pandas`: pandas `read_sql` baseline
  * `just pg_copy`: use COPY command to export data from Postgres
    * `just pg_multi_copy [num]`: split query into [num] partitions
  * `just pg_pyarrow`: modify on pg_copy, convert to arrow format first and then convert to dataframe
    * `just pg_multi_pyarrow [num]`: split query into [num] partitions
  * `just pg_rust [num]`: Rust implementation and split query into [num] partitions
  
In this experiment we use TPC-H table *lineitem* with scale 10 (named `lineitem_s10`), you can also replace it with your own query.
