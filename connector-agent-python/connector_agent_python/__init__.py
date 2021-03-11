from .connector_agent_python import write_pandas, read_pg
from .connector_agent_python import read_sql as _read_sql


def read_sql(conn, query, return_type, partition=None):
    if partition is not None:
        return _read_sql(conn, query, return_type, (partition['col'],
                                                    partition['min'],
                                                    partition['max'],
                                                    partition['num'])
                         )
    else:
        return _read_sql(conn, query, return_type)