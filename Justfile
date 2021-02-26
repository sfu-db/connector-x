build-release:
    cargo build --release

build-debug:
    cargo build

test:
    cargo test

bootstrap-python:
    cd connector-agent-python && poetry install
    
setup-python:
    cd connector-agent-python && poetry run maturin develop --release --strip
    
test-python: setup-python
    cd connector-agent-python && poetry run pytest connector_agent_python/tests -v

seed-db:
    psql $POSTGRES_URL -c "DROP TABLE test_table;"
    psql $POSTGRES_URL -f scripts/postgres.sql

tpch:setup-python
    RUST_LOG=connector_agent_python=debug PYTHONPATH=connector-agent-python python scripts/test_tpch.py