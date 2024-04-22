import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql


@pytest.fixture(scope="module")  # type: ignore
def trino_url() -> str:
    conn = os.environ["TRINO_URL"]
    return conn


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_without_partition(trino_url: str) -> None:
    query = "select * from test.test_table order by test_int limit 3"
    df = read_sql(trino_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 3], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3], dtype="float64"),
            "test_null": pd.Series([None, None, None], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_with_partition(trino_url: str) -> None:
    query = "select * from test.test_table order by test_int"
    df = read_sql(
        trino_url,
        query,
        partition_on="test_int",
        partition_range=(0, 10),
        partition_num=6,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3, 4.4, 5.5, 6.6], dtype="float64"),
            "test_null": pd.Series([None, None, None, None, None, None], dtype="Int64"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_without_partition(trino_url: str) -> None:
    query = "SELECT * FROM test.test_table order by test_int"
    df = read_sql(trino_url, query)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3, 4.4, 5.5, 6.6], dtype="float64"),
            "test_null": pd.Series([None, None, None, None, None, None], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_limit_without_partition(trino_url: str) -> None:
    query = "SELECT * FROM test.test_table order by test_int limit 3"
    df = read_sql(trino_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 3], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3], dtype="float64"),
            "test_null": pd.Series([None, None, None], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_limit_large_without_partition(trino_url: str) -> None:
    query = "SELECT * FROM test.test_table order by test_int limit 10"
    df = read_sql(trino_url, query)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3, 4.4, 5.5, 6.6], dtype="float64"),
            "test_null": pd.Series([None, None, None, None, None, None], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_with_partition(trino_url: str) -> None:
    query = "SELECT * FROM test.test_table order by test_int"
    df = read_sql(
        trino_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3, 4.4, 5.5, 6.6], dtype="float64"),
            "test_null": pd.Series([None, None, None, None, None, None], dtype="Int64"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_limit_with_partition(trino_url: str) -> None:
    query = "SELECT * FROM test.test_table order by test_int limit 3"
    df = read_sql(
        trino_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 3], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3], dtype="float64"),
            "test_null": pd.Series([None, None, None], dtype="Int64"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_limit_large_with_partition(trino_url: str) -> None:
    query = "SELECT * FROM test.test_table order by test_int limit 10"
    df = read_sql(
        trino_url,
        query,
        partition_on="test_int",
        partition_range=(0, 2000),
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3, 4.4, 5.5, 6.6], dtype="float64"),
            "test_null": pd.Series([None, None, None, None, None, None], dtype="Int64"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_with_partition_without_partition_range(trino_url: str) -> None:
    query = "SELECT * FROM test.test_table where test_float > 3"
    df = read_sql(
        trino_url,
        query,
        partition_on="test_int",
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_int": pd.Series([3, 4, 5, 6], dtype="Int64"),
            "test_float": pd.Series([3.3, 4.4, 5.5, 6.6], dtype="float64"),
            "test_null": pd.Series([None, None, None, None], dtype="Int64"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)

    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_manual_partition(trino_url: str) -> None:
    queries = [
        "SELECT * FROM test.test_table WHERE test_int < 2 order by test_int",
        "SELECT * FROM test.test_table WHERE test_int >= 2 order by test_int",
    ]
    df = read_sql(trino_url, query=queries)
    expected = pd.DataFrame(
        index=range(6),
        data={
            "test_int": pd.Series([1, 2, 3, 4, 5, 6], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3, 4.4, 5.5, 6.6], dtype="float64"),
            "test_null": pd.Series([None, None, None, None, None, None], dtype="Int64"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_selection_and_projection(trino_url: str) -> None:
    query = "SELECT test_int FROM test.test_table WHERE test_float < 5 order by test_int"
    df = read_sql(
        trino_url,
        query,
        partition_on="test_int",
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(4),
        data={
            "test_int": pd.Series([1, 2, 3, 4], dtype="Int64"),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_join(trino_url: str) -> None:
    query = "SELECT T.test_int, T.test_float, S.test_str FROM test.test_table T INNER JOIN test.test_table_extra S ON T.test_int = S.test_int order by T.test_int"
    df = read_sql(
        trino_url,
        query,
        partition_on="test_int",
        partition_num=3,
    )
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_int": pd.Series([1, 2, 3], dtype="Int64"),
            "test_float": pd.Series([1.1, 2.2, 3.3], dtype="float64"),
            "test_str": pd.Series(
                [
                    "Ha好ち😁ðy̆",
                    "こんにちは",
                    "русский",
                ],
                dtype="object",
            ),
        },
    )
    df.sort_values(by="test_int", inplace=True, ignore_index=True)
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_aggregate(trino_url: str) -> None:
    query = "select AVG(test_float) as avg_float, SUM(T.test_int) as sum_int, SUM(test_null) as sum_null from test.test_table as T"
    df = read_sql(trino_url, query)
    expected = pd.DataFrame(
        index=range(1),
        data={
            "avg_float": pd.Series([3.85], dtype="float64"),
            "sum_int": pd.Series([21], dtype="Int64"),
            "sum_null": pd.Series([None], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_trino_types_binary(trino_url: str) -> None:
    query = "select test_boolean, test_int, test_bigint, test_real, test_double, test_decimal, test_date, test_time, test_timestamp, test_varchar, test_uuid from test.test_types order by test_int"
    df = read_sql(trino_url, query)
    expected = pd.DataFrame(
        index=range(3),
        data={
            "test_boolean": pd.Series([True, False, None], dtype="boolean"),
            "test_int": pd.Series([123, 321, None], dtype="Int64"),
            "test_bigint": pd.Series([1000, 2000, None], dtype="Int64"),
            "test_real": pd.Series([123.456, 123.456, None], dtype="float64"),
            "test_double": pd.Series([123.4567890123, 123.4567890123, None], dtype="float64"),
            "test_decimal": pd.Series([1234567890.12, 1234567890.12, None], dtype="float64"),
            "test_date": pd.Series(["2023-01-01", "2023-01-01", None], dtype="datetime64[ns]"),
            "test_time": pd.Series(["12:00:00", "12:00:00", None], dtype="object"),
            "test_timestamp": pd.Series(["2023-01-01 12:00:00.123456", "2023-01-01 12:00:00.123456", None], dtype="datetime64[ns]"),
            "test_varchar": pd.Series(["Sample text", "Sample text", None], dtype="object"),
            "test_uuid": pd.Series(["f4967dbb-33e9-4242-a13a-45b56ce60dba", "1c8b79d0-4508-4974-b728-7651bce4a5a5", None], dtype="object"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_empty_result(trino_url: str) -> None:
    query = "SELECT * FROM test.test_table where test_int < -100"
    df = read_sql(trino_url, query)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([], dtype="Int64"),
            "test_float": pd.Series([], dtype="float64"),
            "test_null": pd.Series([], dtype="Int64"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_empty_result_on_partition(trino_url: str) -> None:
    query = "SELECT * FROM test.test_table where test_int < -100"
    df = read_sql(trino_url, query, partition_on="test_int", partition_num=3)
    expected = pd.DataFrame(
        data={
            "test_int": pd.Series([], dtype="Int64"),
            "test_float": pd.Series([], dtype="float64"),
            "test_null": pd.Series([], dtype="Int64"),
        }
    )
    assert_frame_equal(df, expected, check_names=True)


@pytest.mark.skipif(
    not os.environ.get("TRINO_URL"), reason="Test Trino only when `TRINO_URL` is set"
)
def test_empty_result_on_some_partition(trino_url: str) -> None:
    query = "SELECT * FROM test.test_table where test_int = 6"
    df = read_sql(trino_url, query, partition_on="test_int", partition_num=3)
    expected = pd.DataFrame(
        index=range(1),
        data={
            "test_int": pd.Series([6], dtype="Int64"),
            "test_float": pd.Series([6.6], dtype="float64"),
            "test_null": pd.Series([None], dtype="Int64"),
        },
    )
    assert_frame_equal(df, expected, check_names=True)
