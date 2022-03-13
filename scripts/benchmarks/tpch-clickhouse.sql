-- mysql --local-infile --protocol tcp -h$CLICKHOUSE_HOST -P$CLICKHOUSE_PORT -u$CLICKHOUSE_USER -p$CLICKHOUSE_PASSWORD $CLICKHOUSE_DB < tpch-clickhouse.sql
-- clickhouse-client --user $CLICKHOUSE_USER --password $CLICKHOUSE_PASSWORD --database $CLICKHOUSE_DB --format_csv_delimiter="|" --query="INSERT INTO tpch.lineitem FORMAT CSV" < $TPCH_DIR/lineitem.tbl

DROP TABLE IF EXISTS lineitem;
CREATE TABLE lineitem ( L_ORDERKEY    INTEGER NOT NULL,
                             L_PARTKEY     INTEGER NOT NULL,
                             L_SUPPKEY     INTEGER NOT NULL,
                             L_LINENUMBER  INTEGER NOT NULL,
                             L_QUANTITY    DOUBLE NOT NULL,
                             L_EXTENDEDPRICE  DOUBLE NOT NULL,
                             L_DISCOUNT    DOUBLE NOT NULL,
                             L_TAX         DOUBLE NOT NULL,
                             L_RETURNFLAG  CHAR(1) NOT NULL,
                             L_LINESTATUS  CHAR(1) NOT NULL,
                             L_SHIPDATE    DATE NOT NULL,
                             L_COMMITDATE  DATE NOT NULL,
                             L_RECEIPTDATE DATE NOT NULL,
                             L_SHIPINSTRUCT CHAR(25) NOT NULL,
                             L_SHIPMODE     CHAR(10) NOT NULL,
                             L_COMMENT      VARCHAR(44) NOT NULL
		)Engine=MergeTree() ORDER BY L_ORDERKEY;
