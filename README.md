# connector-agent [![status][ci_badge]][ci_page] [![docs][docs_badge]][docs_page]

[ci_badge]: https://github.com/dovahcrow/treerite/workflows/ci/badge.svg
[ci_page]: https://github.com/sfu-db/connector-agent/actions

[docs_badge]: https://github.com/dovahcrow/treerite/workflows/docs/badge.svg
[docs_page]: https://sfu-db.github.io/connector-agent/connector_agent/
## Environment Setup
* Install rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* Install Just: `cargo install just`

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
