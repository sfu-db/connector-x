import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql

@pytest.fixture(scope="module")  # type: ignore
def oracle_url() -> str:
    conn = os.environ["ORACLE_URL"]
    return conn


def test_oracle_types(oracle_url: str) -> None:
    query = "select * from test_types"
    df = read_sql(oracle_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "TEST_INT": pd.Series([1, 2, 3], dtype="Int64"),
            "TEST_FLOAT": pd.Series([1.1, 2.2, 3.3], dtype="float64"),
            "TEST_VARCHAR": pd.Series(['varchar1', 'varchar2', 'varchar3'], dtype="object"),
            "TEST_CHAR": pd.Series(['char1', 'char2', 'char3'], dtype="object"),
            "TEST_DATE": pd.Series(['2019-05-21', '2020-05-21', '2021-05-21'], dtype="datetime64[ns]"),
            "TEST_TIMESTAMP": pd.Series(['2019-05-21', '2020-05-21', '2021-05-21'], dtype="datetime64[ns]"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_oracle_without_partition(oracle_url: str) -> None:
    query = "select * from test_partition"
    df = read_sql(oracle_url, query)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "TEST_INT": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "TEST_FLOAT": pd.Series([1.1, 2.2, 3.3, 4.4, 5.5, 6.6], dtype="float64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_oracle_with_partition(oracle_url: str) -> None:
    query = "select * from test_partition"
    df = read_sql(
        oracle_url,
        query,
        partition_on="TEST_INT",
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "TEST_INT": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "TEST_FLOAT": pd.Series([1.1, 2.2, 3.3, 4.4, 5.5, 6.6], dtype="float64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_oracle_with_partition_and_selection(oracle_url: str) -> None:
    query = "select * from test_partition where 1 = 3 OR 2 = 2"
    df = read_sql(
        oracle_url,
        query,
        partition_on="TEST_INT",
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "TEST_INT": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "TEST_FLOAT": pd.Series([1.1, 2.2, 3.3, 4.4, 5.5, 6.6], dtype="float64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_oracle_with_partition_and_range(oracle_url: str) -> None:
    query = "select * from test_partition"
    df = read_sql(
        oracle_url,
        query,
        partition_on="TEST_INT",
        partition_num=3,
        partition_range=(0,10)
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "TEST_INT": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "TEST_FLOAT": pd.Series([1.1, 2.2, 3.3, 4.4, 5.5, 6.6], dtype="float64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_empty_query(oracle_url: str) -> None:
    query = "select * from test_partition where TEST_INT < -100"
    df = read_sql(
        oracle_url,
        query
    )
    expected = pd.DataFrame(
        index=range(0),
        data={
            "TEST_INT": pd.Series([], dtype="Int64"),
            "TEST_FLOAT": pd.Series([], dtype="float64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_empty_query_on_some_partition(oracle_url: str) -> None:
    query = "select * from test_partition where TEST_INT < -100"
    df = read_sql(
        oracle_url,
        query,
        partition_on="TEST_INT", 
        partition_num=3
    )
    expected = pd.DataFrame(
        index=range(0),
        data={
            "TEST_INT": pd.Series([], dtype="Int64"),
            "TEST_FLOAT": pd.Series([], dtype="float64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)