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
    cd connector-agent-python && poetry run pytest connector_agent_python/tests -v -s

seed-db:
    psql $POSTGRES_URL -c "DROP TABLE IF EXISTS test_table;"
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
build-python-wheel:
    cd connector-agent-python && poetry build
    
rename-wheel:
    #!/usr/bin/env python3
    from pathlib import Path
    import sys
    import os

    platform = sys.platform
    for p in Path("connector-agent-python/connector_agent_python").iterdir():
        print("file is", p)
        if platform == "win32" and p.suffix == ".pyd":
            platform_string = p.suffixes[0][1:]
            abi_tag, platform_tag = platform_string.split("-")
            py_tag = abi_tag.rstrip("m")
            platform_string = f"{py_tag}-{abi_tag}-{platform_tag}"
        elif platform == "linux" and p.suffix == ".so":
            platform_string = p.suffixes[0][1:]
            cpython, abi_tag, *platform_tag = platform_string.split("-")
            py_tag = abi_tag.rstrip("m")
            platform_string = f"cp{py_tag}-cp{abi_tag}-manylinux2014_x86_64"
        elif platform == "darwin" and p.suffix == ".so": 
            platform_string = p.suffixes[0][1:]
            cpython, abi_tag, *platform_tag = platform_string.split("-")
            py_tag = abi_tag.rstrip("m")
            platform_string = f"cp{py_tag}-cp{abi_tag}-macosx_10_15_intel"
        else:
            pass

    for p in Path("connector-agent-python/dist").iterdir():
        if p.suffix == ".whl":
            pkgname, version, *rest = p.stem.split("-")
            break

    os.rename(
        p,
        f"connector-agent-python/dist/{pkgname}-{version}-{platform_string}.whl",
    )