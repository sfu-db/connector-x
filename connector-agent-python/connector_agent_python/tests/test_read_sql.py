import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def postgres_url() -> str:
    conn = os.environ["POSTGRES_URL"]
    return conn


def test_manual_partition(postgres_url: str) -> None:

    queries = [
        "SELECT * FROM test_table WHERE test_int < 2",
        "SELECT * FROM test_table WHERE test_int >= 2",
    ]

    df = read_sql(postgres_url, query=queries)

    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 0, 2, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([3, 5, None, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["str1", "a", "str2", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([None, 3.1, 2.2, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [True, None, False, False, None, True], dtype="boolean"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_read_sql_without_partition(postgres_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([3, None, 5, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["str1", "str2", "a", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([None, 2.2, 3.1, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [True, False, None, False, None, True], dtype="boolean"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_read_sql_with_partition(postgres_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        postgres_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([3, None, 5, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["str1", "str2", "a", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([None, 2.2, 3.1, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [True, False, None, False, None, True], dtype="boolean"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_read_sql_with_partition_without_partition_range(postgres_url: str) -> None:
    query = "SELECT * FROM test_table where test_float > 3"
    df = read_sql(
        postgres_url,
        query,
        partition_on="test_int",
        partition_num=3,
    )

    expected = pd.DataFrame(
        index=range(2),
        data={
            "test_int": pd.Series([0, 4], dtype="Int64"),
            "test_nullint": pd.Series([5, 9], dtype="Int64"),
            "test_str": pd.Series(
                ["a", "c"], dtype="object"
            ),
            "test_float": pd.Series([3.1, 7.8], dtype="float64"),
            "test_bool": pd.Series(
                [None, None], dtype="boolean"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_read_sql_with_partition_and_selection(postgres_url: str) -> None:
    query = "SELECT * FROM test_table WHERE 1 = 3 OR 2 = 2"
    df = read_sql(
        postgres_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([3, None, 5, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["str1", "str2", "a", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([None, 2.2, 3.1, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [True, False, None, False, None, True], dtype="boolean"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)