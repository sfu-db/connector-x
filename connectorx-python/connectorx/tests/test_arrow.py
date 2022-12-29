import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal
import datetime

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def postgres_url() -> str:
    conn = os.environ["POSTGRES_URL"]
    return conn


def test_arrow(postgres_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        postgres_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
        return_type="arrow",
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="int64"),
            "test_nullint": pd.Series([5, 3, None, 7, 9, 2], dtype="float64"),
            "test_str": pd.Series(
                ["a", "str1", "str2", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([3.1, None, 2.2, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [None, True, False, False, None, True], dtype="object"
            ),
        },
    )

    df = df.to_pandas()
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_arrow2(postgres_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        postgres_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
        return_type="arrow2",
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([0, 1, 2, 3, 4, 1314], dtype="int32"),
            "test_nullint": pd.Series([5, 3, None, 7, 9, 2], dtype="float64"),
            "test_str": pd.Series(
                ["a", "str1", "str2", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([3.1, None, 2.2, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [None, True, False, False, None, True], dtype="object"
            ),
        },
    )

    df = df.to_pandas()
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


def test_arrow2_type(postgres_url: str) -> None:
    query = "SELECT test_date, test_timestamp, test_timestamptz, test_int16, test_int64, test_float32, test_numeric, test_bpchar, test_char, test_varchar, test_uuid, test_time, test_bytea, test_json, test_jsonb, test_f4array, test_f8array, test_narray, test_i2array, test_i4array, test_i8array, test_enum, test_ltree FROM test_types"
    df = read_sql(postgres_url, query, return_type="arrow2")
    df = df.to_pandas(date_as_object=False)
    df.sort_values(by="test_int16", inplace=True, ignore_index=True)
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
                    "1970-01-01 00:00:01+00:00",
                    "2000-02-28 16:00:10+00:00",
                    "2038-01-18 15:59:59+00:00",
                    None,
                ],
                dtype="datetime64[ns, UTC]",
            ),
            "test_int16": pd.Series([0, 1, 2, 3], dtype="int32"),
            "test_int64": pd.Series(
                [-9223372036854775808, 0, 9223372036854775807, None], dtype="float64"
            ),
            "test_float32": pd.Series(
                [None, 3.1415926535, 2.71, -1e-37], dtype="float32"
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
                [
                    datetime.time(8, 12, 40),
                    None,
                    datetime.time(23, 0, 10),
                    datetime.time(18, 30),
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
            "test_enum": pd.Series(
                ["happy", "very happy", "ecstatic", None], dtype="object"
            ),
            "test_ltree": pd.Series(["A.B.C.D", "A.B.E", "A", None], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)
