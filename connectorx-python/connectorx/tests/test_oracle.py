import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql

@pytest.fixture(scope="module")  # type: ignore
def oracle_url() -> str:
    conn = os.environ["ORACLE_URL"]
    return conn

def test_oracle_without_partition(oracle_url: str) -> None:
    query = "select * from test_types"
    df = read_sql(oracle_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "TEST_INT": pd.Series([1, 2, 3], dtype="Int64"),
            "TEST_FLOAT": pd.Series([1.1, 2.2, 3.3], dtype="float64"),
            "TEST_STR": pd.Series(['a1', 'b2', 'c3'], dtype="object")
        }
    )
    assert_frame_equal(df, expected, check_names=True)