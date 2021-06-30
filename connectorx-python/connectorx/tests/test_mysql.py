import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def mysql_url() -> str:
    conn = os.environ["MYSQL_URL"]
    return conn


def test_mysql_base(mysql_url: str) -> None:
    query = "select * from test_table"
    df = read_sql(mysql_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 3], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3], dtype="float64")
        }
    )
    assert_frame_equal(df, expected, check_names=True)


def test_mysql_types(mysql_url: str) -> None:
    query = "select * from test_types"
    df = read_sql(mysql_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_date": pd.Series(["1999-07-25", "2020-12-31", "2021-01-28"], dtype="datetime64[ns]"),
            "test_time": pd.Series(["00:00:00", "23:59:59", "12:30:30"], dtype="object"),
            "test_datetime": pd.Series(["1999-07-25 00:00:00", "2020-12-31 23:59:59", "2021-01-28 12:30:30"], dtype="datetime64[ns]"),
            "test_new_decimal": pd.Series([1.1, 2.2, 3.3], dtype="float"),
            "test_decimal": pd.Series([1, 2, 3], dtype="float"),
            "test_varchar": pd.Series(["varchar1", "varchar2", "varchar3"], dtype="object"),
            "test_char": pd.Series(["char1", "char2", "char3"], dtype="object")
        }
    )
    assert_frame_equal(df, expected, check_names=True)
    import time

    conn = os.environ["TPCH_MYSQL_URL"]
    query = "select count(*) from lineitem"
    time_arr = []

    start_cx = time.time()
    df = read_sql(conn, query)
    end_cx = time.time()
    # time_arr.append(end_cx-start_cx)
    print()
    print('-------------------------------------')
    print(df.head())
    print(time_arr)
    print("connector_x: {}s".format(end_cx-start_cx))
    print('-------------------------------------')
