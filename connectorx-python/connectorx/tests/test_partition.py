from .. import partition_sql


def test_partition_sql(postgres_url: str) -> None:
    query = "SELECT * FROM test_table"
    queires = partition_sql(
        postgres_url, query, partition_on="test_int", partition_num=2
    )
    assert len(queires) == 2
