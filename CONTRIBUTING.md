# Developer's Guide

This doc describes how you can get started at developing ConnectorX.

## Environment Setup

### Install tools and dependencies

Please check out [here](https://sfu-db.github.io/connector-x/install.html#build-from-source-code)


### Run tests

* Set up environment variables by creating a `.env` file under project directory. Here is an example:
```
# postgres
POSTGRES_URL=postgresql://username:password@hostname:5432/db

# mysql
MYSQL_HOST=hostname
MYSQL_PORT=3306
MYSQL_DB=db
MYSQL_USER=username
MYSQL_PASSWORD=password
MYSQL_URL=mysql://${MYSQL_USER}:${MYSQL_PASSWORD}@${MYSQL_HOST}:${MYSQL_PORT}/${MYSQL_DB}

# sqlite
SQLITE_URL=sqlite://db_dir

# mssql
MSSQL_HOST=hostname
MSSQL_PORT=1433
MSSQL_USER=username
MSSQL_PASSWORD=password
MSSQL_DB=db
MSSQL_URL=mssql://username:password@hostname:1433/db

# log
RUST_LOG=connectorx=debug,connectorx_python=debug

# benchmark related
TPCH_TABLE=lineitem
MODIN_ENGINE=dask

```

* Seed database: `just seed-db`
* Run Rust tests: `just test`
* Run Python tests: `just test-python [-k {test case keyword}]`

### Other commands

* Format the code: `cargo fmt`

## How to Add a New Source

* Implement source related logics, including:
  * Define the type system of the new source
  * Implement data fetching and parsing logics
  * Examples can be found [here](https://github.com/sfu-db/connector-x/blob/main/connectorx/src/sources)
* Define the conversion between the new source and existing destinations
  * Examples can be found [here](https://github.com/sfu-db/connector-x/tree/main/connectorx/src/transports) and [here](https://github.com/sfu-db/connector-x/tree/main/connectorx-python/src/pandas/transports)
* Make the new source visable to destinations, including:
  * Add the source to the [source_router](https://github.com/sfu-db/connector-x/blob/main/connectorx-python/src/source_router.rs)
  * Add the source to writing functions of each destination. Here are examples for [pandas](https://github.com/sfu-db/connector-x/blob/main/connectorx-python/src/pandas/mod.rs) and [arrow](https://github.com/sfu-db/connector-x/blob/main/connectorx-python/src/arrow.rs)
* Add corresponding unit tests under `connectorx/tests` for Rust and `connectorx-python/connectorx/tests` for Python

**Please check out [here](https://sfu-db.github.io/connector-x/connectorx/#extending-connectorx) for more detailed implementation instructions of how to extend ConnectorX.**

## How to Add a New Destination

* Implement destination related logics, including:
  * Define the type system of the new destination
  * Implement data writing logics
  * Implement the writing interface of destination
  * Here are examples for [arrow](https://github.com/sfu-db/connector-x/tree/main/connectorx/src/destinations/arrow) and [pandas](https://github.com/sfu-db/connector-x/tree/main/connectorx-python/src/pandas)
* Define the conversion between existing source and the new destination
  * Examples can be found [here](https://github.com/sfu-db/connector-x/tree/main/connectorx/src/transports) and [here](https://github.com/sfu-db/connector-x/tree/main/connectorx-python/src/pandas/transports)
* Add corresponding unit tests under `connectorx/tests` for Rust and `connectorx-python/connectorx/tests` for Python

**Please check out [here](https://sfu-db.github.io/connector-x/connectorx/#extending-connectorx) for more detailed implementation instructions of how to extend ConnectorX.**
