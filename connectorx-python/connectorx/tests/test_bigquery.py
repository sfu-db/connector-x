import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql

@pytest.fixture(scope="module")  # type: ignore
def bigquery_url() -> str:
    conn = os.environ["BIGQUERY_URL"]
    return conn

# def test_bigquery_without_partition(bigquery_url: str) -> None:
#     query = "select * from `dataprep-bigquery.dataprep.test_table` order by test_int"
#     df = read_sql(bigquery_url, query)
#     expected = pd.DataFrame(
#         index=range(5),
#         data={
#             "test_int": pd.Series([1, 2, 4, 5, 2333], dtype="Int64"),
#             "test_string": pd.Series(["str1", "str2", None, "str05", None], dtype="object"),
#             "test_float": pd.Series([1.10, 2.20, -4.44, None, None], dtype="float64"),
#         },
#     )
#     assert_frame_equal(df, expected, check_names=True)


# def test_bigquery_with_partition(bigquery_url: str) -> None:
#     query = "select * from `dataprep-bigquery.dataprep.test_table` order by test_int"
#     df = read_sql(bigquery_url, query, partition_on="test_int", partition_num=3, partition_range=[0,2500])
#     df = df.sort_values("test_int").reset_index(drop=True)
#     expected = pd.DataFrame(
#         index=range(5),
#         data={
#             "test_int": pd.Series([1, 2, 4, 5, 2333], dtype="Int64"),
#             "test_string": pd.Series(["str1", "str2", None, "str05", None], dtype="object"),
#             "test_float": pd.Series([1.10, 2.20, -4.44, None, None], dtype="float64"),
#         },
#     )
#     assert_frame_equal(df, expected, check_names=True)

# def test_bigquery_with_partition_without_partition_range(bigquery_url: str) -> None:
#     query = "select * from `dataprep-bigquery.dataprep.test_table` order by test_int"
#     df = read_sql(bigquery_url, query, partition_on="test_int", partition_num=3)
#     df = df.sort_values("test_int").reset_index(drop=True)
#     expected = pd.DataFrame(
#         index=range(5),
#         data={
#             "test_int": pd.Series([1, 2, 4, 5, 2333], dtype="Int64"),
#             "test_string": pd.Series(["str1", "str2", None, "str05", None], dtype="object"),
#             "test_float": pd.Series([1.10, 2.20, -4.44, None, None], dtype="float64"),
#         },
#     )
#     assert_frame_equal(df, expected, check_names=True)

# def test_bigquery_manual_partition(bigquery_url: str) -> None:
#     queries = [
#         "select * from `dataprep-bigquery.dataprep.test_table` where test_int < 2 order by test_int",
#         "select * from `dataprep-bigquery.dataprep.test_table` where test_int >= 2 order by test_int",
#     ]
#     df = read_sql(bigquery_url, query=queries)
#     df = df.sort_values("test_int").reset_index(drop=True)
#     expected = pd.DataFrame(
#         index=range(5),
#         data={
#             "test_int": pd.Series([1, 2, 4, 5, 2333], dtype="Int64"),
#             "test_string": pd.Series(["str1", "str2", None, "str05", None], dtype="object"),
#             "test_float": pd.Series([1.10, 2.20, -4.44, None, None], dtype="float64"),
#         },
#     )
#     assert_frame_equal(df, expected, check_names=True)

def test_types(bigquery_url: str) -> None:
    query = "select * from `dataprep-bigquery.dataprep.test_types`"
    df = read_sql(bigquery_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([True, None, False], dtype="boolean"),
            "test_date": pd.Series(
                ["1937-01-28", "2053-07-25", None], dtype="datetime64[ns]"),
            "test_time": pd.Series(
                ["00:00:00", None, "12:59:59", None], dtype="object"),
            "test_datetime": pd.Series(
                [None, "2053-07-25 12:59:59", "1937-01-28 00:00:00"], dtype="object"
            ),
            "test_string": pd.Series(["😁😂😜", "こんにちはЗдра́в", None], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)