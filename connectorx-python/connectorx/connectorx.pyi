from __future__ import annotations

from typing import overload, Literal, Any, TypeAlias, TypedDict
import numpy as np

_ArrowArrayPtr: TypeAlias = int
_ArrowSchemaPtr: TypeAlias = int
_Header: TypeAlias = str

class PandasBlockInfo:
    cids: list[int]
    dt: int

class _DataframeInfos(TypedDict):
    data: list[tuple[np.ndarray, ...] | np.ndarray]
    headers: list[_Header]
    block_infos: list[PandasBlockInfo]

_ArrowInfos = tuple[list[_Header], list[list[tuple[_ArrowArrayPtr, _ArrowSchemaPtr]]]]

@overload
def read_sql(
    conn: str,
    return_type: Literal["pandas"],
    protocol: str | None,
    queries: list[str] | None,
    partition_query: dict[str, Any] | None,
    pre_execution_queries: list[str] | None,
) -> _DataframeInfos: ...
@overload
def read_sql(
    conn: str,
    return_type: Literal["arrow"],
    protocol: str | None,
    queries: list[str] | None,
    partition_query: dict[str, Any] | None,
    pre_execution_queries: list[str] | None,
) -> _ArrowInfos: ...
def partition_sql(conn: str, partition_query: dict[str, Any]) -> list[str]: ...
def read_sql2(sql: str, db_map: dict[str, str]) -> _ArrowInfos: ...
def get_meta(
    conn: str,
    protocol: Literal["csv", "binary", "cursor", "simple", "text"] | None,
    query: str,
) -> _DataframeInfos: ...
