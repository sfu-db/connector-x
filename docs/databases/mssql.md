# MsSQL

```{note}
SQLServer does not need to specify protocol.
```

### MsSQL Connection
```{hint} 
By adding `trusted_connection=true` to connection uri parameter, windows authentication will be enabled. Example: `mssql://host:port/db?trusted_connection=true`
By adding `encrypt=true` to connection uri parameter, SQLServer will use SSL encryption. Example: `mssql://host:port/db?encrypt=true&trusted_connection=true`
```

```py
import connectorx as cx
conn = 'mssql://username:password@server:port/database?encrypt=true&trusted_connection=true'         # connection token
query = 'SELECT * FROM table'                                   # query string
cx.read_sql(conn, query)                                        # read data from MsSQL
```

### SQLServer-Pandas Type Mapping
| SQLServer Type  |      Pandas Type            |  Comment                           |
|:---------------:|:---------------------------:|:----------------------------------:|
| TINYINT         | int64, Int64(nullable)      |                                    |
| SMALLINT        | int64, Int64(nullable)      |                                    |
| INT             | int64, Int64(nullable)      |                                    |
| BIGINT          | int64, Int64(nullable)      |                                    |
| FLOAT           | float64                     |                                    |
| NUMERIC         | float64                     |                                    |
| DECIMAL         | float64                     |                                    |
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
