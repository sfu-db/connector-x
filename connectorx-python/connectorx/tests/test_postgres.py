import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal
import datetime
import numpy as np
import ast

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def postgres_url() -> str:
    conn = os.environ["POSTGRES_URL"]
    return conn


@pytest.fixture(scope="module")  # type: ignore
def postgres_url_tls() -> str:
    conn = os.environ["POSTGRES_URL_TLS"]
    return conn


@pytest.fixture(scope="module")  # type: ignore
def postgres_rootcert() -> str:
    cert = os.environ["POSTGRES_ROOTCERT"]
    return cert


@pytest.fixture(scope="module")  # type: ignore
def postgres_sslcert() -> str:
    cert = os.environ["POSTGRES_SSLCERT"]
    return cert


@pytest.fixture(scope="module")  # type: ignore
def postgres_sslkey() -> str:
    key = os.environ["POSTGRES_SSLKEY"]
    return key


@pytest.mark.xfail
def test_postgres_on_non_select(postgres_url: str) -> None:
    query = "CREATE TABLE non_select(id INTEGER NOT NULL)"
    df = read_sql(postgres_url, query)

def test_postgres_aggregation(postgres_url: str) -> None:
    query = "SELECT test_bool, SUM(test_float) FROM test_table GROUP BY test_bool"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([None, False, True], dtype="boolean"),
            "sum": pd.Series([10.9, 5.2, -10.0], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_partition_on_aggregation(postgres_url: str) -> None:
    query = (
        "SELECT test_bool, SUM(test_int) AS test_int FROM test_table GROUP BY test_bool"
    )
    df = read_sql(postgres_url, query, partition_on="test_int", partition_num=2)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([None, False, True], dtype="boolean"),
            "test_int": pd.Series([4, 5, 1315], dtype="Int64"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_aggregation2(postgres_url: str) -> None:
    query = "select DISTINCT(test_bool) from test_table"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([None, False, True], dtype="boolean"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_partition_on_aggregation2(postgres_url: str) -> None:
    query = "select MAX(test_int), MIN(test_int) from test_table"
    df = read_sql(postgres_url, query, partition_on="max", partition_num=2)
    expected = pd.DataFrame(
        index=range(1),
        data={
            "max": pd.Series([1314], dtype="Int64"),
            "min": pd.Series([0], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_udf(postgres_url: str) -> None:
    query = "select increment(test_int) as test_int from test_table ORDER BY test_int"
    df = read_sql(postgres_url, query, partition_on="test_int", partition_num=2)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 1315], dtype="Int64"),
        },
    )
    df = df.sort_values("test_int").reset_index(drop=True)
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_manual_partition(postgres_url: str) -> None:

    queries = [
        "SELECT * FROM test_table WHERE test_int < 2",
        "SELECT * FROM test_table WHERE test_int >= 2",
    ]

    df = read_sql(postgres_url, query=queries)

    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
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


def test_postgres_without_partition(postgres_url: str) -> None:
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


def test_postgres_limit(postgres_url: str) -> None:
    query = "SELECT * FROM test_table limit 3"
    df = read_sql(
        postgres_url,
        query,
    )
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([0, 1, 2], dtype="Int64"),
            "test_nullint": pd.Series([5, 3, None], dtype="Int64"),
            "test_str": pd.Series(["a", "str1", "str2"], dtype="object"),
            "test_float": pd.Series([3.1, None, 2.2], dtype="float64"),
            "test_bool": pd.Series([None, True, False], dtype="boolean"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_limit_large(postgres_url: str) -> None:
    query = "SELECT * FROM test_table limit 10"
    df = read_sql(
        postgres_url,
        query,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
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


def test_postgres_limit_with_partition(postgres_url: str) -> None:
    query = "SELECT * FROM test_table limit 3"
    df = read_sql(
        postgres_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([0, 1, 2], dtype="Int64"),
            "test_nullint": pd.Series([5, 3, None], dtype="Int64"),
            "test_str": pd.Series(["a", "str1", "str2"], dtype="object"),
            "test_float": pd.Series([3.1, None, 2.2], dtype="float64"),
            "test_bool": pd.Series([None, True, False], dtype="boolean"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_limit_large_with_partition(postgres_url: str) -> None:
    query = "SELECT * FROM test_table limit 10"
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
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
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


def test_postgres_with_partition(postgres_url: str) -> None:
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
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
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


def test_postgres_with_partition_without_partition_range(postgres_url: str) -> None:
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
            "test_str": pd.Series(["a", "c"], dtype="object"),
            "test_float": pd.Series([3.1, 7.8], dtype="float64"),
            "test_bool": pd.Series([None, None], dtype="boolean"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_with_partition_and_selection(postgres_url: str) -> None:
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
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
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


def test_postgres_with_partition_and_projection(postgres_url: str) -> None:
    query = "SELECT test_int, test_nullint, test_str FROM test_table"
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
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([5, 3, None, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["a", "str1", "str2", "b", "c", None], dtype="object"
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_with_partition_and_join(postgres_url: str) -> None:
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
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_with_partition_and_spja(postgres_url: str) -> None:
    query = "select test_bool, AVG(test_float) as avg, SUM(test_int) as sum from test_table as a, test_str as b where a.test_int = b.id AND test_nullint is not NULL GROUP BY test_bool ORDER BY sum"
    df = read_sql(postgres_url, query, partition_on="sum", partition_num=2)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([True, False, None], dtype="boolean"),
            "avg": pd.Series([None, 3, 5.45], dtype="float64"),
            "sum": pd.Series([1, 3, 4], dtype="Int64"),
        },
    )
    df.sort_values(by="sum", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_on_utf8(postgres_url: str) -> None:
    query = "SELECT * FROM test_str"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        index=range(9),
        data={
            "id": pd.Series([0, 1, 2, 3, 4, 5, 6, 7, 8], dtype="Int64"),
            "test_language": pd.Series(
                [
                    "English",
                    "中文",
                    "日本語",
                    "русский",
                    "Emoji",
                    "Latin1",
                    "Extra",
                    "Mixed",
                    "",
                ],
                dtype="object",
            ),
            "test_hello": pd.Series(
                [
                    "Hello",
                    "你好",
                    "こんにちは",
                    "Здра́вствуйте",
                    "😁😂😜",
                    "¥§¤®ð",
                    "y̆",
                    "Ha好ち😁ðy̆",
                    None,
                ],
                dtype="object",
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_with_index_col(postgres_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(postgres_url, query, index_col="test_int")
    expected = pd.DataFrame(
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
    expected.set_index("test_int", inplace=True)
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_types_binary(postgres_url: str) -> None:
    query = "SELECT test_date, test_timestamp, test_timestamptz, test_int2, test_int4, test_int8, test_float4, test_float8, test_numeric, test_bpchar, test_char, test_varchar, test_uuid, test_time, test_json, test_jsonb, test_bytea, test_enum, test_citext, test_ltree, test_lquery, test_ltxtquery, test_name FROM test_types"
    df = read_sql(postgres_url, query)
    verify_data_types(df, "binary")

def test_postgres_types_vec_binary(postgres_url: str) -> None:
    query = "SELECT test_boolarray, test_i2array, test_i4array, test_i8array, test_f4array, test_f8array, test_narray FROM test_types where test_int2 is not NULL and test_int2 <> 32767"
    df = read_sql(postgres_url, query)
    verify_data_types_vec(df)

def test_postgres_types_csv(postgres_url: str) -> None:
    query = "SELECT test_date, test_timestamp, test_timestamptz, test_int2, test_int4, test_int8, test_float4, test_float8, test_numeric, test_bpchar, test_char, test_varchar, test_uuid, test_time, test_json, test_jsonb, test_bytea, test_enum, test_citext, test_ltree, test_lquery, test_ltxtquery, test_name FROM test_types"
    df = read_sql(postgres_url, query, protocol="csv")
    verify_data_types(df, "csv")

def test_postgres_types_vec_csv(postgres_url: str) -> None:
    query = "SELECT test_boolarray, test_i2array, test_i4array, test_i8array, test_f4array, test_f8array, test_narray FROM test_types where test_int2 is not NULL and test_int2 <> 32767"
    df = read_sql(postgres_url, query, protocol="csv")
    verify_data_types_vec(df)

def test_postgres_types_cursor(postgres_url: str) -> None:
    query = "SELECT test_date, test_timestamp, test_timestamptz, test_int2, test_int4, test_int8, test_float4, test_float8, test_numeric, test_bpchar, test_char, test_varchar, test_uuid, test_time, test_json, test_jsonb, test_bytea, test_citext, test_ltree, test_lquery, test_ltxtquery, test_name FROM test_types"
    df = read_sql(postgres_url, query, protocol="cursor")
    verify_data_types(df, "cursor")

def test_postgres_types_vec_cursor(postgres_url: str) -> None:
    query = "SELECT test_boolarray, test_i2array, test_i4array, test_i8array, test_f4array, test_f8array, test_narray FROM test_types where test_int2 is not NULL and test_int2 <> 32767"
    df = read_sql(postgres_url, query, protocol="cursor")
    verify_data_types_vec(df)

def test_postgres_types_simple(postgres_url: str) -> None:
    query = "SELECT test_date, test_timestamp, test_timestamptz, test_int2, test_int4, test_int8, test_float4, test_float8, test_numeric, test_bpchar, test_char, test_varchar, test_uuid, test_time, test_bytea, test_citext, test_ltree, test_lquery, test_ltxtquery, test_name FROM test_types"
    df = read_sql(postgres_url, query, protocol="simple")
    verify_data_types(df, "simple")

def test_postgres_types_vec_simple(postgres_url: str) -> None:
    query = "SELECT test_boolarray, test_i2array, test_i4array, test_i8array, test_f4array, test_f8array, test_narray FROM test_types where test_int2 is not NULL and test_int2 <> 32767"
    df = read_sql(postgres_url, query, protocol="simple")
    verify_data_types_vec(df)

def verify_data_types(df, protocol) -> None:
    expected = pd.DataFrame(
        index=range(5),
        data={
            "test_date": pd.Series(
                ["1970-01-01", "2000-02-28", "2038-01-18", "1901-12-14", None], dtype="datetime64[ns]"
            ),
            "test_timestamp": pd.Series(
                [
                    "1970-01-01 00:00:01",
                    "2000-02-28 12:00:10",
                    "2038-01-18 23:59:59",
                    "1901-12-14 00:00:00.062547",
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_timestamptz": pd.Series(
                [
                    "1970-01-01 00:00:01",
                    "2000-02-28 16:00:10",
                    "2038-01-18 15:59:59",
                    "1901-12-14 12:00:00.062547",
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_int2": pd.Series([-32768, 0, 1, 32767], dtype="Int64"),
            "test_int4": pd.Series([0, 1, -2147483648, 2147483647], dtype="Int64"),
            "test_int8": pd.Series(
                [-9223372036854775808, 0, 9223372036854775807, 1], dtype="Int64"
            ),
            "test_float4": pd.Series(
                [-1.1, 0.00, 2.123456, -12345.1, None], dtype="float64"
            ),
            "test_float8": pd.Series(
                [-1.1, 0.00, 2.12345678901, -12345678901.1, None], dtype="float64"
            ),
            "test_numeric": pd.Series([0.01, 521.34, 0, -1.123e2, None], dtype="float64"),
            "test_bpchar": pd.Series(["👨‍🍳  ", "bb   ", "     ", "ddddd", None], dtype="object"),
            "test_char": pd.Series(["a", "ಠ", "😃", "@", None], dtype="object"),
            "test_varchar": pd.Series(["abcdefghij", "", "👨‍🍳👨‍🍳👨‍🍳👨", "@", None], dtype="object"),
            "test_uuid": pd.Series(
                [
                    "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11",
                    "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11",
                    "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11",
                    "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11",
                    None,
                ],
                dtype="object",
            ),
            "test_time": pd.Series(
                ["08:12:40", "18:30:00", "23:00:10", "00:00:59.062547", None], dtype="object"
            ),
            "test_json": pd.Series(
                [
                    '{"customer":"John Doe","items":{"product":"Beer","qty":6}}',
                    '{"customer":"Lily Bush","items":{"product":"Diaper","qty":24}}',
                    '{"customer":"Josh William","items":{"product":"Toy Car","qty":1}}',
                    '{}',
                    None,
                ],
                dtype="object",
            ),
            "test_jsonb": pd.Series(
                [
                    '{"customer":"John Doe","items":{"product":"Beer","qty":6}}',
                    '{"customer":"Lily Bush","items":{"product":"Diaper","qty":24}}',
                    '{"customer":"Josh William","items":{"product":"Toy Car","qty":1}}',
                    '{}',
                    None,
                ],
                dtype="object",
            ),
            "test_bytea": pd.Series(
                [
                    b'\x08',
                    b"\xd0\x97\xd0\xb4\xd1\x80\xd0\xb0\xcc\x81\xd0\xb2\xd1\x81\xd1\x82\xd0\xb2\xd1\x83\xd0\xb9\xd1\x82\xd0\xb5",
                    b"",
                    b"\xf0\x9f\x98\x9c",
                    None
                ],
                dtype="object",
            ),
            "test_enum": pd.Series(
                ["happy", "very happy", "ecstatic", "ecstatic", None]
            ),
            "test_citext": pd.Series(
                ["str_citext", "", "abcdef", "1234", None]
            ),
            "test_ltree": pd.Series(
                ["A.B.C.D", "A.B.E", "A", "", None], dtype="object"
            ),
            "test_lquery": pd.Series(
                ["*.B.*", "A.*", "*", "*.A", None]
            ),
            "test_ltxtquery": pd.Series(
                ["A & B*", "A | B", "A@", "A & B*", None]
            ),
            "test_name": pd.Series(
                ["0", "21", "someName", "101203203-1212323-22131235", None]
            ),
        },
    )

    # Unimplemented
    if protocol == "cursor" or protocol == "simple": 
        expected = expected.drop(columns=['test_enum'])

    # Unimplemented
    if protocol == "simple": 
        expected = expected.drop(columns=['test_json', 'test_jsonb'])

    # CSV protocol can't distinguish "" from NULL
    if protocol == "csv":
        expected.loc[expected.test_varchar == "", 'test_varchar'] = None
        expected.loc[expected.test_citext == "", 'test_citext'] = None
        expected.loc[expected.test_ltree == "", 'test_ltree'] = None

    assert_frame_equal(df, expected, check_names=True)

def verify_data_types_vec(df) -> None:
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_boolarray": pd.Series(
                [[True, False], [], [True]], dtype="object"
            ),
            "test_i2array": pd.Series(
                [[12], [], [-32768, 32767]], dtype="object"
            ),
            "test_i4array": pd.Series(
                [[-1], [], [-2147483648, 2147483647]], dtype="object"
            ),
            "test_i8array": pd.Series(
                [[-9223372036854775808, 9223372036854775807], [], [0]],
                dtype="object",
            ),
            "test_f4array": pd.Series(
                [[-1.1, 0.00], [], [1, -2, -12345.1]], dtype="object"
            ),
            "test_f8array": pd.Series(
                [[-1.1, 0.00], [], [2.12345678901, -12345678901.1]], dtype="object"
            ),
            "test_narray": pd.Series(
                [[0.01, 521.23], [0.12, 333.33, 22.22], []], dtype="object"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)

def test_postgres_empty_result(postgres_url: str) -> None:
    query = "SELECT * FROM test_table where test_int < -100"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([], dtype="Int64"),
            "test_nullint": pd.Series([], dtype="Int64"),
            "test_str": pd.Series([], dtype="object"),
            "test_float": pd.Series([], dtype="float64"),
            "test_bool": pd.Series([], dtype="boolean"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_empty_result_on_partition(postgres_url: str) -> None:
    query = "SELECT * FROM test_table where test_int < -100"
    df = read_sql(postgres_url, query, partition_on="test_int", partition_num=3)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([], dtype="Int64"),
            "test_nullint": pd.Series([], dtype="Int64"),
            "test_str": pd.Series([], dtype="object"),
            "test_float": pd.Series([], dtype="float64"),
            "test_bool": pd.Series([], dtype="boolean"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_empty_result_on_some_partition(postgres_url: str) -> None:
    query = "SELECT * FROM test_table where test_int < 1"
    df = read_sql(postgres_url, query, partition_on="test_int", partition_num=3)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([0], dtype="Int64"),
            "test_nullint": pd.Series([5], dtype="Int64"),
            "test_str": pd.Series(["a"], dtype="object"),
            "test_float": pd.Series([3.1], dtype="float64"),
            "test_bool": pd.Series([None], dtype="boolean"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_posix_regex(postgres_url: str) -> None:
    query = "select test_int, case when test_str ~* 'str.*' then 'convert_str' end as converted_str from test_table"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="Int64"),
            "converted_str": pd.Series(
                ["convert_str", "convert_str", None, None, None, None], dtype="object"
            ),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_json(postgres_url: str) -> None:
    query = "select test_json->>'customer' as customer from test_types"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        data={
            "customer": pd.Series(
                ["John Doe", "Lily Bush", "Josh William", None, None], dtype="object"
            ),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_partition_on_json(postgres_url: str) -> None:
    query = "select test_int2, test_jsonb->'items'->>'qty' as qty from test_types where test_int2 is not NULL"
    df = read_sql(postgres_url, query, partition_on='test_int2', partition_num=3)
    expected = pd.DataFrame(
        data={
            "test_int2": pd.Series([-32768, 0, 1, 32767], dtype="Int64"),
            "qty": pd.Series(["6", "24", "1", None], dtype="object"),
        }
    )
    df.sort_values(by="test_int2", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_cte(postgres_url: str) -> None:
    query = "with test_cte (test_int, test_str) as (select test_int, test_str from test_table where test_float > 0) select test_int, test_str from test_cte"
    df = read_sql(postgres_url, query, partition_on="test_int", partition_num=3)
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_int": pd.Series([0, 2, 3, 4], dtype="Int64"),
            "test_str": pd.Series(["a", "str2", "b", "c"], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("POSTGRES_URL_TLS"),
    reason="Do not test Postgres TLS unless `POSTGRES_URL_TLS` is set",
)
def test_postgres_tls(postgres_url_tls: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        f"{postgres_url_tls}?sslmode=require",
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
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


def test_postgres_partition_on_decimal(postgres_url: str) -> None:
    # partition column can not have None
    query = "SELECT * FROM test_table where test_int<>1"
    df = read_sql(postgres_url, query, partition_on="test_float", partition_num=3)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([0, 2, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([5, None, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(["a", "str2", "b", "c", None], dtype="object"),
            "test_float": pd.Series([3.1, 2.2, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series([None, False, False, None, True], dtype="boolean"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("POSTGRES_URL_TLS"),
    reason="Do not test Postgres TLS unless `POSTGRES_URL_TLS` is set",
)
def test_postgres_tls_with_cert(postgres_url_tls: str, postgres_rootcert: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        f"{postgres_url_tls}?sslmode=require&sslrootcert={postgres_rootcert}",
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
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


@pytest.mark.skipif(
    not os.environ.get("POSTGRES_URL_TLS"),
    reason="Do not test Postgres TLS unless `POSTGRES_URL_TLS` is set",
)
def test_postgres_tls_client_auth(
    postgres_url_tls: str,
    postgres_rootcert: str,
    postgres_sslcert: str,
    postgres_sslkey: str,
) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        f"{postgres_url_tls}?sslmode=require&sslrootcert={postgres_rootcert}&sslcert={postgres_sslcert}&sslkey={postgres_sslkey}",
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
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


@pytest.mark.skipif(
    not os.environ.get("POSTGRES_URL_TLS"),
    reason="Do not test Postgres TLS unless `POSTGRES_URL_TLS` is set",
)
def test_postgres_tls_disable(postgres_url_tls: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        f"{postgres_url_tls}?sslmode=disable",
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
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


@pytest.mark.skipif(
    not os.environ.get("POSTGRES_URL_TLS"),
    reason="Do not test Postgres TLS unless `POSTGRES_URL_TLS` is set",
)
@pytest.mark.xfail
def test_postgres_tls_fail(postgres_url_tls: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        f"{postgres_url_tls}?sslmode=require&sslrootcert=fake.cert",
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )

def test_postgres_name_type(postgres_url: str) -> None:
    query = "SELECT test_name FROM test_types"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        data={
            "test_name": pd.Series(["0", "21", "someName", "101203203-1212323-22131235", None]),
        },
    )
    assert_frame_equal(df, expected, check_names=True)

def test_postgres_semicolon_support_str_query(postgres_url: str) -> None:
    query = "SELECT test_name FROM test_types;"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        data={
            "test_name": pd.Series(["0", "21", "someName", "101203203-1212323-22131235", None]),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_semicolon_list_queries(postgres_url: str) -> None:
    queries = [
        "SELECT * FROM test_table WHERE test_int < 2;",
        "SELECT * FROM test_table WHERE test_int >= 2;",
    ]

    df = read_sql(postgres_url, query=queries)

    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
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

def test_postgres_partition_with_orderby(postgres_url: str) -> None:
    query = "select * from test_table order by test_int"
    df = read_sql(postgres_url, query=query, partition_on="test_int", partition_num=2)

    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="Int64"),
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

def test_postgres_partition_with_orderby_limit_asc(postgres_url: str) -> None:
    query = "select * from test_table order by test_int asc limit 2"
    df = read_sql(postgres_url, query=query, partition_on="test_int", partition_num=2)

    expected = pd.DataFrame(
        index=range(2),
        data={
            "test_int": pd.Series([0, 1], dtype="Int64"),
            "test_nullint": pd.Series([5, 3], dtype="Int64"),
            "test_str": pd.Series(
                ["a", "str1"], dtype="object"
            ),
            "test_float": pd.Series([3.1, None], dtype="float64"),
            "test_bool": pd.Series(
                [None, True], dtype="boolean"
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)

def test_postgres_partition_with_orderby_limit_desc(postgres_url: str) -> None:
    query = "select * from test_table order by test_int desc limit 2"
    df = read_sql(postgres_url, query=query, partition_on="test_int", partition_num=2)

    expected = pd.DataFrame(
        index=range(2),
        data={
            "test_int": pd.Series([4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["c", None], dtype="object"
            ),
            "test_float": pd.Series([7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [None, True], dtype="boolean"
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)

def test_postgres_single_pre_execution_queries(postgres_url: str) -> None:
    pre_execution_query = "SET SESSION statement_timeout = 2151"
    query = "SELECT CAST(name AS TEXT) AS name, CAST(setting AS INTEGER) FROM pg_settings WHERE name = 'statement_timeout'"
    df = read_sql(postgres_url, query, pre_execution_query=pre_execution_query)
    expected = pd.DataFrame(
        index=range(1),
        data={
            "name": pd.Series(["statement_timeout"], dtype="str"),
            "setting": pd.Series([2151], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)

def test_postgres_multiple_pre_execution_queries(postgres_url: str) -> None:
    pre_execution_query = [
        "SET SESSION statement_timeout = 2151",
        "SET SESSION idle_in_transaction_session_timeout = 2252",
    ]
    query = "SELECT CAST(name AS TEXT) AS name, CAST(setting AS INTEGER) FROM pg_settings WHERE name IN ('statement_timeout', 'idle_in_transaction_session_timeout') ORDER BY name"
    df = read_sql(postgres_url, query, pre_execution_query=pre_execution_query)
    expected = pd.DataFrame(
        index=range(2),
        data={
            "name": pd.Series(["idle_in_transaction_session_timeout", "statement_timeout"], dtype="str"),
            "setting": pd.Series([2252, 2151], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)

def test_postgres_partitioned_pre_execution_queries(postgres_url: str) -> None:
    pre_execution_query = [
        "SET SESSION statement_timeout = 2151",
        "SET SESSION idle_in_transaction_session_timeout = 2252",
    ]
    query = [
        "SELECT CAST(name AS TEXT) AS name, CAST(setting AS INTEGER) AS setting FROM pg_settings WHERE name = 'statement_timeout'", 
        "SELECT CAST(name AS TEXT) AS name, CAST(setting AS INTEGER) AS setting FROM pg_settings WHERE name = 'idle_in_transaction_session_timeout'"
    ]

    df = read_sql(postgres_url, query, pre_execution_query=pre_execution_query).sort_values(by=['name']).reset_index(drop=True)
    expected = pd.DataFrame(
        index=range(2),
        data={
            "name": pd.Series(["statement_timeout", "idle_in_transaction_session_timeout"], dtype="str"),
            "setting": pd.Series([2151, 2252], dtype="Int64"),
        },
    ).sort_values(by=['name']).reset_index(drop=True)
    assert_frame_equal(df, expected, check_names=True)

def test_postgres_inet_type(postgres_url: str) -> None:
    query = "SELECT test_inet FROM test_types"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        data={
            "test_inet": pd.Series(
                ["192.168.1.1", "10.0.0.0/24", "2001:db8::1", "2001:db8::/32", None],
                dtype="object"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)

def test_postgres_vector_types(postgres_url: str) -> None:
    query = "SELECT dense_vector, half_vector, binary_vector, sparse_vector FROM vector_types"
    df = read_sql(postgres_url, query)
    
    # Parse string vectors into numpy arrays
    def parse_vector(vec_str):
        if vec_str is None:
            return None
        # Handle both string and list inputs
        if isinstance(vec_str, str):
            # Remove brackets and split string
            vec_str = vec_str.strip('[]')
            return np.array([float(x) for x in vec_str.split(',')])
        elif isinstance(vec_str, list):
            return np.array([float(x) for x in vec_str])
        else:
            raise TypeError(f"Unexpected type for vector: {type(vec_str)}")
    
    # Convert dense_vector and half_vector to numpy arrays
    df['dense_vector'] = df['dense_vector'].apply(parse_vector)
    df['half_vector'] = df['half_vector'].apply(parse_vector)
    
    # Verify dense_vector
    expected_dense = np.array([0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0])
    assert df['dense_vector'].iloc[0] is not None
    assert np.allclose(df['dense_vector'].iloc[0], expected_dense, rtol=1e-5)
    assert df['dense_vector'].iloc[1] is None
    
    # Verify half_vector
    expected_half = np.array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0])
    assert df['half_vector'].iloc[0] is not None
    assert np.allclose(df['half_vector'].iloc[0], expected_half, rtol=1e-5)
    assert df['half_vector'].iloc[1] is None
    
    # Verify binary_vector and sparse_vector
    # Convert binary_vector to string representation for comparison
    def binary_to_string(binary):
        if binary is None:
            return None
        # Convert binary to string of 1s and 0s
        return ''.join(format(b, '08b') for b in binary)[:10]  # Take first 10 bits
    
    df['binary_vector'] = df['binary_vector'].apply(binary_to_string)
    
    # Convert sparse vector array to string format
    def sparse_to_string(sparse_vec):
        if sparse_vec is None:
            return None
        # Convert array to sparse format string with integer values
        non_zero = {i+1: int(val) for i, val in enumerate(sparse_vec) if val != 0}
        return f"{non_zero}/{len(sparse_vec)}"
    
    df['sparse_vector'] = df['sparse_vector'].apply(sparse_to_string)
    
    expected = pd.DataFrame(
        data={
            "binary_vector": pd.Series(
                ["1010101010", None],
                dtype="object"
            ),
            "sparse_vector": pd.Series(
                ["{1: 1, 3: 2, 5: 3}/5", None],
                dtype="object"
            ),
        },
    )
    assert_frame_equal(df[['binary_vector', 'sparse_vector']], expected[['binary_vector', 'sparse_vector']], check_names=True)
