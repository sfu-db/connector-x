import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import get_meta


@pytest.fixture(scope="module")  # type: ignore
def postgres_url() -> str:
    conn = os.environ["POSTGRES_URL"]
    return conn

def test_get_meta(postgres_url: str) -> None:
    query = "SELECT * FROM test_table limit 10"
    df = get_meta(
        postgres_url,
        query,
    )
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([], dtype="Int64"),
            "test_nullint": pd.Series([], dtype="Int64"),
            "test_str": pd.Series(
                [], dtype="object"
            ),
            "test_float": pd.Series([], dtype="float64"),
            "test_bool": pd.Series(
                [], dtype="boolean"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)