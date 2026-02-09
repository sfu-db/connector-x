# ClickHouse

## ClickHouse Connection

```{hint}
Adding `protocol=https` to connection uri parameter use SSL connection. Example: `postgresql://username:password@host:port/db?protocol=https`.
```

```py
import connectorx as cx
conn = 'clickhouse://username:password@server:port/database'    # connection token
query = 'SELECT * FROM table'                                   # query string
cx.read_sql(conn, query)                                        # read data from ClickHouse
```

## ClickHouse-Pandas Type Mapping
