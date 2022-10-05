set dotenv-load := true

build-lib:
    cargo build --features src_postgres --features dst_arrow

build-lib-release:
    cargo build --release --features src_postgres --features dst_arrow

build-release:
    cargo build  --release

build-debug:
    cargo build

test +ARGS="": 
    cargo test --features all {{ARGS}} -- --nocapture

test-feature-gate:
    cargo c --features src_postgres
    cargo c --features src_mysql
    cargo c --features src_mssql
    cargo c --features src_sqlite
    cargo c --features src_oracle
    cargo c --features src_csv
    cargo c --features src_dummy
    cargo c --features dst_arrow
    cargo c --features dst_arrow2

bootstrap-python:
    cd connectorx-python && poetry install

setup-java:
    cd federated-query/rewriter && mvn package -Dmaven.test.skip=true
    cp -f ./federated-query/rewriter/target/federated-rewriter-1.0-SNAPSHOT-jar-with-dependencies.jar connectorx-python/connectorx/dependencies/federated-rewriter.jar

setup-python:
    cd connectorx-python && poetry run maturin develop --release
    
test-python +opts="": setup-python
    cd connectorx-python && poetry run pytest connectorx/tests -v -s {{opts}}

test-python-s +opts="":
    cd connectorx-python && poetry run pytest connectorx/tests -v -s {{opts}}

test-fed file="3.sql":
    cd connectorx && cargo run --features src_postgres --features src_mysql --features dst_arrow --features federation --example federated_test "../federated-query/test-queries/{{file}}"

test-datafusion:
    cd connectorx && cargo run --features src_postgres --features src_mysql --features dst_arrow --features federation --example test

seed-db:
    #!/bin/bash
    psql $POSTGRES_URL -f scripts/postgres.sql
    sqlite3 ${SQLITE_URL#sqlite://} < scripts/sqlite.sql
    mysql --protocol tcp -h$MYSQL_HOST -P$MYSQL_PORT -u$MYSQL_USER -p$MYSQL_PASSWORD $MYSQL_DB < scripts/mysql.sql
    mssql-cli -S$MSSQL_HOST -U$MSSQL_USER -P$MSSQL_PASSWORD -d$MSSQL_DB -i scripts/mssql.sql

# dbs not included in ci
seed-db-more:
    mysql --protocol tcp -h$CLICKHOUSE_HOST -P$CLICKHOUSE_PORT -u$CLICKHOUSE_USER -p$CLICKHOUSE_PASSWORD $CLICKHOUSE_DB < scripts/clickhouse.sql
    psql $REDSHIFT_URL -f scripts/redshift.sql
    ORACLE_URL_SCRIPT=`echo ${ORACLE_URL#oracle://} | sed "s/:/\//"`
    cat scripts/oracle.sql | sqlplus $ORACLE_URL_SCRIPT
    mysql --protocol tcp -h$MARIADB_HOST -P$MARIADB_PORT -u$MARIADB_USER -p$MARIADB_PASSWORD $MARIADB_DB < scripts/mysql.sql

# benches 
flame-tpch conn="POSTGRES_URL":
    cd connectorx-python && PYO3_PYTHON=$HOME/.pyenv/versions/3.8.6/bin/python3.8 PYTHONPATH=$HOME/.pyenv/versions/conn/lib/python3.8/site-packages LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.8.6/lib/ cargo run --no-default-features --features executable --features fptr --features nbstr --features dsts --features srcs --release --example flame_tpch {{conn}}

build-tpch:
    cd connectorx-python && cargo build --no-default-features --features executable --features fptr --release --example tpch

cachegrind-tpch: build-tpch
    valgrind --tool=cachegrind target/release/examples/tpch

python-tpch name +ARGS="": setup-python
    #!/bin/bash
    export PYTHONPATH=$PWD/connectorx-python
    cd connectorx-python && \
    poetry run python ../benchmarks/tpch-{{name}}.py {{ARGS}}

python-tpch-ext name +ARGS="":
    cd connectorx-python && poetry run python ../benchmarks/tpch-{{name}}.py {{ARGS}}

python-ddos name +ARGS="": setup-python
    #!/bin/bash
    export PYTHONPATH=$PWD/connectorx-python
    cd connectorx-python && \
    poetry run python ../benchmarks/ddos-{{name}}.py {{ARGS}}

python-ddos-ext name +ARGS="":
    cd connectorx-python && poetry run python ../benchmarks/ddos-{{name}}.py {{ARGS}}


python-shell:
    cd connectorx-python && \
    poetry run ipython

benchmark-report: setup-python
    cd connectorx-python && \
    poetry run pytest connectorx/tests/benchmarks.py --benchmark-json ../benchmark.json
    
# releases
build-python-wheel:
    # need to get the j4rs dependency first
    cd connectorx-python && maturin build --release -i python --no-sdist
    # copy files
    cp -rf connectorx-python/target/release/jassets connectorx-python/connectorx/dependencies
    # build final wheel
    cd connectorx-python && maturin build --release -i python --no-sdist

bench-fed path:
    just python-tpch fed --file {{path}}/q2.sql
    just python-tpch-ext fed --file {{path}}/q3.sql
    just python-tpch-ext fed --file {{path}}/q4.sql
    just python-tpch-ext fed --file {{path}}/q5.sql
    just python-tpch-ext fed --file {{path}}/q7.sql
    just python-tpch-ext fed --file {{path}}/q8.sql
    just python-tpch-ext fed --file {{path}}/q9.sql
    just python-tpch-ext fed --file {{path}}/q10.sql
    just python-tpch-ext fed --file {{path}}/q11.sql
    just python-tpch-ext fed --file {{path}}/q12.sql
    just python-tpch-ext fed --file {{path}}/q13.sql
    just python-tpch-ext fed --file {{path}}/q14.sql
    just python-tpch-ext fed --file {{path}}/q16.sql
    just python-tpch-ext fed --file {{path}}/q17.sql
    just python-tpch-ext fed --file {{path}}/q18.sql
    just python-tpch-ext fed --file {{path}}/q19.sql
    just python-tpch-ext fed --file {{path}}/q20.sql
    just python-tpch-ext fed --file {{path}}/q22.sql
