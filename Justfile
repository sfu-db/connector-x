build-release:
    cargo build --release

build-debug:
    cargo build

test:
    cargo test -- --nocapture

bootstrap-python:
    cp README.md connectorx-python/README.md
    cd connectorx-python && poetry install

build-python-extention:
    cd connectorx-python && cargo build --release

setup-python: build-python-extention
    cd connectorx-python && poetry run python ../scripts/python-helper.py copy-extension
    
test-python: setup-python
    cd connectorx-python && poetry run pytest connectorx/tests -v -s

seed-db:
    psql $POSTGRES_URL -f scripts/postgres.sql

# benches 
flame-tpch:
    cd connectorx-python && LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.8.6/lib/ cargo run --no-default-features --features executable --features fptr --release --example flame_tpch

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