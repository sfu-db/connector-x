import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def postgres_url() -> str:
    conn = os.environ["POSTGRES_URL"]
    return conn


@pytest.mark.xfail
def test_wrong_partition(postgres_url: str) -> None:
    query = "select * from test_table"
    df = read_sql(conn=postgres_url, query=query, return_type="pandas", partition={'col': 'test_int',
                                                                                   'min': 1,
                                                                                   'max_wrong': 1000,
                                                                                   'num': 3}
                  )


def test_read_sql_without_partition(postgres_url: str) -> None:
    query = "select * from test_table"
    df = read_sql(postgres_url, query, "pandas")
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([3, None, 5, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["str1", "str2", "a", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([None, 2.2, 3.1, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [True, False, None, False, None, True], dtype="boolean"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


def test_read_sql_with_partition(postgres_url: str) -> None:
    query = "select * from test_table"
    df = read_sql(postgres_url, query, "pandas", partition={'col': 'test_int',
                                                            'min': 0,
                                                            'max': 2000,
                                                            'num': 3}
                  )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 0, 3, 4, 1314], dtype="Int64"),
            "test_nullint": pd.Series([3, None, 5, 7, 9, 2], dtype="Int64"),
            "test_str": pd.Series(
                ["str1", "str2", "a", "b", "c", None], dtype="object"
            ),
            "test_float": pd.Series([None, 2.2, 3.1, 3, 7.8, -10], dtype="float64"),
            "test_bool": pd.Series(
                [True, False, None, False, None, True], dtype="boolean"
            ),
        },
    )
    assert_frame_equal(df, expected, check_names=True)