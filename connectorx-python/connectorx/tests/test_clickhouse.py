import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql, ConnectionUrl

# clickhouse_url fixture is now defined in conftest.py
# It uses testcontainers if available, otherwise the CLICKHOUSE_URL environment variable

def test_clickhouse_without_partition(clickhouse_url: str) -> None:
    query = "select * from test_table limit 3"
    # clickhouse does not support binary protocol
    df = read_sql(clickhouse_url, query)
    # result from clickhouse might have different order each time
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 3], dtype="int64"),
            "test_str": pd.Series(["abc", "defg", "hijkl"], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_clickhouse_with_partition(clickhouse_url: str) -> None:
    query = "select * from test_table"
    df = read_sql(
        clickhouse_url, query, partition_on="test_int", partition_num=3
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 6], dtype="int64"),
            "test_str": pd.Series(
                ["abc", "defg", "hijkl", "mnopqr", "st", "u"], dtype="object"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_clickhouse_types(clickhouse_url: str) -> None:
    query = "select * from test_types"
    df = read_sql(clickhouse_url, query)
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 3], dtype="int64"),
            "test_float": pd.Series([2.3, 3.3, 4.3], dtype="float64"),
            "test_date": pd.Series(
                ["1999-07-25", "1979-04-07", "2149-01-01"], dtype="datetime64[us]"
            ),
            "test_datetime": pd.Series(
                ["1999-07-25 23:14:07", "1979-04-07 03:04:37", "2106-01-01 00:00:00"],
                dtype="datetime64[us]",
            ),
            "test_decimal": pd.Series(["2.22", "3.33", "4.44"], dtype="float64"),
            "test_varchar": pd.Series(["こんにちは", "Ha好ち😁ðy", "b"], dtype="object"),
            "test_char": pd.Series(["0123456789", "abcdefghij", "321"], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)
