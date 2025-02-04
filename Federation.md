# Enabling Query Federation

We use [accio](https://github.com/sfu-db/accio) to rewrite a federated query into multiple single-source ones, and combine the result locally with [datafusion](https://github.com/apache/datafusion).


Currently, this feature is only enabled by building connectorx from source code as follows:

1. Clone connectorx: `git clone git@github.com:sfu-db/connector-x.git`.
2. Build connectorx from source follows the [instruction](https://sfu-db.github.io/connector-x/install.html#build-from-source-code). Note, for the final step, build wheel with `build-python-wheel-fed` command instead of `build-python-wheel`.
3. Install connectorx: `pip install ${YOUR_LOCAL_CONNECTORX_PATH}/connectorx-python/target/wheels/${YOUR_WHEEL_FILE}`.
4. Clone accio: `git clone git@github.com:sfu-db/accio.git`.
5. Build accio: `cd accio/rewriter && mvn package -Dmaven.test.skip=true`.
6. Move the jar file to location `${YOUR_LOCAL_PYTHON_PATH}/site-packages/connectorx/dependencies/federated-rewriter.jar`
7. Configure accio and set the configuration path as `FED_CONFIG_PATH`. Example configurations can be found [here](https://github.com/sfu-db/accio/tree/main/benchmark/config/tpch10_datafusion/10gbit).
8. Run federated query using connectorx!

Alternatively, accio provides wrappers that can directly run federated queries on various query engines. In particular, it uses connectorx as the data fetching method when datafusion or polars are the federation engine. For more details, check out [here](https://github.com/sfu-db/accio).
