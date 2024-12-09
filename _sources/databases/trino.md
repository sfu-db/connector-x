# Trino

## Connection

```{hint}
Using `trino+http` as connection protocol disables TLS for the connection. Example: `trino+http://host:port/catalog`
Notice that Trino requires TLS for basic auth credentials. To use self-signed certificates, add `verify=false` like this: `trino+https://host:port/catalog?verify=false`
```

```py
import connectorx as cx
conn = 'trino+https://username:password@server:port/catalog'     # connection token
query = "SELECT * FROM table"                                    # query string
cx.read_sql(conn, query)                                         # read data from Trino
```

## Trino-Pandas Type Mapping

| Trino Type |       Pandas Type       | Comment |
| :--------: | :---------------------: | :-----: |
|  BOOLEAN   | bool, boolean(nullable) |         |
|  TINYINT   | int64, Int64(nullable)  |         |
|  SMALLINT  | int64, Int64(nullable)  |         |
|    INT     | int64, Int64(nullable)  |         |
|   BIGINT   | int64, Int64(nullable)  |         |
|    REAL    |         float64         |         |
|   DOUBLE   |         float64         |         |
|  DECIMAL   |         float64         |         |
|  VARCHAR   |         object          |         |
|    CHAR    |         object          |         |
|    DATE    |     datetime64[ns]      |         |
|    TIME    |         object          |         |
| TIMESTAMP  |     datetime64[ns]      |         |
|    UUID    |         object          |         |
|    JSON    |         object          |         |
| IPADDRESS  |         object          |         |
