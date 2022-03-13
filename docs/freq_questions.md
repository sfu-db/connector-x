# Frequently asked questions

## How to specify the partition number?

`partition_num` will determine how many queries we are going to split from the original one and issue to the database. Underlying, we use [rayon](https://github.com/rayon-rs/rayon) as our parallel executor, which adopts a pool of threads to handle each partitioned query. The number of threads in the pool equals to the number of logical cores on the machine. It is recommended to set the `partition_num` to the number of available logical cores.

## How to choose the partition column?

`partition_on` specifies on which column we will do the partition as above procedure. In order to achieve the best performance, it is ideal that each partitioned query will return the same number of rows. And since we partition the column evenly, it is recommended that the numerical `partition_on` column is evenly distributed. Whether a column has index or not might also affect the performance depends on the source database. You can give it a try if you have multiple candidates. Also, you can manually partition the query if our partition method cannot match your need. ConnectorX will still return a whole dataframe with all the results of the list of queries you input.

## How to print log in Python?

Set the environment variable `RUST_LOG` to have a detailed look at Rust log.
```python
import os
os.environ["RUST_LOG"]="connectorx=debug,connectorx_python=debug"
import connectorx as cx

df = cx.read_sql(conn, query) // It will be more clear to test when no partitioning first
```

## How to build ConnectorX locally?

* Step 1: Fresh clone of source
```bash
git clone https://github.com/sfu-db/connector-x.git
```

* Step 2: Install nightly-2021-10-21
```bash
$ rustup install nightly-2021-10-21
info: syncing channel updates for 'nightly-2021-10-21-aarch64-apple-darwin'

  nightly-2021-10-21-aarch64-apple-darwin unchanged - rustc 1.58.0-nightly (efd048394 2021-10-20)

info: checking for self-updates
```

* Step 3: Override default project toolchain
```base
$ rustup default nightly-2021-10-21
$ rustup override set nightly-2021-10-21
$ rustup show
Default host: aarch64-apple-darwin
rustup home:  /Users/user/.rustup

installed toolchains
--------------------

stable-aarch64-apple-darwin
nightly-2021-10-21-aarch64-apple-darwin (default)
nightly-aarch64-apple-darwin

active toolchain
----------------

nightly-2021-10-21-aarch64-apple-darwin (default)
rustc 1.58.0-nightly (efd048394 2021-10-20)
```

* Step 4: Build
```bash
$ just bootstrap-python
$ just ci-build-python-extention ci-build-python-wheel ci-rename-wheel
```
