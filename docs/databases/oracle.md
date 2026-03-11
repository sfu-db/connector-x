# Oracle

### System Authentication
```{hint}
If you want to use system authentication to access Oracle, username & password should not present in the connection string.
```

### Oracle Connection
```py
import connectorx as cx
conn = 'oracle://username:password@server:port/database'        # connection token
query = 'SELECT * FROM table'                                   # query string
cx.read_sql(conn, query)                                        # read data from Oracle
```

### Oracle TNS Alias (DNS) Connection
```py
import connectorx as cx
conn = 'oracle://username:password@alias_name?alias=true'        # connection token
query = 'SELECT * FROM table'                                   # query string
cx.read_sql(conn, query)                                        # read data from Oracle
```

### Specifying Default Schema

With version>=0.4.5 you can specify a default schema by adding the `schema` query parameter to the connection URL. This automatically sets the current schema for all connections, eliminating the need to prefix table names with the schema in your queries.

```py
import connectorx as cx
# Specify schema in the connection URL
conn = 'oracle://username:password@server:port/database?schema=MY_SCHEMA'
query = 'SELECT * FROM table'  # No need to use MY_SCHEMA.table
cx.read_sql(conn, query)
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

### Development Setup

To load the database seed data into the database, you will need `sqlplus`.

Install it from Oracle's website's InstantClient page: https://www.oracle.com/database/technologies/instant-client/linux-x86-64-downloads.html.

On Linux, this can be done by:

```bash
wget https://download.oracle.com/otn_software/linux/instantclient/2370000/instantclient-sqlplus-linux.x64-23.7.0.25.01.zip
unzip instantclient-sqlplus-linux.x64-23.7.0.25.01.zip
mkdir /opt/oracle/instantclient_23_7/
mv instantclient_23_7/*.so /opt/oracle/instantclient_23_7/
mv instantclient_23_7/* /usr/bin/
export LD_LIBRARY_PATH=/opt/oracle/instantclient_23_7:$LD_LIBRARY_PATH
sqlplus -h
```

Run the Oracle tests with:
```bash
cat scripts/oracle.sql | sqlplus $ORACLE_URL_SCRIPT

cargo test --features all  --test test_oracle -- --ignored
```
