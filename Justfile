build-release:
    cargo build --release

build-debug:
    cargo build

setup-python:
    cd connector-agent-python && poetry run maturin develop --release --strip
    
test:
    cargo test

test-python: setup-python
    cd connector-agent-python && poetry run pytest connector_agent_python/tests -v