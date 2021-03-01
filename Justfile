build-release:
    cargo build --release

build-debug:
    cargo build

test:
    cargo test -- --nocapture

bootstrap-python:
    cd connector-agent-python && poetry install
    
setup-python:
    cd connector-agent-python && poetry run maturin develop --release --strip
    
test-python: setup-python
    cd connector-agent-python && poetry run pytest connector_agent_python/tests -v

seed-db:
    psql $POSTGRES_URL -c "DROP TABLE IF EXISTS test_table;"
    psql $POSTGRES_URL -f scripts/postgres.sql

perf-tpch:
    cd connector-agent-python && LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.8.6/lib/ cargo run --no-default-features --features executable --release --example perf_tpch

flame-tpch-old:
    cd connector-agent-python && LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.8.6/lib/ cargo criterion --no-default-features --features executable --bench tpch_old -- --profile-time=300


python-tpch n="1": setup-python
    cd connector-agent-python && \
    poetry run python ../scripts/test_tpch.py {{n}}

python-tpch-rust-arrow n="1": setup-python
    cd connector-agent-python && \
    poetry run python ../scripts/tpch-rust-arrow.py {{n}}

python-tpch-pyarrow: 
    cd connector-agent-python && \
    poetry run python ../scripts/tpch-pyarrow.py


python-tpch-pyarrow-p n="1": 
    cd connector-agent-python && \
    poetry run python ../scripts/tpch-pyarrow-p.py {{n}}