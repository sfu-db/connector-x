"""Errors raised for bad arguments, before any database is contacted."""

import pytest

import connectorx as cx

# Nothing listens on port 1: the protocol is rejected before connecting.
POSTGRES_URL = "postgresql://user:pass@127.0.0.1:1/db"
MYSQL_URL = "mysql://user:pass@127.0.0.1:1/db"


@pytest.mark.parametrize("return_type", ["pandas", "arrow", "polars", "arrow_stream"])
def test_unsupported_postgres_protocol_raises(return_type: str) -> None:
    if return_type == "polars":
        pytest.importorskip("polars")
    with pytest.raises(
        RuntimeError, match=r'Protocol "text" is not supported for Postgres'
    ):
        cx.read_sql(POSTGRES_URL, "SELECT 1", protocol="text", return_type=return_type)


@pytest.mark.parametrize("return_type", ["pandas", "arrow"])
def test_unsupported_mysql_protocol_raises(return_type: str) -> None:
    with pytest.raises(
        RuntimeError,
        match=r'Protocol "cursor" is not supported for MySQL; supported protocols: binary, text',
    ):
        cx.read_sql(MYSQL_URL, "SELECT 1", protocol="cursor", return_type=return_type)


def test_unsupported_protocol_in_get_meta_raises() -> None:
    with pytest.raises(
        RuntimeError, match=r'Protocol "text" is not supported for Postgres'
    ):
        cx.get_meta(POSTGRES_URL, "SELECT 1", protocol="text")
