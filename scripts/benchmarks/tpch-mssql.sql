-- mssql-cli -S$MSSQL_HOST -U$MSSQL_USER -P$MSSQL_PSWD -d$MSSQL_DB -i tpch-mssql.sql

DROP TABLE IF EXISTS LINEITEM;
CREATE TABLE LINEITEM ( L_ORDERKEY    INTEGER NOT NULL,
	L_PARTKEY     INTEGER NOT NULL,
	L_SUPPKEY     INTEGER NOT NULL,
	L_LINENUMBER  INTEGER NOT NULL,
	L_QUANTITY    DECIMAL(15,2) NOT NULL,
	L_EXTENDEDPRICE  DECIMAL(15,2) NOT NULL,
	L_DISCOUNT    DECIMAL(15,2) NOT NULL,
	L_TAX         DECIMAL(15,2) NOT NULL,
	L_RETURNFLAG  CHAR(1) NOT NULL,
	L_LINESTATUS  CHAR(1) NOT NULL,
	L_SHIPDATE    DATE NOT NULL,
	L_COMMITDATE  DATE NOT NULL,
	L_RECEIPTDATE DATE NOT NULL,
	L_SHIPINSTRUCT CHAR(25) NOT NULL,
	L_SHIPMODE     CHAR(10) NOT NULL,
	L_COMMENT      VARCHAR(44) NOT NULL);

CREATE INDEX lineitem_l_orderkey_idx ON LINEITEM (l_orderkey);

BULK INSERT LINEITEM
FROM '/tmp/lineitem.tbl'
WITH
(
	FORMAT = 'CSV',
	FIELDQUOTE = '"',
	FIRSTROW = 1,
	FIELDTERMINATOR = '|',  --CSV field delimiter
	ROWTERMINATOR = '\n',   --Use to shift the control to next row
	TABLOCK
)

-- bcp tpch.dbo.lineitem in '$TPCH_DIR/lineitem.tbl' -f format.fmt
