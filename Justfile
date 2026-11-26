set dotenv-load := true

# Build the connectorx crate in release mode with all features
build-release:
    cargo build --release --features all

# Build the connectorx crate in debug mode with all features
build-debug:
    cargo build --features all

# Build the C++ bindings (connectorx-cpp) in debug mode
build-cpp +ARGS="":
    cd connectorx-cpp && cargo build {{ARGS}}

# Build the C++ bindings (connectorx-cpp) in release mode
build-cpp-release +ARGS="":
    cd connectorx-cpp && cargo build --release {{ARGS}}

# Run all Rust tests with all features (needs the databases from .env)
test +ARGS="":
    cargo test --features all {{ARGS}} -- --nocapture

# Run the Rust unit and per-source integration tests as CI does (needs Docker)
test-ci:
    cargo test --lib --features all
    cargo test --lib --features src_mssql_tiberius,src_mssql_tds -- sources::mssql
    cargo test --features src_postgres --features dst_arrow --test test_postgres
    cargo test --features src_postgres --features src_dummy --features dst_polars --test test_polars
    cargo test --features src_mssql_tiberius --features dst_arrow --test test_mssql
    cargo test --features src_mssql_tiberius --features dst_arrow --test test_mssql_types
    cargo test --features src_mssql_tds --features dst_arrow --test test_mssql
    cargo test --features src_mssql_tds --features dst_arrow --test test_mssql_types
    cargo test --features src_mssql_tiberius,src_mssql_tds,dst_arrow --test test_mssql_runtime_switch
    cargo test --features src_postgres --features src_dummy --features dst_arrow --test test_arrow
    cargo test --features src_dummy --features dst_arrow --test test_arrow_stream
    cargo test --features src_csv --features dst_arrow --test test_csv
    cargo test --features src_mysql --features dst_arrow --test test_mysql
    cargo test --features src_clickhouse --features dst_arrow --test test_clickhouse
    cargo test --features src_trino --features dst_arrow --test test_trino

# Coverage is split into a unit and an integration report so each shows up
# separately in Codecov. Both stay under cargo-llvm-cov's instrumented build
# the whole time (only --profraw-only is cleaned between runs, never the
# compiled artifacts), so this is still a single compile overall.
# Rust unit-test coverage to lcov-unit.info (CI; no databases needed)
coverage-unit:
    cargo llvm-cov clean --workspace --profraw-only
    cargo llvm-cov --no-report --lib --features all
    cargo llvm-cov --no-report --lib --features src_mssql_tiberius,src_mssql_tds -- sources::mssql
    cargo llvm-cov --no-report --lib --features src_mssql_tiberius -- sources::mssql
    cargo llvm-cov report --lcov --output-path lcov-unit.info

# Removes any testcontainers-managed container/image left behind by the
# just-finished test invocation, so disk usage stays at "one DB's footprint"
# instead of stacking across postgres/polars/mssql within the same job.
# Remove containers and images left behind by testcontainers
_reap-test-containers:
    docker rm -f $(docker ps -aq --filter "label=org.testcontainers.managed-by=testcontainers") 2>/dev/null || true
    docker image prune -af

# Rust integration-test coverage to lcov-integration.info (CI; needs Docker)
coverage-integration:
    cargo llvm-cov clean --workspace --profraw-only
    cargo llvm-cov --no-report --features src_postgres --features dst_arrow --test test_postgres
    just _reap-test-containers
    cargo llvm-cov --no-report --features src_postgres --features src_dummy --features dst_polars --test test_polars
    just _reap-test-containers
    cargo llvm-cov --no-report --features src_mssql_tiberius --features dst_arrow --test test_mssql
    just _reap-test-containers
    cargo llvm-cov --no-report --features src_mssql_tiberius --features dst_arrow --test test_mssql_types
    just _reap-test-containers
    cargo llvm-cov --no-report --features src_mssql_tds --features dst_arrow --test test_mssql
    just _reap-test-containers
    cargo llvm-cov --no-report --features src_mssql_tds --features dst_arrow --test test_mssql_types
    just _reap-test-containers
    cargo llvm-cov --no-report --features src_mssql_tiberius,src_mssql_tds,dst_arrow --test test_mssql_runtime_switch
    just _reap-test-containers
    cargo llvm-cov --no-report --features src_postgres --features src_dummy --features dst_arrow --test test_arrow
    just _reap-test-containers
    cargo llvm-cov --no-report --features src_dummy --features dst_arrow --test test_arrow_stream
    cargo llvm-cov --no-report --features src_csv --features dst_arrow --test test_csv
    cargo llvm-cov --no-report --features src_mysql --features dst_arrow --test test_mysql
    just _reap-test-containers
    cargo llvm-cov --no-report --features src_clickhouse --features dst_arrow --test test_clickhouse
    just _reap-test-containers
    cargo llvm-cov --no-report --features src_trino --features dst_arrow --test test_trino
    just _reap-test-containers
    cargo llvm-cov report --lcov --output-path lcov-integration.info

# Check that each source/destination feature compiles on its own without warnings (CI)
test-feature-gate:
    cargo clippy --features src_postgres -- -D warnings
    cargo clippy --features src_mysql -- -D warnings
    cargo clippy --features src_mssql_tiberius -- -D warnings
    cargo clippy --features src_mssql_tds -- -D warnings
    cargo clippy --features src_sqlite -- -D warnings
    cargo clippy --features src_oracle -- -D warnings
    cargo clippy --features src_trino -- -D warnings
    cargo clippy --features src_clickhouse -- -D warnings
    cargo clippy --features dst_arrow -- -D warnings
    cargo c --features all --test test_mssql

# Remove Rust build artifacts and the built Python extension
cleanup:
    cargo clean
    cd connectorx-python && cargo clean
    rm connectorx-python/connectorx/connectorx*.so

# Install the Python dev dependencies with uv
bootstrap-python:
    cd connectorx-python && uv sync

# Build the federated-query rewriter jar (needs ACCIO_PATH and Maven)
setup-java:
    cd $ACCIO_PATH/rewriter && mvn package -Dmaven.test.skip=true
    cp -f $ACCIO_PATH/rewriter/target/accio-rewriter-1.0-SNAPSHOT-jar-with-dependencies.jar connectorx-python/connectorx/dependencies/federated-rewriter.jar

# Build the Python extension in release mode into the uv environment
setup-python:
    cd connectorx-python && uv run maturin develop --uv --release --locked

# Build the Python extension, then run the Python tests (pytest options allowed)
test-python +opts="": setup-python
    cd connectorx-python && uv run pytest connectorx/tests -v -s {{opts}}

# Run the Python tests without rebuilding the extension
test-python-s +opts="":
    cd connectorx-python && uv run pytest connectorx/tests -v -s {{opts}}

# Load test data into the Postgres, SQLite and MySQL databases from .env
seed-db:
    #!/bin/bash
    psql $POSTGRES_URL -f scripts/postgres.sql
    sqlite3 ${SQLITE_URL#sqlite://} < scripts/sqlite.sql
    mysql --protocol tcp -h$MYSQL_HOST -P$MYSQL_PORT -u$MYSQL_USER -p$MYSQL_PASSWORD $MYSQL_DB < scripts/mysql.sql

# Load test data into the databases CI does not use (MSSQL, Redshift, Oracle, MariaDB, Trino, ClickHouse)
seed-db-more:
    mssql-cli -S$MSSQL_HOST -U$MSSQL_USER -P$MSSQL_PASSWORD -d$MSSQL_DB -i scripts/mssql.sql
    psql $REDSHIFT_URL -f scripts/redshift.sql
    ORACLE_URL_SCRIPT=`echo ${ORACLE_URL#oracle://} | sed "s/:/\//"`
    cat scripts/oracle.sql | sqlplus $ORACLE_URL_SCRIPT
    mysql --protocol tcp -h$MARIADB_HOST -P$MARIADB_PORT -u$MARIADB_USER -p$MARIADB_PASSWORD $MARIADB_DB < scripts/mysql.sql
    trino $TRINO_URL --catalog=$TRINO_CATALOG < scripts/trino.sql
    clickhouse-client -h $CLICKHOUSE_HOST --port $CLICKHOUSE_PORT -u $CLICKHOUSE_USER --password $CLICKHOUSE_PASSWORD -d $CLICKHOUSE_DB < scripts/clickhouse.sql

# benches

# Build the Python extension, then run benchmarks/tpch-<name>.py
python-tpch name +ARGS="": setup-python
    #!/bin/bash
    export PYTHONPATH=$PWD/connectorx-python
    cd connectorx-python && \
    uv run python ../benchmarks/tpch-{{name}}.py {{ARGS}}

# Run benchmarks/tpch-<name>.py without rebuilding the extension
python-tpch-ext name +ARGS="":
    cd connectorx-python && uv run python ../benchmarks/tpch-{{name}}.py {{ARGS}}

# Build the Python extension, then run benchmarks/ddos-<name>.py
python-ddos name +ARGS="": setup-python
    #!/bin/bash
    export PYTHONPATH=$PWD/connectorx-python
    cd connectorx-python && \
    uv run python ../benchmarks/ddos-{{name}}.py {{ARGS}}

# Run benchmarks/ddos-<name>.py without rebuilding the extension
python-ddos-ext name +ARGS="":
    cd connectorx-python && uv run python ../benchmarks/ddos-{{name}}.py {{ARGS}}


# Open an IPython shell in the Python dev environment
python-shell:
    cd connectorx-python && \
    uv run ipython

# Build the Python extension and write benchmark results to benchmark.json
benchmark-report: setup-python
    cd connectorx-python && \
    uv run pytest connectorx/tests/benchmarks.py --benchmark-json ../benchmark.json

# releases

# Build a release Python wheel
build-python-wheel:
    cd connectorx-python && maturin build --release -i python

# Build a release Python wheel with federated queries enabled
build-python-wheel-fed:
    # need to get the j4rs dependency first
    cd connectorx-python && maturin build --release -i python
    # copy files
    cp -rf connectorx-python/target/release/jassets connectorx-python/connectorx/dependencies
    # build final wheel
    cd connectorx-python && maturin build --release -i python