build-release:
    cargo build --release

build-debug:
    cargo build

test:
    cargo test -- --nocapture

bootstrap-python:
    cd connector-agent-python && poetry install

build-python-extention:
    cd connector-agent-python && cargo build --release

setup-python: build-python-extention
    cd connector-agent-python && poetry run python ../scripts/copy-extension.py
    
test-python: setup-python
    cd connector-agent-python && poetry run pytest connector_agent/tests -v -s

seed-db:
    psql $POSTGRES_URL -c "DROP TABLE IF EXISTS test_table;"
    psql $POSTGRES_URL -c "DROP TABLE IF EXISTS test_str;"
    psql $POSTGRES_URL -f scripts/postgres.sql

# benches 
flame-tpch:
    cd connector-agent-python && LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.8.6/lib/ cargo run --no-default-features --features executable --features fptr --release --example flame_tpch

build-tpch:
    cd connector-agent-python && cargo build --no-default-features --features executable --features fptr --release --example tpch

cachegrind-tpch: build-tpch
    valgrind --tool=cachegrind target/release/examples/tpch

python-tpch name +ARGS="": setup-python
    cd connector-agent-python && \
    poetry run python ../benchmarks/tpch-{{name}}.py {{ARGS}}

python-shell:
    cd connector-agent-python && \
    poetry run ipython


# releases
ci-build-python-extention:
    cd connector-agent-python && cargo build --release
    ls target/release
    cd connector-agent-python && poetry run python ../scripts/python-helper.py copy-extension

ci-build-python-wheel:
    cd connector-agent-python && poetry build
    
ci-rename-wheel:
    cd connector-agent-python && poetry run python ../scripts/python-helper.py rename-wheel