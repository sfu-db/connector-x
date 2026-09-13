# Getting Started

## Installation

### Pip

The easiest way to install ConnectorX is using pip, with the following command:

```bash
pip install connectorx
```

### Databricks containers

On `databricksruntime/minimal:16.4-LTS`, some Linux wheels abort during
`import connectorx` with:

```text
crypto/fips/fips.c:154: OpenSSL internal error: FATAL FIPS SELFTEST FAILURE
```

This was reproduced with the Linux x86_64 wheels for 0.4.3 and 0.4.6a1,
using Python 3.12.11. Those wheels bundle a distribution-specific OpenSSL
1.1 library through Kerberos/GSSAPI. Its initializer aborts even with the
container's inherited `OPENSSL_FORCE_FIPS_MODE=0`. This happens before a
database connection; changing PostgreSQL connection options will not fix it.

The Linux release build now builds MIT Kerberos against OpenSSL 3 before
packaging the wheel. GSSAPI remains enabled, and ConnectorX does not change
the application's OpenSSL environment or host FIPS settings. This is an
import-compatibility fix, **not a claim of FIPS-compliant or certified
cryptographic operation**.

To check a candidate Python 3.12 Linux x86_64 wheel on a machine with Bash
and Docker's Linux engine:

```bash
bash scripts/check-databricks-wheel.sh path/to/connectorx-wheel.whl
```

The check uses the image digest from the original report, installs the supplied
wheel without querying PyPI, and checks repeated imports and both import orders
with Python's `ssl` module in fresh processes. It leaves inherited crypto
settings unchanged. The `release` workflow can also be run manually to build
and check a candidate Linux x86_64/Python 3.12 wheel without publishing it.
Do not assume an older published wheel contains this build change.

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
