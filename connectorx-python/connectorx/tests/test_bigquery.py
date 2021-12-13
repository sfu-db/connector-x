import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql

@pytest.fixture(scope="module")  # type: ignore
def bigquery_url() -> str:
    conn = os.environ["BIGQUERY_URL"]
    return conn

def test_bigquery_types(bigquery_url: str) -> None:
    query = "select * from `dataprep-bigquery.dataprep.test_table` order by test_int"
    df = read_sql(bigqueryl_url, query)
    print(df)
    # expected = pd.DataFrame(
    #     index=range(5),
    #     data={
    #         "avg_float": pd.Series([2.2, 2.2], dtype="float64"),
    #         "sum_int": pd.Series([2.0, 4.0], dtype="float64"),
    #         "sum_null": pd.Series([None, None], dtype="float64"),
    #     },
    # )
    # assert_frame_equal(df, expected, check_names=True)