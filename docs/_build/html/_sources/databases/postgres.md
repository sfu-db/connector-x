# Postgres

## Authentication
```python
import connectorx as cx

cx.read_sql("postgresql://username:password@server:port/database", "SELECT * FROM lineitem", partition_on="l_orderkey", partition_num=10)
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
| INT2[]          | object                    | list of i64                        |
| INT4[]          | object                    | list of i64                        |
| INT8[]          | object                    | list of i64                        |
| FLOAT4[]        | object                    | list of f64                        |
| FLOAT8[]        | object                    | list of f64                        |
| NUMERIC[]       | object                    | list of f64                        |

## Performance (db.m6g.4xlarge RDS)

### Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/pg-time.png"/></p>

### Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/pg-mem.png"/></p>

In conclusion, ConnectorX uses **3x** less memory and **13x** less time compared with Pandas.