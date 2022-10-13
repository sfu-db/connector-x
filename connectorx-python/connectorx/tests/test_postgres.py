import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

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
            "sum": pd.Series([10.9, 5.2, -10.0], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_partition_on_aggregation(postgres_url: str) -> None:
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


def test_aggregation2(postgres_url: str) -> None:
    query = "select DISTINCT(test_bool) from test_table"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([None, False, True], dtype="boolean"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_partition_on_aggregation2(postgres_url: str) -> None:
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


def test_udf(postgres_url: str) -> None:
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


def test_manual_partition(postgres_url: str) -> None:

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
    query = "SELECT test_date, test_timestamp, test_timestamptz, test_int16, test_int64, test_float32, test_numeric, test_bpchar, test_char, test_varchar, test_uuid, test_time, test_json, test_jsonb, test_bytea, test_enum, test_f4array, test_f8array, test_narray, test_i2array, test_i4array, test_i8array, test_citext FROM test_types"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_date": pd.Series(
                ["1970-01-01", "2000-02-28", "2038-01-18", None], dtype="datetime64[ns]"
            ),
            "test_timestamp": pd.Series(
                [
                    "1970-01-01 00:00:01",
                    "2000-02-28 12:00:10",
                    "2038-01-18 23:59:59",
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_timestamptz": pd.Series(
                [
                    "1970-01-01 00:00:01",
                    "2000-02-28 16:00:10",
                    "2038-01-18 15:59:59",
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_int16": pd.Series([0, 1, 2, 3], dtype="Int64"),
            "test_int64": pd.Series(
                [-9223372036854775808, 0, 9223372036854775807, None], dtype="Int64"
            ),
            "test_float32": pd.Series(
                [None, 3.1415926535, 2.71, -1e-37], dtype="float64"
            ),
            "test_numeric": pd.Series([None, 521.34, 999.99, 0.00], dtype="float64"),
            "test_bpchar": pd.Series(["a    ", "bb   ", "ccc  ", None], dtype="object"),
            "test_char": pd.Series(["a", "b", None, "d"], dtype="object"),
            "test_varchar": pd.Series([None, "bb", "c", "defghijklm"], dtype="object"),
            "test_uuid": pd.Series(
                [
                    "86b494cc-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49b84-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49c42-96b2-11eb-9298-3e22fbb9fe9d",
                    None,
                ],
                dtype="object",
            ),
            "test_time": pd.Series(
                ["08:12:40", None, "23:00:10", "18:30:00"], dtype="object"
            ),
            "test_json": pd.Series(
                [
                    '{"customer":"John Doe","items":{"product":"Beer","qty":6}}',
                    '{"customer":"Lily Bush","items":{"product":"Diaper","qty":24}}',
                    '{"customer":"Josh William","items":{"product":"Toy Car","qty":1}}',
                    None,
                ],
                dtype="object",
            ),
            "test_jsonb": pd.Series(
                [
                    '{"product":"Beer","qty":6}',
                    '{"product":"Diaper","qty":24}',
                    '{"product":"Toy Car","qty":1}',
                    None,
                ],
                dtype="object",
            ),
            "test_bytea": pd.Series(
                [
                    None,
                    b"\xd0\x97\xd0\xb4\xd1\x80\xd0\xb0\xcc\x81\xd0\xb2\xd1\x81\xd1\x82\xd0\xb2\xd1\x83\xd0\xb9\xd1\x82\xd0\xb5",
                    b"",
                    b"\xf0\x9f\x98\x9c",
                ],
                dtype="object",
            ),
            "test_enum": pd.Series(
                ["happy", "very happy", "ecstatic", None], dtype="object"
            ),
            "test_f4array": pd.Series(
                [[], None, [123.123], [-1e-37, 1e37]], dtype="object"
            ),
            "test_f8array": pd.Series(
                [[], None, [-1e-307, 1e308], [0.000234, -12.987654321]], dtype="object"
            ),
            "test_narray": pd.Series(
                [[], None, [521.34], [0.12, 333.33, 22.22]], dtype="object"
            ),
            "test_i2array": pd.Series(
                [[-1, 0, 1], [], [-32768, 32767], None], dtype="object"
            ),
            "test_i4array": pd.Series(
                [[-1, 0, 1123], [], [-2147483648, 2147483647], None], dtype="object"
            ),
            "test_i8array": pd.Series(
                [[-9223372036854775808, 9223372036854775807], [], [0], None],
                dtype="object",
            ),
            "test_citext": pd.Series(["str_citext", "", "s", None], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_types_csv(postgres_url: str) -> None:
    query = "SELECT test_date, test_timestamp, test_timestamptz, test_int16, test_int64, test_float32, test_numeric, test_bpchar, test_char, test_varchar, test_uuid, test_time, test_json, test_jsonb, test_bytea, test_enum::text, test_f4array, test_f8array, test_narray, test_i2array, test_i4array, test_i8array, test_citext FROM test_types"
    df = read_sql(postgres_url, query, protocol="csv")
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_date": pd.Series(
                ["1970-01-01", "2000-02-28", "2038-01-18", None], dtype="datetime64[ns]"
            ),
            "test_timestamp": pd.Series(
                [
                    "1970-01-01 00:00:01",
                    "2000-02-28 12:00:10",
                    "2038-01-18 23:59:59",
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_timestamptz": pd.Series(
                [
                    "1970-01-01 00:00:01",
                    "2000-02-28 16:00:10",
                    "2038-01-18 15:59:59",
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_int16": pd.Series([0, 1, 2, 3], dtype="Int64"),
            "test_int64": pd.Series(
                [-9223372036854775808, 0, 9223372036854775807, None], dtype="Int64"
            ),
            "test_float32": pd.Series(
                [None, 3.1415926535, 2.71, -1e-37], dtype="float64"
            ),
            "test_numeric": pd.Series([None, 521.34, 999.99, 0.00], dtype="float64"),
            "test_bpchar": pd.Series(["a    ", "bb   ", "ccc  ", None], dtype="object"),
            "test_char": pd.Series(["a", "b", None, "d"], dtype="object"),
            "test_varchar": pd.Series([None, "bb", "c", "defghijklm"], dtype="object"),
            "test_uuid": pd.Series(
                [
                    "86b494cc-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49b84-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49c42-96b2-11eb-9298-3e22fbb9fe9d",
                    None,
                ],
                dtype="object",
            ),
            "test_time": pd.Series(
                ["08:12:40", None, "23:00:10", "18:30:00"], dtype="object"
            ),
            "test_json": pd.Series(
                [
                    '{"customer":"John Doe","items":{"product":"Beer","qty":6}}',
                    '{"customer":"Lily Bush","items":{"product":"Diaper","qty":24}}',
                    '{"customer":"Josh William","items":{"product":"Toy Car","qty":1}}',
                    None,
                ],
                dtype="object",
            ),
            "test_jsonb": pd.Series(
                [
                    '{"product":"Beer","qty":6}',
                    '{"product":"Diaper","qty":24}',
                    '{"product":"Toy Car","qty":1}',
                    None,
                ],
                dtype="object",
            ),
            "test_bytea": pd.Series(
                [
                    None,
                    b"\xd0\x97\xd0\xb4\xd1\x80\xd0\xb0\xcc\x81\xd0\xb2\xd1\x81\xd1\x82\xd0\xb2\xd1\x83\xd0\xb9\xd1\x82\xd0\xb5",
                    b"",
                    b"\xf0\x9f\x98\x9c",
                ],
                dtype="object",
            ),
            "test_enum": pd.Series(
                ["happy", "very happy", "ecstatic", None], dtype="object"
            ),
            "test_f4array": pd.Series(
                [[], None, [123.123], [-1e-37, 1e37]], dtype="object"
            ),
            "test_f8array": pd.Series(
                [[], None, [1e-307, 1e308], [0.000234, -12.987654321]], dtype="object"
            ),
            "test_narray": pd.Series(
                [[], None, [521.34], [0.12, 333.33, 22.22]], dtype="object"
            ),
            "test_i2array": pd.Series(
                [[-1, 0, 1], [], [-32768, 32767], None], dtype="object"
            ),
            "test_i4array": pd.Series(
                [[-1, 0, 1123], [], [-2147483648, 2147483647], None], dtype="object"
            ),
            "test_i8array": pd.Series(
                [[-9223372036854775808, 9223372036854775807], [], [0], None],
                dtype="object",
            ),
            "test_citext": pd.Series(["str_citext", None, "s", None], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_postgres_types_cursor(postgres_url: str) -> None:
    query = "SELECT test_date, test_timestamp, test_timestamptz, test_int16, test_int64, test_float32, test_numeric, test_bpchar, test_char, test_varchar, test_uuid, test_time, test_json, test_jsonb, test_bytea, test_enum::text, test_f4array, test_f8array, test_narray, test_i2array, test_i4array, test_i8array, test_citext FROM test_types"
    df = read_sql(postgres_url, query, protocol="cursor")
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_date": pd.Series(
                ["1970-01-01", "2000-02-28", "2038-01-18", None], dtype="datetime64[ns]"
            ),
            "test_timestamp": pd.Series(
                [
                    "1970-01-01 00:00:01",
                    "2000-02-28 12:00:10",
                    "2038-01-18 23:59:59",
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_timestamptz": pd.Series(
                [
                    "1970-01-01 00:00:01",
                    "2000-02-28 16:00:10",
                    "2038-01-18 15:59:59",
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "test_int16": pd.Series([0, 1, 2, 3], dtype="Int64"),
            "test_int64": pd.Series(
                [-9223372036854775808, 0, 9223372036854775807, None], dtype="Int64"
            ),
            "test_float32": pd.Series(
                [None, 3.1415926535, 2.71, -1e-37], dtype="float64"
            ),
            "test_numeric": pd.Series([None, 521.34, 999.99, 0.00], dtype="float64"),
            "test_bpchar": pd.Series(["a    ", "bb   ", "ccc  ", None], dtype="object"),
            "test_char": pd.Series(["a", "b", None, "d"], dtype="object"),
            "test_varchar": pd.Series([None, "bb", "c", "defghijklm"], dtype="object"),
            "test_uuid": pd.Series(
                [
                    "86b494cc-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49b84-96b2-11eb-9298-3e22fbb9fe9d",
                    "86b49c42-96b2-11eb-9298-3e22fbb9fe9d",
                    None,
                ],
                dtype="object",
            ),
            "test_time": pd.Series(
                ["08:12:40", None, "23:00:10", "18:30:00"], dtype="object"
            ),
            "test_json": pd.Series(
                [
                    '{"customer":"John Doe","items":{"product":"Beer","qty":6}}',
                    '{"customer":"Lily Bush","items":{"product":"Diaper","qty":24}}',
                    '{"customer":"Josh William","items":{"product":"Toy Car","qty":1}}',
                    None,
                ],
                dtype="object",
            ),
            "test_jsonb": pd.Series(
                [
                    '{"product":"Beer","qty":6}',
                    '{"product":"Diaper","qty":24}',
                    '{"product":"Toy Car","qty":1}',
                    None,
                ],
                dtype="object",
            ),
            "test_bytea": pd.Series(
                [
                    None,
                    b"\xd0\x97\xd0\xb4\xd1\x80\xd0\xb0\xcc\x81\xd0\xb2\xd1\x81\xd1\x82\xd0\xb2\xd1\x83\xd0\xb9\xd1\x82\xd0\xb5",
                    b"",
                    b"\xf0\x9f\x98\x9c",
                ],
                dtype="object",
            ),
            "test_enum": pd.Series(
                ["happy", "very happy", "ecstatic", None], dtype="object"
            ),
            "test_f4array": pd.Series(
                [[], None, [123.123], [-1e-37, 1e37]], dtype="object"
            ),
            "test_f8array": pd.Series(
                [[], None, [1e-307, 1e308], [0.000234, -12.987654321]], dtype="object"
            ),
            "test_narray": pd.Series(
                [[], None, [521.34], [0.12, 333.33, 22.22]], dtype="object"
            ),
            "test_i2array": pd.Series(
                [[-1, 0, 1], [], [-32768, 32767], None], dtype="object"
            ),
            "test_i4array": pd.Series(
                [[-1, 0, 1123], [], [-2147483648, 2147483647], None], dtype="object"
            ),
            "test_i8array": pd.Series(
                [[-9223372036854775808, 9223372036854775807], [], [0], None],
                dtype="object",
            ),
            "test_citext": pd.Series(["str_citext", "", "s", None], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_empty_result(postgres_url: str) -> None:
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


def test_empty_result_on_partition(postgres_url: str) -> None:
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


def test_empty_result_on_some_partition(postgres_url: str) -> None:
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


def test_posix_regex(postgres_url: str) -> None:
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


def test_json(postgres_url: str) -> None:
    query = "select test_json->>'customer' as customer from test_types"
    df = read_sql(postgres_url, query)
    expected = pd.DataFrame(
        data={
            "customer": pd.Series(
                ["John Doe", "Lily Bush", "Josh William", None], dtype="object"
            ),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_partition_on_json(postgres_url: str) -> None:
    query = "select test_int16, test_jsonb->>'qty' as qty from test_types"
    df = read_sql(postgres_url, query, partition_on="test_int16", partition_num=3)
    expected = pd.DataFrame(
        data={
            "test_int16": pd.Series([0, 1, 2, 3], dtype="Int64"),
            "qty": pd.Series(["6", "24", "1", None], dtype="object"),
        }
    )
    df.sort_values(by="test_int16", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_cte(postgres_url: str) -> None:
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


def test_partition_on_decimal(postgres_url: str) -> None:
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
