import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

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
