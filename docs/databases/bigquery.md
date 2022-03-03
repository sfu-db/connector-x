# BigQuery

BigQuery does not need to specify protocol.

### BigQuery-Pandas Type Mapping
| BigQuery Type             |      Pandas Type            |  Comment                           |
|:-------------------------:|:---------------------------:|:----------------------------------:|
| Bool, Boolean             | bool, boolean(nullable)     |                                    |
| Int64, Integer            | int64, Int64(nullable)      |                                    |
| Float64, Float            | float64                     |                                    |
| Numeric                   | float64                     |                                    |
| String                    | object                      |                                    |
| BYTES                     | object                      |                                    |
| Time                      | object                      |                                    |
| DATE                      | datetime64[ns]              |                                    |
| Datetime                  | datetime64[ns]              |                                    |
| TIMESTAMP                 | datetime64[ns]              | UTC                                |