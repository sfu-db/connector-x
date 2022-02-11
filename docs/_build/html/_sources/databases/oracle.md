# Oracle

Oracle does not need to specify protocol.

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
