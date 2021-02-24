import os

import numpy as np
import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import write_pandas


@pytest.fixture(scope="module")  # type: ignore
def postgres_url() -> str:
    conn = os.environ["POSTGRES_URL"]
    return conn


def test_write_pandas(postgres_url: str) -> None:

    queries = [
        "select * from example where id < 1",
        "select * from example where id >= 1",
    ]

    schema = ["uint64", "UInt64", "float64", "string"]
    df = write_pandas(postgres_url, queries, schema)
    expected = pd.DataFrame(
        index=range(4),
        data={
            "0": pd.Series([1, 2, 3, 4], dtype="uint64"),
            "1": pd.Series([0, None, 4, 1314], dtype="UInt64"),
            "2": pd.Series([3.1, 3, float("nan"), -10], dtype="float64"),
            "3": pd.Series(["a", "b", "c", ""], dtype="string"),
        },
    )

    assert_frame_equal(df, expected, check_names=True)
