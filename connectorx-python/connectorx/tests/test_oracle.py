import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def oracle_url() -> str:
    conn = os.environ["ORACLE_URL"]
    return conn


@pytest.mark.xfail
@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_on_non_select(oracle_url: str) -> None:
    query = "CREATE TABLE non_select(id INTEGER NOT NULL)"
    df = read_sql(oracle_url, query)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_aggregation(oracle_url: str) -> None:
    query = "select avg(test_int), test_char from test_table group by test_char"
    df = read_sql(oracle_url, query)
    df = df.sort_values("AVG(TEST_INT)").reset_index(drop=True)
    expected = pd.DataFrame(
        data={
            "AVG(TEST_INT)": pd.Series([1, 2, 5, 1168.5], dtype="float64"),
            "TEST_CHAR": pd.Series(["str1 ", "str2 ", "str05", None], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_partition_on_aggregation(oracle_url: str) -> None:
    query = "select sum(test_int) cid, test_char from test_table group by test_char"
    df = read_sql(oracle_url, query, partition_on="cid", partition_num=3)
    df = df.sort_values("CID").reset_index(drop=True)
    expected = pd.DataFrame(
        index=range(4),
        data={
            "CID": pd.Series([1, 2, 5, 2337], dtype="float64"),
            "TEST_CHAR": pd.Series(["str1 ", "str2 ", "str05", None], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_aggregation2(oracle_url: str) -> None:
    query = "select DISTINCT(test_char) from test_table"
    df = read_sql(oracle_url, query)
    expected = pd.DataFrame(
        data={
            "TEST_CHAR": pd.Series(["str2 ", "str05", None, "str1 "], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_partition_on_aggregation2(oracle_url: str) -> None:
    query = "select MAX(test_int) MAX, MIN(test_int) MIN from test_table"
    df = read_sql(oracle_url, query, partition_on="MAX", partition_num=2)
    expected = pd.DataFrame(
        index=range(1),
        data={
            "MAX": pd.Series([2333], dtype="float64"),
            "MIN": pd.Series([1], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_manual_partition(oracle_url: str) -> None:
    queries = [
        "SELECT * FROM test_table WHERE test_int < 2",
        "SELECT * FROM test_table WHERE test_int >= 2",
    ]
    df = read_sql(oracle_url, query=queries)
    expected = pd.DataFrame(
        data={
            "TEST_INT": pd.Series([1, 2, 2333, 4, 5], dtype="Int64"),
            "TEST_CHAR": pd.Series(
                ["str1 ", "str2 ", None, None, "str05"], dtype="object"
            ),
            "TEST_FLOAT": pd.Series([1.1, 2.2, None, -4.44, None], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_without_partition(oracle_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(oracle_url, query)
    expected = pd.DataFrame(
        data={
            "TEST_INT": pd.Series([1, 2, 2333, 4, 5], dtype="Int64"),
            "TEST_CHAR": pd.Series(
                ["str1 ", "str2 ", None, None, "str05"], dtype="object"
            ),
            "TEST_FLOAT": pd.Series([1.1, 2.2, None, -4.44, None], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_limit_without_partition(oracle_url: str) -> None:
    query = "SELECT * FROM test_table where rownum <= 3"
    df = read_sql(oracle_url, query)
    expected = pd.DataFrame(
        data={
            "TEST_INT": pd.Series([1, 2, 2333], dtype="Int64"),
            "TEST_CHAR": pd.Series(["str1 ", "str2 ", None], dtype="object"),
            "TEST_FLOAT": pd.Series([1.1, 2.2, None], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_limit_large_without_partition(oracle_url: str) -> None:
    query = "SELECT * FROM test_table where rownum < 10"
    df = read_sql(oracle_url, query)
    expected = pd.DataFrame(
        data={
            "TEST_INT": pd.Series([1, 2, 2333, 4, 5], dtype="Int64"),
            "TEST_CHAR": pd.Series(
                ["str1 ", "str2 ", None, None, "str05"], dtype="object"
            ),
            "TEST_FLOAT": pd.Series([1.1, 2.2, None, -4.44, None], dtype="float64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_with_partition(oracle_url: str) -> None:
    query = "SELECT * FROM test_table"
    df = read_sql(
        oracle_url,
        query,
        partition_on="test_int",
        partition_range=(0, 5001),
        partition_num=3,
    )
    expected = pd.DataFrame(
        data={
            "TEST_INT": pd.Series([1, 2, 4, 5, 2333], dtype="Int64"),
            "TEST_CHAR": pd.Series(
                ["str1 ", "str2 ", None, "str05", None], dtype="object"
            ),
            "TEST_FLOAT": pd.Series([1.1, 2.2, -4.44, None, None], dtype="float64"),
        },
    )
    df.sort_values(by="TEST_INT", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_with_partition_without_partition_range(oracle_url: str) -> None:
    query = "SELECT * FROM test_table where test_float > 1"
    df = read_sql(
        oracle_url,
        query,
        partition_on="test_int",
        partition_num=3,
    )
    expected = pd.DataFrame(
        data={
            "TEST_INT": pd.Series([1, 2], dtype="Int64"),
            "TEST_CHAR": pd.Series(["str1 ", "str2 "], dtype="object"),
            "TEST_FLOAT": pd.Series([1.1, 2.2], dtype="float64"),
        },
    )
    df.sort_values(by="TEST_INT", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_with_partition_and_selection(oracle_url: str) -> None:
    query = "SELECT * FROM test_table WHERE 1 = 3 OR 2 = 2"
    df = read_sql(
        oracle_url,
        query,
        partition_on="test_int",
        partition_range=(1, 2333),
        partition_num=3,
    )
    expected = pd.DataFrame(
        data={
            "TEST_INT": pd.Series([1, 2, 4, 5, 2333], dtype="Int64"),
            "TEST_CHAR": pd.Series(
                ["str1 ", "str2 ", None, "str05", None], dtype="object"
            ),
            "TEST_FLOAT": pd.Series([1.1, 2.2, -4.44, None, None], dtype="float64"),
        },
    )
    df.sort_values(by="TEST_INT", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_with_partition_and_spja(oracle_url: str) -> None:
    query = "select test_table.test_int cid, SUM(test_types.test_num_float) sfloat from test_table, test_types where test_table.test_int=test_types.test_num_int group by test_table.test_int;"
    df = read_sql(oracle_url, query, partition_on="cid", partition_num=2)
    expected = pd.DataFrame(
        data={
            "CID": pd.Series([1, 5], dtype="Int64"),
            "SFLOAT": pd.Series([2.3, -0.2], dtype="float64"),
        },
    )
    df.sort_values(by="CID", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_types(oracle_url: str) -> None:
    query = "SELECT * FROM test_types"
    df = read_sql(oracle_url, query)
    expected = pd.DataFrame(
        data={
            "TEST_NUM_INT": pd.Series([1, 5, 5, None], dtype="Int64"),
            "TEST_INT": pd.Series([-10, 22, 22, 100], dtype="Int64"),
            "TEST_NUM_FLOAT": pd.Series([2.3, -0.1, -0.1, None], dtype="float64"),
            "TEST_FLOAT": pd.Series([2.34, 123.455, 123.455, None], dtype="float64"),
            "TEST_BINARY_FLOAT": pd.Series(
                [-3.456, 3.1415926535, 3.1415926535, None], dtype="float64"
            ),
            "TEST_BINARY_DOUBLE": pd.Series(
                [9999.99991, -111111.2345, -111111.2345, None], dtype="float64"
            ),
            "TEST_CHAR": pd.Series(["char1", "char2", "char2", None], dtype="object"),
            "TEST_VARCHAR": pd.Series(
                ["varchar1", "varchar222", "varchar222", None], dtype="object"
            ),
            "TEST_NCHAR": pd.Series(
                ["y123  ", "aab123", "aab123", None], dtype="object"
            ),
            "TEST_NVARCHAR": pd.Series(
                ["aK>?KJ@#$%", ")>KDS)(F*&%J", ")>KDS)(F*&%J", None], dtype="object"
            ),
            "TEST_DATE": pd.Series(
                ["2019-05-21", "2020-05-21", "2020-05-21", None], dtype="datetime64[ns]"
            ),
            "TEST_TIMESTAMP": pd.Series(
                [
                    "2019-05-21 01:02:33",
                    "2020-05-21 01:02:33",
                    "2020-05-21 01:02:33",
                    None,
                ],
                dtype="datetime64[ns]",
            ),
            "TEST_TIMESTAMPTZ": pd.Series(
                [
                    "1999-12-01 11:00:00",
                    "1899-12-01 11:00:00",
                    "1899-12-01 11:00:00",
                    None,
                ],
                dtype="datetime64[ns]",
            ),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_empty_result(oracle_url: str) -> None:
    query = "SELECT * FROM test_table where test_int < -100"
    df = read_sql(oracle_url, query)
    expected = pd.DataFrame(
        data={
            "TEST_INT": pd.Series([], dtype="Int64"),
            "TEST_CHAR": pd.Series([], dtype="object"),
            "TEST_FLOAT": pd.Series([], dtype="float64"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_empty_result_on_partition(oracle_url: str) -> None:
    query = "SELECT * FROM test_table where test_int < -100"
    df = read_sql(oracle_url, query, partition_on="test_int", partition_num=3)
    print(df)
    expected = pd.DataFrame(
        data={
            "TEST_INT": pd.Series([], dtype="Int64"),
            "TEST_CHAR": pd.Series([], dtype="object"),
            "TEST_FLOAT": pd.Series([], dtype="float64"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_empty_result_on_some_partition(oracle_url: str) -> None:
    query = "SELECT * FROM test_table where test_int < 2"
    df = read_sql(oracle_url, query, partition_on="test_int", partition_num=3)
    expected = pd.DataFrame(
        data={
            "TEST_INT": pd.Series([1], dtype="Int64"),
            "TEST_CHAR": pd.Series(["str1 "], dtype="object"),
            "TEST_FLOAT": pd.Series([1.1], dtype="float64"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("ORACLE_URL"), reason="Test oracle only when `ORACLE_URL` is set"
)
def test_oracle_cte(oracle_url: str) -> None:
    query = "with test_cte (test_int, test_str) as (select test_int, test_char from test_table where test_float > 0) select test_int, test_str from test_cte"
    df = read_sql(oracle_url, query, partition_on="test_int", partition_num=3)
    df.sort_values(by="TEST_INT", inplace=True, ignore_index=True)
    expected = pd.DataFrame(
        index=range(2),
        data={
            "TEST_INT": pd.Series([1, 2], dtype="Int64"),
            "TEST_STR": pd.Series(["str1 ", "str2 "], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)
