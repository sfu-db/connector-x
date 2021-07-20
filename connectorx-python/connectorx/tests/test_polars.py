import os

import pandas as pd
import pytest
import polars as pl

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def postgres_url() -> str:
    conn = os.environ["POSTGRES_URL"]
    return conn


def test_modin(postgres_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        postgres_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
        return_type="polars",
    )

    expected = pl.DataFrame(
        {
            "test_int": [1, 2, 0, 3, 4, 1314],
            "test_nullint": [3, None, 5, 7, 9, 2],
            "test_str": ["str1", "str2", "a", "b", "c", None],
            "test_float": [None, 2.2, 3.1, 3, 7.8, -10],
            "test_bool": [True, False, None, False, None, True],
        },
    )

    assert df.frame_equal(expected, null_equal=True)
