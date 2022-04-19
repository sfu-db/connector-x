import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql2


@pytest.fixture(scope="module")  # type: ignore
def db1_url() -> str:
    conn = os.environ["DB1"]
    return conn


@pytest.fixture(scope="module")  # type: ignore
def db2_url() -> str:
    conn = os.environ["DB2"]
    return conn


def test_fed(db1_url: str, db2_url: str) -> None:
    query = "select n_name, r_name from db1.nation, db2.region where n_regionkey = r_regionkey and n_nationkey > 3"
    db_map = {"db1": db1_url, "db2": db2_url}
    df = read_sql2(query, db_map)
    print(df)


def test_fed_arrow(db1_url: str, db2_url: str) -> None:
    query = "select n_name, r_name from db1.nation, db2.region where n_regionkey = r_regionkey and n_nationkey > 3"
    db_map = {"db1": db1_url, "db2": db2_url}
    df = read_sql2(query, db_map, return_type="arrow")
    print(df)
