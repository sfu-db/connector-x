from __future__ import annotations

from typing import overload, Literal, Any, TypeAlias
import pandas as pd

_ArrowArrayPtr: TypeAlias = int
_ArrowSchemaPtr: TypeAlias = int
_Column: TypeAlias = str

@overload
def read_sql(
    conn: str,
    return_type: Literal["pandas"],
    protocol: str | None,
    queries: list[str] | None,
    partition_query: dict[str, Any] | None,
) -> pd.DataFrame: ...
@overload
def read_sql(
    conn: str,
    return_type: Literal["arrow", "arrow2"],
    protocol: str | None,
    queries: list[str] | None,
    partition_query: dict[str, Any] | None,
) -> tuple[list[_Column], list[list[tuple[_ArrowArrayPtr, _ArrowSchemaPtr]]]]: ...
def partition_sql(conn: str, partition_query: dict[str, Any]) -> list[str]: ...
def read_sql2(
    sql: str, db_map: dict[str, str]
) -> tuple[list[_Column], list[list[tuple[_ArrowArrayPtr, _ArrowSchemaPtr]]]]: ...
def get_meta(
    conn: str,
    protocol: Literal["csv", "binary", "cursor", "simple", "text"] | None,
    query: str,
) -> dict[str, Any]: ...
