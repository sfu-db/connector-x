# Postgres

### Protocols
* `binary`: [Postgres Binary COPY protocol](https://www.postgresql.org/docs/current/sql-copy.html), recommend to use in general since fast data parsing speed.
* `csv`: [Postgres CSV COPY protocol](https://www.postgresql.org/docs/current/sql-copy.html), recommend to use when network is slow (`csv` usually results in smaller size than `binary`).
* `cursor`: Conventional wire protocol (slowest one), recommend to use only when `binary` and `csv` is not supported by the source (e.g. Redshift).

## Postgres Connection
```{hint}
Adding `sslmode=require` to connection uri parameter force SSL connection. Example: `postgresql://username:password@host:port/db?sslmode=require`. `sslmode=disable` to disable SSL connection.
```

```py
import connectorx as cx
conn = 'postgres://username:password@server:port/database'         # connection token
query = "SELECT * FROM table"                                   # query string
cx.read_sql(conn, query)                                        # read data from Postgres
```

## Postgres-Pandas Type Mapping

| Postgres Type   |      Pandas Type          |  Comment                           |
|:---------------:|:-------------------------:|:----------------------------------:|
| BOOL            | bool, boolean(nullable)   |                                    |
| INT2            | int64, Int64(nullable)    |                                    |
| INT4            | int64, Int64(nullable)    |                                    |
| INT8            | int64, Int64(nullable)    |                                    |
| FLOAT4          | float64                   |                                    |
| FLOAT8          | float64                   |                                    |
| NUMERIC         | float64                   |                                    |
| TEXT            | object                    |                                    |
| BPCHAR          | object                    |                                    |
| VARCHAR         | object                    |                                    |
| CHAR            | object                    |                                    |
| BYTEA           | object                    |                                    |
| DATE            | datetime64[ns]            |                                    |
| TIME            | object                    |                                    |
| TIMESTAMP       | datetime64[ns]            |                                    |
| TIMESTAMPZ      | datetime64[ns]            |                                    |
| UUID            | object                    |                                    |
| JSON            | object                    |                                    |
| JSONB           | object                    |                                    |
| ENUM            | object                    | need to convert enum column to text manually (`::text`) when using `csv` and `cursor` protocol |
| ltree           | object                    | binary protocol returns with a hex char prefix. Check https://github.com/sfu-db/connector-x/pull/382 and https://github.com/sfackler/rust-postgres/issues/960 for status |
| INT2[]          | object                    | list of i64                        |
| INT4[]          | object                    | list of i64                        |
| INT8[]          | object                    | list of i64                        |
| FLOAT4[]        | object                    | list of f64                        |
| FLOAT8[]        | object                    | list of f64                        |
| NUMERIC[]       | object                    | list of f64                        |

## Performance (db.m6g.4xlarge RDS)

- Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/pg-time.png"/></p>

- Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/pg-mem.png"/></p>

In conclusion, ConnectorX uses **3x** less memory and **13x** less time compared with Pandas.
