import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def bigquery_url() -> str:
    conn = os.environ["BIGQUERY_URL"]
    return conn


@pytest.mark.skipif(
    not os.environ.get("BIGQUERY_URL"),
    reason="Test bigquery only when `BIGQUERY_URL` is set",
)
def test_bigquery_without_partition(bigquery_url: str) -> None:
    query = "select * from `dataprep-bigquery.dataprep.test_table` order by test_int"
    df = read_sql(bigquery_url, query)
    expected = pd.DataFrame(
        index=range(5),
        data={
            "test_int": pd.Series([1, 2, 4, 5, 2333], dtype="Int64"),
            "test_string": pd.Series(
                ["str1", "str2", None, "str05", None], dtype="object"
            ),
            "test_float": pd.Series([1.10, 2.20, -4.44, None, None], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("BIGQUERY_URL"),
    reason="Test bigquery only when `BIGQUERY_URL` is set",
)
def test_bigquery_with_partition(bigquery_url: str) -> None:
    query = "select * from `dataprep-bigquery.dataprep.test_table` order by test_int"
    df = read_sql(
        bigquery_url,
        query,
        partition_on="test_int",
        partition_num=3,
        partition_range=[0, 2500],
    )
    df = df.sort_values("test_int").reset_index(drop=True)
    expected = pd.DataFrame(
        index=range(5),
        data={
            "test_int": pd.Series([1, 2, 4, 5, 2333], dtype="Int64"),
            "test_string": pd.Series(
                ["str1", "str2", None, "str05", None], dtype="object"
            ),
            "test_float": pd.Series([1.10, 2.20, -4.44, None, None], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("BIGQUERY_URL"),
    reason="Test bigquery only when `BIGQUERY_URL` is set",
)
def test_bigquery_with_partition_without_partition_range(bigquery_url: str) -> None:
    query = "select * from `dataprep-bigquery.dataprep.test_table` order by test_int"
    df = read_sql(bigquery_url, query, partition_on="test_int", partition_num=3)
    df = df.sort_values("test_int").reset_index(drop=True)
    expected = pd.DataFrame(
        index=range(5),
        data={
            "test_int": pd.Series([1, 2, 4, 5, 2333], dtype="Int64"),
            "test_string": pd.Series(
                ["str1", "str2", None, "str05", None], dtype="object"
            ),
            "test_float": pd.Series([1.10, 2.20, -4.44, None, None], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("BIGQUERY_URL"),
    reason="Test bigquery only when `BIGQUERY_URL` is set",
)
def test_bigquery_manual_partition(bigquery_url: str) -> None:
    queries = [
        "select * from `dataprep-bigquery.dataprep.test_table` where test_int < 2 order by test_int",
        "select * from `dataprep-bigquery.dataprep.test_table` where test_int >= 2 order by test_int",
    ]
    df = read_sql(bigquery_url, query=queries)
    df = df.sort_values("test_int").reset_index(drop=True)
    expected = pd.DataFrame(
        index=range(5),
        data={
            "test_int": pd.Series([1, 2, 4, 5, 2333], dtype="Int64"),
            "test_string": pd.Series(
                ["str1", "str2", None, "str05", None], dtype="object"
            ),
            "test_float": pd.Series([1.10, 2.20, -4.44, None, None], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("BIGQUERY_URL"),
    reason="Test bigquery only when `BIGQUERY_URL` is set",
)
def test_bigquery_join(bigquery_url: str) -> None:
    query = "SELECT T.test_int, T.test_string, S.test_str FROM `dataprep-bigquery.dataprep.test_table` T INNER JOIN `dataprep-bigquery.dataprep.test_types` S ON T.test_int = S.test_int"
    df = read_sql(
        bigquery_url,
        query,
        partition_on="test_int",
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(2),
        data={
            "test_int": pd.Series([1, 2], dtype="Int64"),
            "test_string": pd.Series(
                [
                    "str1",
                    "str2",
                ],
                dtype="object",
            ),
            "test_str": pd.Series(
                [
                    "😁😂😜",
                    "こんにちはЗдра́в",
                ],
                dtype="object",
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("BIGQUERY_URL"),
    reason="Test bigquery only when `BIGQUERY_URL` is set",
)
def test_bigquery_types(bigquery_url: str) -> None:
    query = "select * from `dataprep-bigquery.dataprep.test_types`"
    df = read_sql(bigquery_url, query)
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    print(df)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, None], dtype="Int64"),
            "test_numeric": pd.Series([1.23, 234.56, None], dtype="float"),
            "test_bool": pd.Series([True, None, False], dtype="boolean"),
            "test_date": pd.Series(
                ["1937-01-28", "2053-07-25", None], dtype="datetime64[ns]"
            ),
            "test_time": pd.Series(["00:00:00", "12:59:59", None], dtype="object"),
            "test_datetime": pd.Series(
                [None, "2053-07-25 12:59:59", "1937-01-28 00:00:00"],
                dtype="datetime64[ns]",
            ),
            "test_timestamp": pd.Series(
                ["1970-01-01 00:00:01.000", None, "2004-02-29 09:00:01.300"],
                dtype="datetime64[ns]",
            ),
            "test_str": pd.Series(["😁😂😜", "こんにちはЗдра́в", None], dtype="object"),
            "test_bytes": pd.Series(
                ["8J+YgfCfmILwn5ic", "44GT44KT44Gr44Gh44Gv0JfQtNGA0LDMgdCy", None],
                dtype="object",
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)
