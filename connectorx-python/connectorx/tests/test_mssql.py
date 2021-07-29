import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def mssql_url() -> str:
    conn = os.environ["MSSQL_URL"]
    return conn


@pytest.mark.xfail
def test_on_non_select(mssql_url: str) -> None:
    query = "CREATE TABLE non_select(id INTEGER NOT NULL)"
    df = read_sql(mssql_url, query)


def test_aggregation(mssql_url: str) -> None:
    query = (
        "SELECT test_bool, SUM(test_float) as sum FROM test_table GROUP BY test_bool"
    )
    df = read_sql(mssql_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([None, False, True], dtype="boolean"),
            "sum": pd.Series([10.9, 5.2, -10.0], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_partition_on_aggregation(mssql_url: str) -> None:
    query = (
        "SELECT test_bool, SUM(test_int) AS test_int FROM test_table GROUP BY test_bool"
    )
    df = read_sql(mssql_url, query, partition_on="test_int", partition_num=2)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([None, False, True], dtype="boolean"),
            "test_int": pd.Series([4, 5, 1315], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_aggregation2(mssql_url: str) -> None:
    query = "select DISTINCT(test_bool) from test_table"
    df = read_sql(mssql_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([None, False, True], dtype="boolean"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_partition_on_aggregation2(mssql_url: str) -> None:
    query = "select MAX(test_int) as max, MIN(test_int) as min from test_table"
    df = read_sql(mssql_url, query, partition_on="max", partition_num=2)
    expected = pd.DataFrame(
        index=range(1),
        data={
            "max": pd.Series([1314], dtype="Int64"),
            "min": pd.Series([0], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_udf(mssql_url: str) -> None:
    query = (
        "SELECT dbo.increment(test_int) AS test_int FROM test_table ORDER BY test_int"
    )
    df = read_sql(mssql_url, query, partition_on="test_int", partition_num=2)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 1315], dtype="Int64"),
        },
    )
    df = df.sort_values("test_int").reset_index(drop=True)
    assert_frame_equal(df, expected, check_names=True)


def test_manual_partition(mssql_url: str) -> None:

    queries = [
        "SELECT * FROM test_table WHERE test_int < 2",
        "SELECT * FROM test_table WHERE test_int >= 2",
    ]

    df = read_sql(mssql_url, query=queries)

    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 0, 2, 3, 4, 1314], dtype="int64"),
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


def test_read_sql_without_partition(mssql_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(mssql_url, query)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="int64"),
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


def test_read_sql_with_partition(mssql_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        mssql_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="int64"),
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


def test_read_sql_with_partition_without_partition_range(mssql_url: str) -> None:
    query = "SELECT * FROM test_table where test_float > 3"
    df = read_sql(
        mssql_url,
        query,
        partition_on="test_int",
        partition_num=3,
    )

    expected = pd.DataFrame(
        index=range(2),
        data={
            "test_int": pd.Series([0, 4], dtype="int64"),
            "test_nullint": pd.Series([5, 9], dtype="Int64"),
            "test_str": pd.Series(["a", "c"], dtype="object"),
            "test_float": pd.Series([3.1, 7.8], dtype="float64"),
            "test_bool": pd.Series([None, None], dtype="boolean"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_read_sql_with_partition_and_selection(mssql_url: str) -> None:
    query = "SELECT * FROM test_table WHERE 1 = 3 OR 2 = 2"
    df = read_sql(
        mssql_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="int64"),
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


def test_read_sql_with_partition_and_projection(mssql_url: str) -> None:
    query = "SELECT test_int, test_float, test_str FROM test_table"
    df = read_sql(
        mssql_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="int64"),
            "test_float": pd.Series([None, 2.2, 3.1, 3, 7.8, -10], dtype="float64"),
            "test_str": pd.Series(
                ["str1", "str2", "a", "b", "c", None], dtype="object"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_read_sql_with_partition_and_spja(mssql_url: str) -> None:
    query = """
    SELECT test_bool, AVG(test_float) AS avg, SUM(test_int) AS sum 
    FROM test_table AS a, test_str AS b 
    WHERE a.test_int = b.id AND test_nullint IS NOT NULL 
    GROUP BY test_bool 
    ORDER BY sum
    """
    df = read_sql(mssql_url, query, partition_on="sum", partition_num=2)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([True, False, None], dtype="boolean"),
            "avg": pd.Series([None, 3, 5.45], dtype="float64"),
            "sum": pd.Series([1, 3, 4], dtype="Int64"),
        },
    )
    df = df.sort_values("sum").reset_index(drop=True)
    assert_frame_equal(df, expected, check_names=True)


def test_empty_result(mssql_url: str) -> None:
    query = "SELECT * FROM test_table where test_int < -100"
    df = read_sql(mssql_url, query)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([], dtype="int64"),
            "test_nullint": pd.Series([], dtype="Int64"),
            "test_str": pd.Series([], dtype="object"),
            "test_float": pd.Series([], dtype="float64"),
            "test_bool": pd.Series([], dtype="boolean"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_empty_result_on_some_partition(mssql_url: str) -> None:
    query = "SELECT * FROM test_table where test_int < 1"
    df = read_sql(mssql_url, query, partition_on="test_int", partition_num=3)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([0], dtype="int64"),
            "test_nullint": pd.Series([5], dtype="Int64"),
            "test_str": pd.Series(["a"], dtype="object"),
            "test_float": pd.Series([3.1], dtype="float"),
            "test_bool": pd.Series([None], dtype="boolean"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)
