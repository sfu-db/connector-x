import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def clickhouse_url() -> str:
    conn = os.environ["CLICKHOUSE_URL"]
    return conn


@pytest.mark.skipif(not os.environ.get("CLICKHOUSE_URL"), reason="Do not test Clickhouse unless `CLICKHOUSE_URL` is set")
def test_clickhouse_without_partition(clickhouse_url: str) -> None:
    query = "select * from test_table limit 3"
    # clickhouse does not support binary protocol
    df = read_sql(clickhouse_url, query, protocol="text")
    # result from clickhouse might have different order each time
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 3], dtype="Int64"),
            "test_str": pd.Series(["abc", "defg", "hijkl"], dtype="object")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(not os.environ.get("CLICKHOUSE_URL"), reason="Do not test Clickhouse unless `CLICKHOUSE_URL` is set")
def test_clickhouse_with_partition(clickhouse_url: str) -> None:
    query = "select * from test_table"
    df = read_sql(
        clickhouse_url,
        query,
        partition_on="test_int",
        partition_num=3,
        protocol="text"
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "test_str": pd.Series(["abc", "defg", "hijkl", "mnopqr", 'st', 'u'], dtype="object")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(not os.environ.get("CLICKHOUSE_URL"), reason="Do not test Clickhouse unless `CLICKHOUSE_URL` is set")
def test_clickhouse_types(clickhouse_url: str) -> None:
    query = "select * from test_types"
    df = read_sql(clickhouse_url, query, protocol="text")
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 3], dtype="Int64"),
            "test_float": pd.Series([2.3, 3.3, 4.3], dtype="float64"),
            "test_date": pd.Series(["1999-07-25", "1979-04-07", "1999-09-22"], dtype="datetime64[ns]"),
            "test_datetime": pd.Series(["1999-07-25 23:14:07", "1979-04-07 03:04:37", "1999-07-25 20:21:14"], dtype="datetime64[ns]"),
            "test_decimal": pd.Series(["2.22", "3.33", "4.44"], dtype="object"),
            "test_varchar": pd.Series(["こんにちは", "Ha好ち😁ðy", "b"], dtype="object"),
            "test_char": pd.Series(["0123456789", "abcdefghij", "321"], dtype="object")
        }
    )
    assert_frame_equal(df, expected, check_names=True)
