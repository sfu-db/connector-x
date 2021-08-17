import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def mysql_url() -> str:
    conn = os.environ["MYSQL_URL"]
    return conn


def test_mysql_without_partition(mysql_url: str) -> None:
    query = "select * from test_table limit 3"
    df = read_sql(mysql_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 3], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3], dtype="float64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_mysql_with_partition(mysql_url: str) -> None:
    query = "select * from test_table"
    df = read_sql(
        mysql_url,
        query,
        partition_on="test_int",
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3, 4.4, 5.5, 6.6], dtype="float64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_mysql_types(mysql_url: str) -> None:
    query = "select * from test_types"
    df = read_sql(mysql_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_date": pd.Series(["1999-07-25", "2020-12-31", "2021-01-28"], dtype="datetime64[ns]"),
            "test_time": pd.Series(["00:00:00", "23:59:59", "12:30:30"], dtype="object"),
            "test_datetime": pd.Series(["1999-07-25 00:00:00", "2020-12-31 23:59:59", None], dtype="datetime64[ns]"),
            "test_new_decimal": pd.Series([1.1, None, 3.3], dtype="float"),
            "test_decimal": pd.Series([1, 2, 3], dtype="float"),
            "test_varchar": pd.Series([None, "varchar2", "varchar3"], dtype="object"),
            "test_char": pd.Series(["char1", "char2", "char3"], dtype="object")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_mysql_types_text(mysql_url: str) -> None:
    query = "select * from test_types"
    df = read_sql(mysql_url, query, protocol="text")
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_date": pd.Series(["1999-07-25", "2020-12-31", "2021-01-28"], dtype="datetime64[ns]"),
            "test_time": pd.Series(["00:00:00", "23:59:59", "12:30:30"], dtype="object"),
            "test_datetime": pd.Series(["1999-07-25 00:00:00", "2020-12-31 23:59:59", None], dtype="datetime64[ns]"),
            "test_new_decimal": pd.Series([1.1, None, 3.3], dtype="float"),
            "test_decimal": pd.Series([1, 2, 3], dtype="float"),
            "test_varchar": pd.Series([None, "varchar2", "varchar3"], dtype="object"),
            "test_char": pd.Series(["char1", "char2", "char3"], dtype="object")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_mysql_more_types(mysql_url: str) -> None:
    query = "select * from test_more_types"
    df = read_sql(mysql_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_tiny": pd.Series([0, 1, 0], dtype="Int64"),
            "test_short": pd.Series([-28, 128, 725], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3], dtype="float"),
            "test_year": pd.Series([1901, 2021, 2155], dtype="Int64"),
            "test_timestamp": pd.Series(["1990-01-01 00:00:01", "2021-08-05 12:12:12", "2038-01-19 03:14:07"], dtype="datetime64[ns]"),
            "test_blob": pd.Series([b"blobblobblobblob1", b"blobblobblobblob2", b"blobblobblobblob3"], dtype="object"),
            "test_enum": pd.Series(["apple", "orange", "mango"], dtype="object"),
            "test_json": pd.Series(['{"age":1,"name":"piggy"}', '{"age":2,"name":"kitty"}', '{"age":3,"name":"puppy"}'], dtype="object")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_empty_result(mysql_url: str) -> None:
    query = "SELECT * FROM test_table where test_int < -100"
    df = read_sql(mysql_url, query)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([], dtype="object"),
            "test_float": pd.Series([], dtype="object"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_empty_result_on_some_partition(mysql_url: str) -> None:
    query = "SELECT * FROM test_table where test_int = 6"
    df = read_sql(mysql_url, query, partition_on="test_int", partition_num=3)
    expected = pd.DataFrame(
        index=range(1),
        data={
            "test_int": pd.Series([6], dtype="Int64"),
            "test_float": pd.Series([6.6], dtype="float64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)
