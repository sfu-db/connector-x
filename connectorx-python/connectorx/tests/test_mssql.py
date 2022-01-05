import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def mssql_url() -> str:
    conn = os.environ["MSSQL_URL"]
    # conn = os.environ["AZURE_MSSQL_URL"]
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
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
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
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
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
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="int64"),
            "test_nullint": pd.Series([5, 3, None, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["a", "str1", "str2", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([3.1, None, 2.2, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [None, True, False, False, None, True], dtype="boolean"
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_mssql_without_partition(mssql_url: str) -> None:
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


def test_mssql_limit_without_partition(mssql_url: str) -> None:
    query = "SELECT top 3 * FROM test_table"
    df = read_sql(mssql_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 0], dtype="int64"),
            "test_nullint": pd.Series([3, None, 5], dtype="Int64"),
            "test_str": pd.Series(["str1", "str2", "a"], dtype="object"),
            "test_float": pd.Series([None, 2.2, 3.1], dtype="float64"),
            "test_bool": pd.Series([True, False, None], dtype="boolean"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_mssql_limit_large_without_partition(mssql_url: str) -> None:
    query = "SELECT top 10 * FROM test_table"
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


def test_mssql_with_partition(mssql_url: str) -> None:
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
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="int64"),
            "test_nullint": pd.Series([5, 3, None, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["a", "str1", "str2", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([3.1, None, 2.2, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [None, True, False, False, None, True], dtype="boolean"
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_mssql_limit_with_partition(mssql_url: str) -> None:
    query = "SELECT top 3 * FROM test_table"
    df = read_sql(
        mssql_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([0, 1, 2], dtype="int64"),
            "test_nullint": pd.Series([5, 3, None], dtype="Int64"),
            "test_str": pd.Series(["a", "str1", "str2"], dtype="object"),
            "test_float": pd.Series([3.1, None, 2.20], dtype="float64"),
            "test_bool": pd.Series([None, True, False], dtype="boolean"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_mssql_limit_large_with_partition(mssql_url: str) -> None:
    query = "SELECT top 10 * FROM test_table"
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
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="int64"),
            "test_nullint": pd.Series([5, 3, None, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["a", "str1", "str2", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([3.1, None, 2.2, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [None, True, False, False, None, True], dtype="boolean"
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_mssql_with_partition_without_partition_range(mssql_url: str) -> None:
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
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_mssql_with_partition_and_selection(mssql_url: str) -> None:
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
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="int64"),
            "test_nullint": pd.Series([5, 3, None, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["a", "str1", "str2", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([3.1, None, 2.2, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [None, True, False, False, None, True], dtype="boolean"
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_mssql_with_partition_and_projection(mssql_url: str) -> None:
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
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="int64"),
            "test_float": pd.Series([3.1, None, 2.2, 3, 7.8, -10], dtype="float64"),
            "test_str": pd.Series(
                ["a", "str1", "str2", "b", "c", None], dtype="object"
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_mssql_with_partition_and_spja(mssql_url: str) -> None:
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


def test_empty_result_on_partition(mssql_url: str) -> None:
    query = "SELECT * FROM test_table where test_int < -100"
    df = read_sql(mssql_url, query, partition_on="test_int", partition_num=3)
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


def test_mssql_types(mssql_url: str) -> None:
    query = "SELECT * FROM test_types"
    df = read_sql(mssql_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int1": pd.Series([0, 255, None], dtype="Int64"),
            "test_int2": pd.Series([-32768, 32767, None], dtype="Int64"),
            "test_int4": pd.Series([-2147483648, 2147483647, None], dtype="Int64"),
            "test_int8": pd.Series(
                [-9223372036854775808, 9223372036854775807, None], dtype="Int64"
            ),
            "test_float24": pd.Series([None, 1.18e-38, 3.40e38], dtype="float"),
            "test_float53": pd.Series([None, -2.23e-308, 1.79e308], dtype="float"),
            "test_floatn": pd.Series([None, 0, 123.1234567], dtype="float"),
            "test_date": pd.Series(
                ["1999-07-25", None, "2021-01-28"], dtype="datetime64[ns]"
            ),
            "test_time": pd.Series(["00:00:00", "23:59:59", None], dtype="object"),
            "test_datetime": pd.Series(
                [None, "2020-12-31 23:59:59", "2021-01-28 10:30:30"],
                dtype="datetime64[ns]",
            ),
            "test_smalldatetime": pd.Series(
                ["1990-01-01 10:00:00", None, "2079-06-05 23:00:00"],
                dtype="datetime64[ns]",
            ),
            "test_naivedatetime": pd.Series(
                ["1753-01-01 12:00:00", "2038-12-31 01:00:00", None],
                dtype="datetime64[ns]",
            ),
            "test_naivedatetime2": pd.Series(
                ["1900-01-01 12:00:00.12345", None, "2027-03-18 14:30:30.54321"],
                dtype="datetime64[ns]",
            ),
            "test_new_decimal": pd.Series([1.1, 2.2, None], dtype="float"),
            "test_decimal": pd.Series([1, 2, None], dtype="float"),
            "test_varchar": pd.Series([None, "varchar2", "varchar3"], dtype="object"),
            "test_char": pd.Series([None, "char2     ", "char3     "], dtype="object"),
            "test_varbinary": pd.Series([None, b"1234", b""], dtype="object"),
            "test_binary": pd.Series(
                [None, b"12\x00\x00\x00", b"\x00\x00\x00\x00\x00"], dtype="object"
            ),
            "test_nchar": pd.Series(["1234", None, "12  "], dtype="object"),
            "test_text": pd.Series(["text", "t", None], dtype="object"),
            "test_ntext": pd.Series(["ntext", "nt", None], dtype="object"),
            "test_uuid": pd.Series(
                [
                    "86b494cc-96b2-11eb-9298-3e22fbb9fe9d",
                    None,
                    "86b49b84-96b2-11eb-9298-3e22fbb9fe9d",
                ],
                dtype="object",
            ),
            "test_money": pd.Series(
                [None, 922337203685477.5807, -922337203685477.5808], dtype="float"
            ),
            "test_smallmoney": pd.Series(
                [None, 214748.3647, -214748.3648], dtype="float"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_mssql_unicode(mssql_url: str) -> None:
    query = "SELECT test_hello FROM test_str where 1 <= id and id <= 4"
    df = read_sql(mssql_url, query)
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_hello": pd.Series(
                ["你好", "こんにちは", "Здра́вствуйте", "😁😂😜"], dtype="object"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_mssql_cte(mssql_url: str) -> None:
    query = "with test_cte (test_int, test_str) as (select test_int, test_str from test_table where test_float > 0) select test_int, test_str from test_cte"
    df = read_sql(mssql_url, query, partition_on="test_int", partition_num=3)
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_int": pd.Series([0, 2, 3, 4], dtype="int64"),
            "test_str": pd.Series(["a", "str2", "b", "c"], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)
