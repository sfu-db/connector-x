import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def db1_url() -> str:
    conn = os.environ["DB1"]
    return conn


@pytest.fixture(scope="module")  # type: ignore
def db2_url() -> str:
    conn = os.environ["DB2"]
    return conn


@pytest.mark.skipif(
    not (os.environ.get("DB1") and os.environ.get("DB2") and os.environ.get("FED_CONFIG_PATH")),
    reason="Do not test federated queries is set unless both `FED_CONFIG_PATH`, `DB1` and `DB2` are set",
)
def test_fed_spj(db1_url: str, db2_url: str) -> None:
    query = "SELECT T.test_int, T.test_bool, S.test_language FROM db1.test_table T INNER JOIN db2.test_str S ON T.test_int = S.id"
    df = read_sql({"db1": db1_url, "db2": db2_url}, query)
    expected = pd.DataFrame(
        index=range(5),
        data={
            "TEST_INT": pd.Series([0, 1, 2, 3, 4], dtype="int64"),
            "TEST_BOOL": pd.Series([None, True, False, False, None], dtype="object"),
            "TEST_LANGUAGE": pd.Series(
                ["English", "中文", "日本語", "русский", "Emoji"], dtype="object"
            ),
        },
    )
    df.sort_values(by="TEST_INT", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not (os.environ.get("DB1") and os.environ.get("DB2") and os.environ.get("FED_CONFIG_PATH")),
    reason="Do not test federated queries is set unless both `FED_CONFIG_PATH`, `DB1` and `DB2` are set",
)
def test_fed_spja(db1_url: str, db2_url: str) -> None:
    query = "select test_bool, AVG(test_float) as avg_float, SUM(test_int) as sum_int from db1.test_table as a, db2.test_str as b where a.test_int = b.id AND test_nullint is not NULL GROUP BY test_bool ORDER BY sum_int"
    df = read_sql({"db1": db1_url, "db2": db2_url}, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_bool": pd.Series([True, False, None], dtype="object"),
            "AVG_FLOAT": pd.Series([None, 3, 5.45], dtype="float64"),
            "SUM_INT": pd.Series([1, 3, 4], dtype="int64"),
        },
    )
    df.sort_values(by="SUM_INT", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)

@pytest.mark.skipif(
    not (os.environ.get("DB1") and os.environ.get("DB2") and os.environ.get("FED_CONFIG_PATH")),
    reason="Do not test federated queries is set unless both `FED_CONFIG_PATH`, `DB1` and `DB2` are set",
)
def test_fed_spj_benefit(db1_url: str, db2_url: str) -> None:
    query = "SELECT T.test_int, T.test_bool, S.test_language FROM db1.test_table T INNER JOIN db2.test_str S ON T.test_int = S.id"
    df = read_sql({"db1": db1_url, "db2": db2_url}, query, strategy="benefit")
    expected = pd.DataFrame(
        index=range(5),
        data={
            "TEST_INT": pd.Series([0, 1, 2, 3, 4], dtype="int64"),
            "TEST_BOOL": pd.Series([None, True, False, False, None], dtype="object"),
            "TEST_LANGUAGE": pd.Series(
                ["English", "中文", "日本語", "русский", "Emoji"], dtype="object"
            ),
        },
    )
    df.sort_values(by="TEST_INT", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)