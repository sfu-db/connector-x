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

python-tpch n="1": setup-python
      cd connector-agent-python && \
      poetry run python ../scripts/test_tpch.py {{n}}

tpch:
    cd connector-agent-python && LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.8.6/lib/ cargo criterion --no-default-features --features executable --bench tpch -- --profile-time=300

tpch-old:
    cd connector-agent-python && LD_LIBRARY_PATH=$HOME/.pyenv/versions/3.8.6/lib/ cargo criterion --no-default-features --features executable --bench tpch_old -- --profile-time=300
