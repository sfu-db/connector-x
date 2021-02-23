build-release:
    cargo build --release

build-debug:
    cargo build

setup-python: build-release
    cp target/release/libconnector_agent_python.so python/connector_agent_python.so
    
test:
    cargo test

run-python-agent:
    python python/python-agent.py
