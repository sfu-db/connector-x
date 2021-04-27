import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def postgres_url() -> str:
    conn = os.environ["POSTGRES_URL"]
    return conn


@pytest.mark.xfail
def test_on_non_select(postgres_url: str) -> None:
    query = "CREATE TABLE non_select(id INTEGER NOT NULL)"
    df = read_sql(postgres_url, query)


def test_aggregation(postgres_url: str) -> None:
    query = "SELECT test_bool, SUM(test_float) FROM test_table GROUP BY test_bool"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([None, False, True], dtype="boolean"),
            "sum": pd.Series([10.9, 5.2, -10.0], dtype="float64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_partition_on_aggregation(postgres_url: str) -> None:
    query = "SELECT test_bool, SUM(test_int) AS test_int FROM test_table GROUP BY test_bool"
    df = read_sql(postgres_url, query,
                  partition_on="test_int", partition_num=2)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([None, False, True], dtype="boolean"),
            "test_int": pd.Series([4, 5, 1315], dtype="Int64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_aggregation2(postgres_url: str) -> None:
    query = "select DISTINCT(test_bool) from test_table"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([None, False, True], dtype="boolean"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_partition_on_aggregation2(postgres_url: str) -> None:
    query = "select MAX(test_int), MIN(test_int) from test_table"
    df = read_sql(postgres_url, query,
                  partition_on="max", partition_num=2)
    expected = pd.DataFrame(
        index=range(1),
        data={
            "max": pd.Series([1314], dtype="Int64"),
            "min": pd.Series([0], dtype="Int64"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_udf(postgres_url: str) -> None:
    query = "select increment(test_int) as test_int from test_table ORDER BY test_int"
    df = read_sql(postgres_url, query,
                  partition_on="test_int", partition_num=2)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 1315], dtype="Int64"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


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


def test_read_sql_with_partition_and_projection(postgres_url: str) -> None:
    query = "SELECT test_int, test_float, test_str FROM test_table"
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
            "test_float": pd.Series([None, 2.2, 3.1, 3, 7.8, -10], dtype="float64"),
            "test_str": pd.Series(
                ["str1", "str2", "a", "b", "c", None], dtype="object"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_read_sql_with_partition_and_join(postgres_url: str) -> None:
    query = "SELECT T.test_int, T.test_bool, S.test_language FROM test_table T INNER JOIN test_str S ON T.test_int = S.id"
    df = read_sql(
        postgres_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(5),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4], dtype="Int64"),
            "test_bool": pd.Series([None, True, False, False, None], dtype="boolean"),
            "test_language": pd.Series(
                ["English", "中文", "日本語", "русский", "Emoji"], dtype="object"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_read_sql_with_partition_and_spja(postgres_url: str) -> None:
    query = "select test_bool, AVG(test_float) as avg, SUM(test_int) as sum from test_table as a, test_str as b where a.test_int = b.id AND test_nullint is not NULL GROUP BY test_bool ORDER BY sum"
    df = read_sql(postgres_url, query,
                  partition_on="sum", partition_num=2)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([True, False, None], dtype="boolean"),
            "avg": pd.Series([None, 3, 5.45], dtype="float64"),
            "sum": pd.Series([1, 3, 4], dtype="Int64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_read_sql_on_utf8(postgres_url: str) -> None:
    query = "SELECT * FROM test_str"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        index=range(8),
        data={
            "id": pd.Series([0, 1, 2, 3, 4, 5, 6, 7], dtype="Int64"),
            "test_language": pd.Series(
                ["English", "中文", "日本語", "русский", "Emoji", "Latin1", "Extra", "Mixed"], dtype="object"
            ),
            "test_hello": pd.Series(
                ["Hello", "你好", "こんにちは", "Здра́вствуйте", "😁😂😜", "¥§¤®ð", "y̆", "Ha好ち😁ðy̆"], dtype="object"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_types_binary(postgres_url: str) -> None:
    query = "SELECT test_int16, test_char, test_uuid, test_time, test_json, test_jsonb, test_bytea, test_enum FROM test_types"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_int16": pd.Series([0, 1, 2, 3], dtype="Int64"),
            "test_char": pd.Series(["a", "b", "c", "d"], dtype="object"),
            "test_uuid": pd.Series(
                [
                    "86b494cc-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49b84-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49c42-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49cce-96b2-11eb-9298-3e22fbb9fe9d"
                ], dtype="object"
            ),
            "test_time": pd.Series(["08:12:40", "10:03:00", "23:00:10", "18:30:00"], dtype="object"),
            "test_json": pd.Series(
                [
                    '{"customer":"John Doe","items":{"product":"Beer","qty":6}}',
                    '{"customer":"Lily Bush","items":{"product":"Diaper","qty":24}}',
                    '{"customer":"Josh William","items":{"product":"Toy Car","qty":1}}',
                    '{"customer":"Mary Clark","items":{"product":"Toy Train","qty":2}}',
                ], dtype="object"
            ),
            "test_jsonb": pd.Series(
                [
                    '{"qty":6,"product":"Beer"}',
                    '{"qty":24,"product":"Diaper"}',
                    '{"qty":1,"product":"Toy Car"}',
                    '{"qty":2,"product":"Toy Train"}',
                ], dtype="object"
            ),
            "test_bytea": pd.Series(
                [
                    b'test',
                    b'\xd0\x97\xd0\xb4\xd1\x80\xd0\xb0\xcc\x81\xd0\xb2\xd1\x81\xd1\x82\xd0\xb2\xd1\x83\xd0\xb9\xd1\x82\xd0\xb5',
                    b'123bhaf4',
                    b'\xf0\x9f\x98\x9c'
                ], dtype="object"),
            "test_enum": pd.Series(['happy', 'very happy', 'ecstatic', 'ecstatic'], dtype="object")
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_types_csv(postgres_url: str) -> None:
    query = "SELECT test_int16, test_char, test_uuid, test_time, test_json, test_jsonb, test_bytea, test_enum::text FROM test_types"
    df = read_sql(postgres_url, query, protocol="csv")
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_int16": pd.Series([0, 1, 2, 3], dtype="Int64"),
            "test_char": pd.Series(["a", "b", "c", "d"], dtype="object"),
            "test_uuid": pd.Series(
                [
                    "86b494cc-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49b84-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49c42-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49cce-96b2-11eb-9298-3e22fbb9fe9d"
                ], dtype="object"
            ),
            "test_time": pd.Series(["08:12:40", "10:03:00", "23:00:10", "18:30:00"], dtype="object"),
            "test_json": pd.Series(
                [
                    '{"customer":"John Doe","items":{"product":"Beer","qty":6}}',
                    '{"customer":"Lily Bush","items":{"product":"Diaper","qty":24}}',
                    '{"customer":"Josh William","items":{"product":"Toy Car","qty":1}}',
                    '{"customer":"Mary Clark","items":{"product":"Toy Train","qty":2}}',
                ], dtype="object"
            ),
            "test_jsonb": pd.Series(
                [
                    '{"qty":6,"product":"Beer"}',
                    '{"qty":24,"product":"Diaper"}',
                    '{"qty":1,"product":"Toy Car"}',
                    '{"qty":2,"product":"Toy Train"}',
                ], dtype="object"
            ),
            "test_bytea": pd.Series(
                [
                    b'test',
                    b'\xd0\x97\xd0\xb4\xd1\x80\xd0\xb0\xcc\x81\xd0\xb2\xd1\x81\xd1\x82\xd0\xb2\xd1\x83\xd0\xb9\xd1\x82\xd0\xb5',
                    b'123bhaf4',
                    b'\xf0\x9f\x98\x9c'
                ], dtype="object"),
            "test_enum": pd.Series(['happy', 'very happy', 'ecstatic', 'ecstatic'], dtype="object")
        },
    )
    assert_frame_equal(df, expected, check_names=True)
