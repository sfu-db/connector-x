import os

import numpy as np
import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def redshift_url() -> str:
    conn = os.environ["REDSHIFT_URL"]
    return conn


@pytest.mark.skipif(not os.environ.get("REDSHIFT_URL"), reason="Do not test Redshift unless `REDSHIFT_URL` is set")
def test_redshift_without_partition(redshift_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(redshift_url, query, protocol="cursor")
    # result from redshift might have different order each time
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
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
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(not os.environ.get("REDSHIFT_URL"), reason="Do not test Redshift unless `REDSHIFT_URL` is set")
def test_redshift_with_partition(redshift_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        redshift_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
        protocol="cursor"
    )
    # result from redshift might have different order each time
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
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
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(not os.environ.get("REDSHIFT_URL"), reason="Do not test Redshift unless `REDSHIFT_URL` is set")
def test_redshift_types(redshift_url: str) -> None:
    query = "SELECT test_int16, test_char, test_time, test_datetime FROM test_types"
    df = read_sql(redshift_url, query, protocol="cursor")
    # result from redshift might have different order each time
    df.sort_values(by="test_int16", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_int16": pd.Series([0, 1, 2, 3], dtype="Int64"),
            "test_char": pd.Series(["a", "b", "c", "d"], dtype="object"),
            "test_time": pd.Series(
                ["08:12:40", "10:03:00", "23:00:10", "18:30:00"], dtype="object"
            ),
            "test_datetime": pd.Series(
                [
                    np.datetime64("2007-01-01T10:00:19"),
                    np.datetime64("2005-01-01T22:03:00"),
                    None,
                    np.datetime64("1987-01-01T11:00:00"),
                ], dtype="datetime64[ns]"
            ),

        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(not os.environ.get("REDSHIFT_URL"), reason="Do not test Redshift unless `REDSHIFT_URL` is set")
def test_read_sql_on_utf8(redshift_url: str) -> None:
    query = "SELECT * FROM test_str"
    df = read_sql(redshift_url, query, protocol="cursor")
    # result from redshift might have different order each time
    df.sort_values(by="id", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(8),
        data={
            "id": pd.Series([0, 1, 2, 3, 4, 5, 6, 7], dtype="Int64"),
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
                ],
                dtype="object",
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)
