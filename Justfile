build-release:
    cargo build  --release

build-debug:
    cargo build

test:
    cargo test --features src_postgres --features src_sqlite --features src_mysql --features src_csv --features dst_arrow --features dst_memory --features dst_polars -- --nocapture

test-all:
    cargo test --features all

bootstrap-python:
    cp README.md connectorx-python/README.md
    cd connectorx-python && poetry install

build-python-extention:
    cd connectorx-python && cargo build --release

setup-python: build-python-extention
    cd connectorx-python && poetry run python ../scripts/python-helper.py copy-extension
    
test-python +opts="": setup-python
    cd connectorx-python && poetry run pytest connectorx/tests -v -s {{opts}}

seed-db:
    psql $POSTGRES_URL -f scripts/postgres.sql
    sqlite3 ${SQLITE_URL#sqlite://} < scripts/sqlite.sql
    mysql --protocol tcp -h$MYSQL_HOST -P$MYSQL_PORT -u$MYSQL_USER -p$MYSQL_PASSWORD $MYSQL_DB < scripts/mysql.sql

seed-db-more:
    mysql --protocol tcp -h$CLICKHOUSE_HOST -P$CLICKHOUSE_PORT -u$CLICKHOUSE_USER -p$CLICKHOUSE_PASSWORD $CLICKHOUSE_DB < scripts/clickhouse.sql
    psql $REDSHIFT_URL -f scripts/redshift.sql

# benches 
flame-tpch conn="POSTGRES_URL":
    cd connectorx-python && PYO3_PYTHON=$HOME/.pyenv/versions/3.8.6/bin/python3.8 PYTHONPATH=$HOME/.pyenv/versions/conn/lib/python3.8/site-packages LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.8.6/lib/ cargo run --no-default-features --features executable --features fptr --release --example flame_tpch {{conn}}

build-tpch:
    cd connectorx-python && cargo build --no-default-features --features executable --features fptr --release --example tpch

cachegrind-tpch: build-tpch
    valgrind --tool=cachegrind target/release/examples/tpch

python-tpch name +ARGS="": setup-python
    #!/bin/bash
    export PYTHONPATH=$PWD/connectorx-python
    cd connectorx-python && \
    poetry run python ../benchmarks/tpch-{{name}}.py {{ARGS}}

python-shell:
    cd connectorx-python && \
    poetry run ipython

benchmark-report: setup-python
    cd connectorx-python && \
    poetry run pytest connectorx/tests/benchmarks.py --benchmark-json ../benchmark.json
    
# releases
ci-build-python-extention:
    cd connectorx-python && cargo build --release
    ls target/release
    cd connectorx-python && poetry run python ../scripts/python-helper.py copy-extension

ci-build-python-wheel:
    cp README.md connectorx-python/README.md
    cd connectorx-python && poetry build
    
ci-rename-wheel:
    cd connectorx-python && poetry run python ../scripts/python-helper.py rename-wheel
