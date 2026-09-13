"""Check an installed ConnectorX wheel in fresh, isolated Python processes."""

import argparse
from email.parser import Parser
import hashlib
from pathlib import Path
import os
import platform
import subprocess
import sys
import tempfile
import zipfile


IMPORT_CHECK = """
import importlib
from importlib.metadata import distribution
import os
from pathlib import Path
import sys

order, expected_version = sys.argv[1:]
crypto_keys = lambda: {
    key: value for key, value in os.environ.items()
    if key.startswith("OPENSSL_") or key in ("SSL_CERT_FILE", "SSL_CERT_DIR")
}
before = crypto_keys()
if order == "ssl-first":
    import ssl

import connectorx
from connectorx import read_sql

dist = distribution("connectorx")
assert dist.version == expected_version, (dist.version, expected_version)
assert connectorx.__version__ == expected_version
package_dir = Path(dist.locate_file("connectorx")).resolve()
assert Path(connectorx.__file__).resolve().parent == package_dir
native = importlib.import_module("connectorx.connectorx")
assert Path(native.__file__).resolve().parent == package_dir
for _ in range(2):
    assert importlib.import_module("connectorx") is connectorx
    assert connectorx.read_sql is read_sql
if order == "ssl-last":
    import ssl
assert crypto_keys() == before, (before, crypto_keys())
print(f"{connectorx.__version__} at {connectorx.__file__}")
"""

SQLITE_CHECK = """
import sqlite3
from pathlib import Path
import connectorx

path = Path("smoke.db").resolve()
with sqlite3.connect(path) as connection:
    connection.execute("CREATE TABLE example (value INTEGER)")
    connection.execute("INSERT INTO example VALUES (42)")
for _ in range(2):
    frame = connectorx.read_sql(
        f"sqlite://{path}", "SELECT value FROM example", return_type="polars"
    )
    assert frame.to_dict(as_series=False) == {"value": [42]}
print("two queries passed")
"""


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("wheel", type=Path)
    parser.add_argument(
        "--databricks",
        action="store_true",
        help="Require the Python, architecture, and FIPS environment from issue #831",
    )
    parser.add_argument(
        "--sqlite",
        action="store_true",
        help="Also query SQLite (requires polars and pyarrow in the environment)",
    )
    args = parser.parse_args()
    if args.databricks:
        if sys.version_info[:3] != (3, 12, 11) or platform.machine() != "x86_64":
            parser.error("The Databricks regression requires Python 3.12.11 on x86_64")
        if os.environ.get("OPENSSL_FORCE_FIPS_MODE") != "0":
            parser.error("The Databricks regression requires OPENSSL_FORCE_FIPS_MODE=0")
    with zipfile.ZipFile(args.wheel) as wheel:
        metadata_paths = [
            name for name in wheel.namelist() if name.endswith(".dist-info/METADATA")
        ]
        if len(metadata_paths) != 1:
            parser.error("Expected exactly one wheel METADATA file")
        metadata = Parser().parsestr(wheel.read(metadata_paths[0]).decode("utf-8"))
    if metadata["Name"] != "connectorx" or not metadata["Version"]:
        parser.error("Expected a ConnectorX wheel with a version")
    version = metadata["Version"]
    digest = hashlib.sha256(args.wheel.read_bytes()).hexdigest()
    print(f"Checking {args.wheel.name}, sha256={digest}", flush=True)
    checks = [
        (f"{order}, process {attempt}", IMPORT_CHECK, [order, version])
        for order in ("connectorx-only", "ssl-first", "ssl-last")
        for attempt in (1, 2)
    ]
    if args.sqlite:
        checks.append(("SQLite-to-Polars", SQLITE_CHECK, []))
    with tempfile.TemporaryDirectory(prefix="connectorx-import-") as cwd:
        for name, code, arguments in checks:
            try:
                result = subprocess.run(
                    [sys.executable, "-I", "-c", code, *arguments],
                    cwd=cwd,
                    capture_output=True,
                    text=True,
                    timeout=30,
                )
            except subprocess.TimeoutExpired as error:
                raise SystemExit(
                    f"{name}: timed out after 30 seconds: {error}"
                ) from error
            if result.returncode != 0:
                raise SystemExit(
                    f"{name}: child exited with {result.returncode}\n"
                    f"{result.stdout}{result.stderr}"
                )
            print(f"{name}: {result.stdout.strip()}", flush=True)


if __name__ == "__main__":
    main()
