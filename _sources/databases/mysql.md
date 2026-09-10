# MySQL

## Protocols
* `binary`: [MySQL Binary protocol](https://github.com/blackbeam/rust-mysql-simple), recommend to use in general.
* `text`: [MySQL Text protocol](https://github.com/blackbeam/rust-mysql-simple), slower than `binary`, recommend to use only when `binary` protocol is not supported by the source (e.g. Clickhouse).

## MySQL Connection
```py
import connectorx as cx
conn = 'mysql://username:password@server:port/database'         # connection token
query = 'SELECT * FROM table'                                   # query string
cx.read_sql(conn, query)                                        # read data from MySQL
```

## MySQL SSL/TLS

Add an `ssl-mode` parameter to the connection URI to negotiate a TLS connection, e.g.
`mysql://username:password@server:port/database?ssl-mode=REQUIRED`. Values are case-insensitive and
follow MySQL's semantics:

| `ssl-mode`        | Behavior                                                                 |
|:-----------------:|:-------------------------------------------------------------------------|
| `DISABLED`        | No TLS. Same as omitting `ssl-mode` (the default).                       |
| `PREFERRED`       | Encrypt without verifying the server certificate. See the note below.    |
| `REQUIRED`        | Encrypt without verifying the server certificate.                        |
| `VERIFY_CA`       | Encrypt and verify the server certificate against a CA.                  |
| `VERIFY_IDENTITY` | Encrypt and verify both the CA and the server hostname.                  |

For `VERIFY_CA` and `VERIFY_IDENTITY`, supply the CA certificate with `ssl-ca` (a path to a PEM or DER
file), e.g. `?ssl-mode=VERIFY_CA&ssl-ca=/path/to/ca.pem`. When `ssl-ca` is omitted, verification uses
the system trust store.

> **Note:** `PREFERRED` is treated the same as `REQUIRED` (the connection is always encrypted); unlike
> the MySQL client, it does not fall back to an unencrypted connection when the server lacks TLS.

> **Note:** Client-certificate (mutual TLS) authentication is not yet supported.

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
| DECIMAL         | float64, object(Clickhouse) | Clickhouse return DECIMAL in string, cannot support precision larger than 28 |
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


### Performance (db.m6g.4xlarge RDS)

- Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/mysql-time.png"/></p>

- Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/mysql-mem.png"/></p>

In conclusion, ConnectorX uses **3x** less memory and **8x** less time compared with Pandas.
