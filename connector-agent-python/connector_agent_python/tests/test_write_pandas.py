import os

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
        "select * from test_table where test_int < 2",
        "select * from test_table where test_int >= 2",
    ]

    df = write_pandas(postgres_url, queries, True)

    expected = pd.DataFrame(
        index=range(6),
        data={
            "0": pd.Series([1, 0, 2, 3, 4, 1314], dtype="Int64"),
            "1": pd.Series([3, 5, None, 7, 9, 2], dtype="Int64"),
            "2": pd.Series(["str1", "a", "str2", "b", "c", None], dtype="string"),
            "3": pd.Series([None, 3.1, 2.2, 3, 7.8, -10], dtype="float64"),
            "4": pd.Series([True, None, False, False, None, True], dtype="boolean"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.xfail
def test_wrong_dimension(postgres_url: str) -> None:

    queries = [
        "select * from test_table where test_int < 2",
        "select * from test_table where test_int >= 2",
    ]

    schema = ["uint64", "UInt64", "string", "float64"]
    write_pandas(postgres_url, queries, schema, True)
