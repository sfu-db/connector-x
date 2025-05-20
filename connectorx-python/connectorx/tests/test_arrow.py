import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal
import datetime
from decimal import localcontext, Decimal

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

def decimal_s10(val):
    return Decimal(val).quantize(Decimal("0.0000000001"))

def test_arrow_type(postgres_url: str) -> None:
    query = "SELECT test_date, test_timestamp, test_timestamptz, test_int2, test_int4, test_int8, test_float4, test_float8, test_numeric, test_bpchar, test_char, test_varchar, test_uuid, test_time, test_bytea, test_json, test_jsonb, test_ltree, test_name FROM test_types"
    df = read_sql(postgres_url, query, return_type="arrow")
    df = df.to_pandas(date_as_object=False)
    df.sort_values(by="test_int2", inplace=True, ignore_index=True)
    with localcontext() as ctx:
        ctx.prec = 38
        expected = pd.DataFrame(
            index=range(5),
            data={
                "test_date": pd.Series(
                    ["1970-01-01", "2000-02-28", "2038-01-18", "1901-12-14", None], dtype="datetime64[ms]"
                ),
                "test_timestamp": pd.Series(
                    [
                        "1970-01-01 00:00:01",
                        "2000-02-28 12:00:10",
                        "2038-01-18 23:59:59",
                        "1901-12-14 00:00:00.062547",
                        None,
                    ],
                    dtype="datetime64[us]",
                ),
                "test_timestamptz": pd.Series(
                    [
                        "1970-01-01 00:00:01+00:00",
                        "2000-02-28 12:00:10-04:00",
                        "2038-01-18 23:59:59+08:00",
                        "1901-12-14 00:00:00.062547-12:00",
                        None,
                    ],
                    dtype="datetime64[us, UTC]",
                ),
                "test_int2": pd.Series([-32768, 0, 1, 32767], dtype="int16"),
                "test_int4": pd.Series([0, 1, -2147483648, 2147483647], dtype="int32"),
                "test_int8": pd.Series(
                    [-9223372036854775808, 0, 9223372036854775807, 1], dtype="float64"
                ),
                "test_float4": pd.Series(
                    [-1.1, 0.00, 2.123456, -12345.1, None], dtype="float32"
                ),
                "test_float8": pd.Series(
                    [-1.1, 0.00, 2.12345678901, -12345678901.1, None], dtype="float64"
                ),
                "test_numeric": pd.Series([decimal_s10(0.01), decimal_s10(521.34), decimal_s10(0), decimal_s10(-1.123e2), None], dtype="object"),
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
                    [
                        datetime.time(8, 12, 40),
                        datetime.time(18, 30),
                        datetime.time(23, 0, 10),
                        datetime.time(0, 0, 59, 62547),
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
                "test_ltree": pd.Series(
                    ["A.B.C.D", "A.B.E", "A", "", None], dtype="object"
                ),
                "test_name": pd.Series(
                    ["0", "21", "someName", "101203203-1212323-22131235", None]
                )
                
            },
        )

    assert_frame_equal(df, expected, check_names=True)