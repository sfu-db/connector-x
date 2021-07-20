# Supported Protocols, Data Types and Mappings

**Currently we assume all columns are nullabel when inferring schema.**

## Postgres (Redshift)

### Protocols
* `binary`: [Postgres Binary COPY protocol](https://www.postgresql.org/docs/current/sql-copy.html), recommend to use in general since fast data parsing speed.
* `csv`: [Postgres CSV COPY protocol](https://www.postgresql.org/docs/current/sql-copy.html), recommend to use when network is slow (`csv` usually results in smaller size than `binary`).
* `cursor`: Conventional wire protocol (slowest one), recommend to use only when `binary` and `csv` is not supported by the source (e.g. Redshift).

### Postgres-Pandas Type Mapping
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


## MySQL (Clickhouse)

### Protocols
* `binary`: [MySQL Binary protocol](https://github.com/blackbeam/rust-mysql-simple), recommend to use in general.
* `text`: [MySQL Text protocol](https://github.com/blackbeam/rust-mysql-simple), slower than `binary`, recommend to use only when `binary` protocol is not supported by the source (e.g. Clickhouse).

### MySQL-Pandas Type Mapping
| MySQL Type      |      Pandas Type            |  Comment                           |
|:---------------:|:---------------------------:|:----------------------------------:|
| INT             | int64, Int64(nullable)      |                                    |
| BIGINT          | int64, Int64(nullable)      |                                    |
| DOUBLE          | float64                     |                                    |
| DECIMAL         | float64, object(Clickhouse) | Clickhouse return DECIMAL in string |
| VARCHAR         | object                      |                                    |
| CHAR            | object                      |                                    |
| DATE            | datetime64[ns]              | only support date after year 1970  |
| TIME            | object                      |                                    |
| DATETIME        | datetime64[ns]              | only support date after year 1970  |

## SQLite

SQLite does not need to specify protocol.

### SQLite-Pandas Type Mapping

Since SQLite adopts a [dynamic type system](https://www.sqlite.org/datatype3.html), we infer type as follow:
* If there is a declared type of the column, we derive the type using [column affinity rules](https://www.sqlite.org/datatype3.html#affname), code can be found [here](https://github.com/sfu-db/connector-x/blob/main/connectorx/src/sources/sqlite/typesystem.rs#L47).
* Otherwise we directly adopt the value's type in the first row of the result (in each partition), which results in INTEGER, REAL, TEXT and BLOB.
  * If the first row of the result is NULL in the partition, try next partition. Throw an error if first rows of all partitions are NULL for a column.

| SQLite Type      |      Pandas Type            |  Comment                           |
|:----------------:|:---------------------------:|:----------------------------------:|
| INTEGER          | int64, Int64(nullable)      | declared type that contains substring "int" |
| BOOL             | bool, boolean(nullable)     | declared type is "boolean" or "bool" |
| REAL             | float64                     | declared type that contains substring "real", "floa", "doub" |
| TEXT             | object                      | declared type that contains substring "char", "clob", "text" |
| BLOB             | object                      | declared type that contains substring "blob" |
| DATE             | datetime64[ns]              | declared type is "date"            |
| TIME             | object                      | declared type is "time"            |
| TIMESTAMP        | datetime64[ns]              | declared type is "datetime" or "timestamp" |


