# MsSQL

```{note}
SQLServer does not need to specify protocol.
```

### MsSQL Connection
```{hint} 
if the user password has special characters, they need to be sanitized. example: `from urllib import parse; password = parse.quote_plus(password)`
```

```py
import connectorx as cx
conn = 'mssql://username:password@server:port/database?encrypt=true&trusted_connection=true'         # connection token
query = 'SELECT * FROM table'                                   # query string
cx.read_sql(conn, query)                                        # read data from MsSQL
```

### MSSQL Driver

ConnectorX uses `mssql-tds` by default. Pre-built Python wheels also include
the previous Tiberius backend for compatibility:

```py
import connectorx as cx

cx.mssql_driver = "tiberius"  # opt in before starting a query
cx.mssql_driver = "mssql-tds" # restore the default
```

Changing `mssql_driver` only affects queries started after the assignment.
Rust builds can select one backend with the `src_mssql_tds` or
`src_mssql_tiberius` Cargo feature; enabling both exposes the runtime switch.

Existing Python applications keep using `cx.read_sql(conn, query)` unchanged;
the driver choice does not add a required argument or require compiling wheels.
New wheels use `mssql-tds` by default. Set `cx.mssql_driver = "tiberius"` before
starting reads to retain the previous driver, including its TLS behavior.
The setting is process-wide: do not switch it while other threads start reads.

Existing Rust builds using `src_mssql` (an alias for `src_mssql_tiberius`) or
the `all` feature bundle continue to expose the original Tiberius source,
partition, and parser types. They do not silently opt into the enum wrapper.
Enabling both drivers explicitly selects the runtime-switchable wrapper;
its low-level types are not interchangeable with backend-specific types.

### Connection Parameters
* By adding `trusted_connection=true` to connection uri parameter, windows authentication will be enabled. 
    * Example: `mssql://host:port/db?trusted_connection=true`
* By adding `encrypt=true` to connection uri parameter, SQLServer will use SSL encryption. 
    * Example: `mssql://host:port/db?encrypt=true&trusted_connection=true`
* By adding `trust_server_certificate=true` to connection uri parameter, the SQLServer certificate will not be validated and it is accepted as-is. 
    * Example: `mssql://host:port/db?trust_server_certificate=true&encrypt=true`
* By adding `trust_server_certificate_ca=/path/to/ca-cert.crt` to connection uri parameter, the SQLServer certificate will be validated against the given CA certificate in addition to the system-truststore.
    * Example: `mssql://host:port/db?encrypt=true&trust_server_certificate_ca=/path/to/ca-cert.crt`

### Microsoft Entra ID access token

With the default `mssql-tds` driver, Azure SQL Database, Azure SQL Managed
Instance and other Entra ID-enabled servers can be queried with a Microsoft
Entra ID (formerly Azure AD) access token. The token is passed through
`MsSqlOptions` and the `source_options` argument, never in the connection
string:

```py
import connectorx as cx
from azure.identity import DefaultAzureCredential

token = DefaultAzureCredential().get_token("https://database.windows.net/.default").token
options = cx.MsSqlOptions(access_token=token)
conn = "mssql://myserver.database.windows.net:1433/mydb?encrypt=true"  # no username/password
df = cx.read_sql(conn, "SELECT * FROM table", source_options=options)
```

* Pass the raw token string; ConnectorX handles the wire encoding.
* `source_options` is also accepted by `get_meta`, `partition_sql` and `read_sql_pandas`.
  `MsSqlOptions` can only be used with `mssql://` connections, and its `repr`
  never shows the token.
* The connection string must not contain a username or password, nor
  `trusted_connection=true`; combining them with a token is an error.
* Tokens expire (typically after 60–90 minutes). ConnectorX only uses the token
  to open connections for the current call, so acquire a fresh token (or reuse
  a cached, unexpired one from `azure-identity`) for each call.
* The Tiberius driver (`cx.mssql_driver = "tiberius"`) does not support access
  tokens and raises an error when one is given.
* Rust users pass `SourceOptions::MsSql(MsSqlOptions::new().with_access_token(token))`
  to `get_arrow_with_options`, `new_record_batch_iter_with_options` or
  `partition_with_options`, or call `MsSQLSource::new_with_options` directly.
  The existing functions without options are unchanged.

### `mssql-tds` backend details

Rust builds can select `src_mssql_tds` instead of `src_mssql`
(`src_mssql_tiberius`) to link only TDS, or enable both for runtime selection.

The TDS backend shares a bounded connection pool across metadata, row counts,
and partition readers, sized by `MsSQLSource::new`'s `nconn`. Partitions acquire
leases only while executing, so there may be more partitions than connections.
Like Tiberius, `trusted_connection=true` selects integrated authentication on
Windows, or on Unix when the `integrated-auth-gssapi` feature is enabled.

TLS settings are not fully equivalent between the drivers:

| URL setting | Tiberius | `mssql-tds` |
|:------------|:---------|:------------|
| `encrypt` unset | `NotSupported` (prelogin `0x02`), advertises TLS as unsupported | `PreferOff` (`0x00`); login-only TLS, or full-session TLS when required by the server |
| `encrypt=false` | `Off` (`0x00`) | `PreferOff` (`0x00`) |
| `encrypt=true` | `Required` (`0x03`) | `Required` (`0x03`) |

`mssql-tds` has no public setting matching Tiberius's unset default. Its
login-only TLS also skips certificate-chain validation unconditionally, even with
`trust_server_certificate=false`. Use `encrypt=true` to require full-session
TLS with certificate validation; chain validation is disabled only if
`trust_server_certificate=true` is explicitly requested in this mode.
Boolean values for `encrypt` and `trust_server_certificate` are case-insensitive.

`trust_server_certificate_ca` is rejected by the TDS backend: the driver's
certificate-pinning option is not a substitute for CA validation.

### SQLServer-Pandas Type Mapping
| SQLServer Type  |      Pandas Type            |  Comment                           |
|:---------------:|:---------------------------:|:----------------------------------:|
| TINYINT         | int64, Int64(nullable)      |                                    |
| SMALLINT        | int64, Int64(nullable)      |                                    |
| INT             | int64, Int64(nullable)      |                                    |
| BIGINT          | int64, Int64(nullable)      |                                    |
| FLOAT           | float64                     |                                    |
| NUMERIC         | float64                     |                                    |
| DECIMAL         | float64                     | cannot support precision larger than 28                                   |
| BIT             | bool, boolean(nullable)     |                                    |
| VARCHAR         | object                      |                                    |
| CHAR            | object                      |                                    |
| TEXT            | object                      |                                    |
| NVARCHAR        | object                      |                                    |
| NCHAR           | object                      |                                    |
| NTEXT           | object                      |                                    |
| VARBINARY       | object                      |                                    |
| BINARY          | object                      |                                    |
| IMAGE           | object                      |                                    |
| DATETIME        | datetime64[ns]              |                                    |
| DATETIME2       | datetime64[ns]              |                                    |
| SMALLDATETIME   | datetime64[ns]              |                                    |
| DATE            | datetime64[ns]              |                                    |
| DATETIMEOFFSET  | datetime64[ns]              |                                    |
| TIME            | object                      |                                    |
| UNIQUEIDENTIFIER| object                      |                                    |

### Performance (r5.4xlarge docker in another EC2 instance)

**Modin does not support read_sql on Mssql**

- Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/mssql-time.png"/></p>

- Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/mssql-mem.png"/></p>

In conclusion, ConnectorX uses **3x** less memory and **14x** less time compared with Pandas.
