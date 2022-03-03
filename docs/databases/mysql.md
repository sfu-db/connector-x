# MySQL

## Protocols
* `binary`: [MySQL Binary protocol](https://github.com/blackbeam/rust-mysql-simple), recommend to use in general.
* `text`: [MySQL Text protocol](https://github.com/blackbeam/rust-mysql-simple), slower than `binary`, recommend to use only when `binary` protocol is not supported by the source (e.g. Clickhouse).

## MySQL-Pandas Type Mapping
| MySQL Type      |      Pandas Type            |  Comment                           |
|:---------------:|:---------------------------:|:----------------------------------:|
| TINYINT         | int64, Int64(nullable)      |                                    |
| SMALLINT        | int64, Int64(nullable)      |                                    |
| MEDIUMINT       | int64, Int64(nullable)      |                                    |
| INT             | int64, Int64(nullable)      |                                    |
| BIGINT          | int64, Int64(nullable)      |                                    |
| FLOAT           | float64                     |                                    |
| DOUBLE          | float64                     |                                    |
| DECIMAL         | float64, object(Clickhouse) | Clickhouse return DECIMAL in string |
| VARCHAR         | object                      |                                    |
| CHAR            | object                      |                                    |
| DATE            | datetime64[ns]              | only support date after year 1970  |
| TIME            | object                      |                                    |
| DATETIME        | datetime64[ns]              | only support date after year 1970  |
| TIMESTAMP       | datetime64[ns]              |                                    |
| YEAR            | int64, Int64(nullable)      |                                    |
| TINYBLOB        | object                      |                                    |
| BLOB            | object                      |                                    |
| MEDIUMBLOB      | object                      |                                    |
| LONGBLOB        | object                      |                                    |
| JSON            | object                      |                                    |
| ENUM            | object                      |                                    |
