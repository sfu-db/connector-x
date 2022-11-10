# Getting Started

## Installation

### Pip

The easiest way to install ConnectorX is using pip, with the following command:

```bash
pip install connectorx
```

### Build from source code

* Step 0: Install tools.
    * Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    * Install [just](https://github.com/casey/just): `cargo install just`
    * Install [Poetry](https://python-poetry.org/docs/): `pip3 install poetry`

* Step 1: Fresh clone of source.
```bash
git clone https://github.com/sfu-db/connector-x.git
```

* Step 2: Install and switch to the correct rust version (please refer [this file](https://github.com/sfu-db/connector-x/blob/main/.github/workflows/release.yml) and search for `rust` for the latest using version).
```bash
rustup install {version}
rustup override set {version}
```

* Step 3: Install system dependencies. Please refer to [release.yml](https://github.com/sfu-db/connector-x/blob/main/.github/workflows/release.yml) for dependencies needed for different os.

* Step 4: Install python dependencies.
```bash
just bootstrap-python
```

* Step 5: Build wheel.
```bash
just build-python-wheel
```

NOTES:
* `OPENSSL_NO_VENDOR=1` might required to compile for windows users.
* Dynamic library is required for the python installation. (e.g. If you are using `pyenv`, use command `PYTHON_CONFIGURE_OPTS=“--enable-shared” pyenv install {version}` to install python since dylib is not enabled by default.)

