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
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="int32"),
            "test_nullint": pd.Series([3, float("nan"), 5, 7, 9, 2], dtype="float64"),
            "test_str": pd.Series(
                ["str1", "str2", "a", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([None, 2.2, 3.1, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [True, False, None, False, None, True], dtype="object"
            ),
        },
    )

    assert_frame_equal(df.to_pandas(), expected, check_names=True)
