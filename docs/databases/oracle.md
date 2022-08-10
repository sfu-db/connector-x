# Oracle


### Oracle Connection
```py
import connectorx as cx
conn = 'oracle://username:password@server:port/database'        # connection token
query = 'SELECT * FROM table'                                   # query string
cx.read_sql(conn, query)                                        # read data from Oracle
```

### Oracle-Pandas Type Mapping
| Oracle Type               |      Pandas Type            |  Comment                           |
|:-------------------------:|:---------------------------:|:----------------------------------:|
| Number(\*,0)              | int64, Int64(nullable)      |                                    |
| Number(\*,>0)             | float64                     |                                    |
| Float                     | float64                     |                                    |
| BINARY_FLOAT              | float64                     |                                    |
| BINARY_DOUBLE             | float64                     |                                    |
| VARCHAR2                  | object                      |                                    |
| CHAR                      | object                      |                                    |
| NCHAR                     | object                      |                                    |
| NVarchar2                 | object                      |                                    |
| DATE                      | datetime64[ns]              |                                    |
| TIMESTAMP                 | datetime64[ns]              |                                    |
| TIMESTAMP WITH TIME ZONE  | datetime64[ns]              |                                    |

### Performance (db.r5.4xlarge RDS)

**Modin and Turbodbc does not support read_sql on Oracle**

- Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/oracle-time.png"/></p>

- Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/oracle-mem.png"/></p>

In conclusion, ConnectorX uses **3x** less memory and **3x** less time compared with Pandas.
