import os

import numpy as np
import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def sqlite_db() -> str:
    conn = os.environ["SQLITE_URL"]
    return conn


def test_sqlite_without_partition(sqlite_db: str) -> None:
    query = "SELECT test_int, test_nullint, test_str, test_float, test_bool, test_date, test_time, test_datetime FROM test_table"
    df = read_sql(sqlite_db, query)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([3, None, 5, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["str1", "str2", "こんにちは", "b", "Ha好ち😁ðy̆", None], dtype="object"
            ),
            "test_float": pd.Series([None, 2.2, 3.1, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [True, False, None, False, None, True], dtype="boolean"
            ),
            "test_date": pd.Series(
                [
                    np.datetime64("1996-03-13"),
                    np.datetime64("1996-01-30"),
                    np.datetime64("1996-02-28"),
                    np.datetime64("2020-01-12"),
                    np.datetime64("1996-04-20"),
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_time": pd.Series(
                [
                    "08:12:40",
                    "10:03:00",
                    "23:00:10",
                    "23:00:10",
                    "18:30:00",
                    "18:30:00",
                ],
                dtype="object",
            ),
            "test_datetime": pd.Series(
                [
                    np.datetime64("2007-01-01T10:00:19"),
                    np.datetime64("2005-01-01T22:03:00"),
                    None,
                    np.datetime64("1987-01-01T11:00:00"),
                    None,
                    np.datetime64("2007-10-01T10:32:00"),
                ],
                dtype="datetime64[ns]",
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_sqlite_limit_without_partition(sqlite_db: str) -> None:
    query = "SELECT test_int, test_nullint, test_str, test_float, test_bool, test_date, test_time, test_datetime FROM test_table limit 3"
    df = read_sql(sqlite_db, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 0], dtype="Int64"),
            "test_nullint": pd.Series([3, None, 5], dtype="Int64"),
            "test_str": pd.Series(["str1", "str2", "こんにちは"], dtype="object"),
            "test_float": pd.Series([None, 2.2, 3.1], dtype="float64"),
            "test_bool": pd.Series([True, False, None], dtype="boolean"),
            "test_date": pd.Series(
                [
                    np.datetime64("1996-03-13"),
                    np.datetime64("1996-01-30"),
                    np.datetime64("1996-02-28"),
                ],
                dtype="datetime64[ns]",
            ),
            "test_time": pd.Series(
                ["08:12:40", "10:03:00", "23:00:10"], dtype="object"
            ),
            "test_datetime": pd.Series(
                [
                    np.datetime64("2007-01-01T10:00:19"),
                    np.datetime64("2005-01-01T22:03:00"),
                    None,
                ],
                dtype="datetime64[ns]",
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_sqlite_limit_large_without_partition(sqlite_db: str) -> None:
    query = "SELECT test_int, test_nullint, test_str, test_float, test_bool, test_date, test_time, test_datetime FROM test_table limit 10"
    df = read_sql(sqlite_db, query)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([3, None, 5, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["str1", "str2", "こんにちは", "b", "Ha好ち😁ðy̆", None], dtype="object"
            ),
            "test_float": pd.Series([None, 2.2, 3.1, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [True, False, None, False, None, True], dtype="boolean"
            ),
            "test_date": pd.Series(
                [
                    np.datetime64("1996-03-13"),
                    np.datetime64("1996-01-30"),
                    np.datetime64("1996-02-28"),
                    np.datetime64("2020-01-12"),
                    np.datetime64("1996-04-20"),
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_time": pd.Series(
                [
                    "08:12:40",
                    "10:03:00",
                    "23:00:10",
                    "23:00:10",
                    "18:30:00",
                    "18:30:00",
                ],
                dtype="object",
            ),
            "test_datetime": pd.Series(
                [
                    np.datetime64("2007-01-01T10:00:19"),
                    np.datetime64("2005-01-01T22:03:00"),
                    None,
                    np.datetime64("1987-01-01T11:00:00"),
                    None,
                    np.datetime64("2007-10-01T10:32:00"),
                ],
                dtype="datetime64[ns]",
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_sqlite_with_partition(sqlite_db: str) -> None:
    query = "SELECT test_int, test_nullint, test_str, test_float, test_bool, test_date, test_time, test_datetime FROM test_table"
    df = read_sql(
        sqlite_db,
        query,
        partition_on="test_int",
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([5, 3, None, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["こんにちは", "str1", "str2", "b", "Ha好ち😁ðy̆", None], dtype="object"
            ),
            "test_float": pd.Series([3.1, None, 2.2, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [None, True, False, False, None, True], dtype="boolean"
            ),
            "test_date": pd.Series(
                [
                    np.datetime64("1996-02-28"),
                    np.datetime64("1996-03-13"),
                    np.datetime64("1996-01-30"),
                    np.datetime64("2020-01-12"),
                    np.datetime64("1996-04-20"),
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_time": pd.Series(
                [
                    "23:00:10",
                    "08:12:40",
                    "10:03:00",
                    "23:00:10",
                    "18:30:00",
                    "18:30:00",
                ],
                dtype="object",
            ),
            "test_datetime": pd.Series(
                [
                    None,
                    np.datetime64("2007-01-01T10:00:19"),
                    np.datetime64("2005-01-01T22:03:00"),
                    np.datetime64("1987-01-01T11:00:00"),
                    None,
                    np.datetime64("2007-10-01T10:32:00"),
                ],
                dtype="datetime64[ns]",
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_manual_partition(sqlite_db: str) -> None:

    queries = [
        "SELECT test_int, test_nullint, test_str, test_float, test_bool, test_date, test_time, test_datetime FROM test_table WHERE test_int < 2",
        "SELECT test_int, test_nullint, test_str, test_float, test_bool, test_date, test_time, test_datetime FROM test_table WHERE test_int >= 2",
    ]

    df = read_sql(sqlite_db, query=queries)

    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([5, 3, None, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["こんにちは", "str1", "str2", "b", "Ha好ち😁ðy̆", None], dtype="object"
            ),
            "test_float": pd.Series([3.1, None, 2.2, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [None, True, False, False, None, True], dtype="boolean"
            ),
            "test_date": pd.Series(
                [
                    np.datetime64("1996-02-28"),
                    np.datetime64("1996-03-13"),
                    np.datetime64("1996-01-30"),
                    np.datetime64("2020-01-12"),
                    np.datetime64("1996-04-20"),
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_time": pd.Series(
                [
                    "23:00:10",
                    "08:12:40",
                    "10:03:00",
                    "23:00:10",
                    "18:30:00",
                    "18:30:00",
                ],
                dtype="object",
            ),
            "test_datetime": pd.Series(
                [
                    None,
                    np.datetime64("2007-01-01T10:00:19"),
                    np.datetime64("2005-01-01T22:03:00"),
                    np.datetime64("1987-01-01T11:00:00"),
                    None,
                    np.datetime64("2007-10-01T10:32:00"),
                ],
                dtype="datetime64[ns]",
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_sqlite_without_partition_and_spa(sqlite_db: str) -> None:
    query = """
    SELECT test_bool, AVG(test_float) AS avg, SUM(test_int) AS sum 
    FROM test_table
    WHERE test_nullint IS NOT NULL 
    GROUP BY test_bool 
    ORDER BY sum
    """
    df = read_sql(sqlite_db, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([False, None, True], dtype="boolean"),
            "avg": pd.Series([3.00, 5.45, -10.00], dtype="float64"),
            "sum": pd.Series([3, 4, 1315], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_sqlite_with_partition_and_spa(sqlite_db: str) -> None:
    query = """
    SELECT test_bool, AVG(test_float) AS avg, SUM(test_int) AS sum 
    FROM test_table
    WHERE test_nullint IS NOT NULL 
    GROUP BY test_bool 
    ORDER BY sum
    """
    df = read_sql(sqlite_db, query, partition_on="sum", partition_num=2)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([False, None, True], dtype="boolean"),
            "avg": pd.Series([3.00, 5.45, -10.00], dtype="float64"),
            "sum": pd.Series([3, 4, 1315], dtype="Int64"),
        },
    )
    df = df.sort_values("sum").reset_index(drop=True)
    assert_frame_equal(df, expected, check_names=True)


def test_empty_result(sqlite_db: str) -> None:
    query = "SELECT * FROM test_table where test_int < -100"
    df = read_sql(sqlite_db, query)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([], dtype="object"),
            "test_nullint": pd.Series([], dtype="object"),
            "test_str": pd.Series([], dtype="object"),
            "test_float": pd.Series([], dtype="object"),
            "test_bool": pd.Series([], dtype="object"),
            "test_date": pd.Series([], dtype="object"),
            "test_time": pd.Series([], dtype="object"),
            "test_datetime": pd.Series([], dtype="object"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_empty_result_on_partition(sqlite_db: str) -> None:
    query = "SELECT * FROM test_table where test_int < -100"
    df = read_sql(sqlite_db, query, partition_on="test_int", partition_num=3)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([], dtype="object"),
            "test_nullint": pd.Series([], dtype="object"),
            "test_str": pd.Series([], dtype="object"),
            "test_float": pd.Series([], dtype="object"),
            "test_bool": pd.Series([], dtype="object"),
            "test_date": pd.Series([], dtype="object"),
            "test_time": pd.Series([], dtype="object"),
            "test_datetime": pd.Series([], dtype="object"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_empty_result_on_some_partition(sqlite_db: str) -> None:
    query = "SELECT * FROM test_table where test_int < 1"
    df = read_sql(sqlite_db, query, partition_on="test_int", partition_num=3)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([0], dtype="Int64"),
            "test_nullint": pd.Series([5], dtype="Int64"),
            "test_str": pd.Series(["こんにちは"], dtype="object"),
            "test_float": pd.Series([3.1], dtype="float"),
            "test_bool": pd.Series([None], dtype="boolean"),
            "test_date": pd.Series(
                [
                    np.datetime64("1996-02-28"),
                ],
                dtype="datetime64[ns]",
            ),
            "test_time": pd.Series(["23:00:10"], dtype="object"),
            "test_datetime": pd.Series(
                [
                    None,
                ],
                dtype="datetime64[ns]",
            ),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_sqlite_cte(sqlite_db: str) -> None:
    query = "with test_cte (test_int, test_str) as (select test_int, test_str from test_table where test_float > 0) select test_int, test_str from test_cte"
    df = read_sql(sqlite_db, query, partition_on="test_int", partition_num=3)
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_int": pd.Series([0, 2, 3, 4], dtype="Int64"),
            "test_str": pd.Series(["こんにちは", "str2", "b", "Ha好ち😁ðy̆"], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)
