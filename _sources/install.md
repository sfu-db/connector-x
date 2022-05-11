# Getting Started

## Installation

### Pip

The easiest way to install ConnectorX is using pip, with the following command:

```bash
pip install connectorx
```

### Build from source code

* Step 1: Fresh clone of source
```bash
git clone https://github.com/sfu-db/connector-x.git
```

* Step 2: Install and switch to rust nightly (please refer [this file](https://github.com/sfu-db/connector-x/blob/main/.github/workflows/release.yml) and search for `Install Rust` for the latest using version)
```bash
rustup install nightly-{version}
rustup default nightly-{version}
rustup override set nightly-{version}
```

* Step 3: Install dependencies
```bash
just bootstrap-python
```

* Step 4: Build wheel
```bash
just build-python-wheel
```

NOTE: `OPENSSL_NO_VENDOR=1` might required to compile for windows users.
