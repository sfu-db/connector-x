# Enabling Query Federation

We use [accio](https://github.com/sfu-db/accio) to rewrite a federated query into multiple single-source ones, and combine the result locally with [datafusion](https://github.com/apache/datafusion).

To enable query federation for connectorx:
1. Clone accio: `git@github.com:sfu-db/accio.git`.
2. Build accio: `cd accio/rewriter && mvn package -Dmaven.test.skip=true`.
3. Move the jar file to location `${YOUR_LOCAL_PYTHON_PATH}/site-packages/connectorx/dependencies/federated-rewriter.jar`
4. Configure accio and set the configuration path as `FED_CONFIG_PATH`. Example configurations can be found [here](https://github.com/sfu-db/accio/tree/main/benchmark/config/tpch10_datafusion/10gbit).
5. Run federated query using connectorx!

Alternatively, accio provides wrappers that can directly run federated queries on various query engines. In particular, it uses connectorx as the data fetching method when datafusion or polars are the federation engine. For more details, check out [here](https://github.com/sfu-db/accio).
