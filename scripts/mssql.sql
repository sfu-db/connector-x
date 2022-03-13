DROP TABLE IF EXISTS test_table;

CREATE TABLE test_table(
    test_int INTEGER NOT NULL,
    test_nullint INTEGER,
    test_str VARCHAR(128),
    test_float FLOAT(53),
    test_bool BIT
);


INSERT INTO test_table VALUES (1, 3, 'str1', NULL, 1);
INSERT INTO test_table VALUES (2, NULL, 'str2', 2.2, 0);
INSERT INTO test_table VALUES (0, 5, 'a', 3.1, NULL);
INSERT INTO test_table VALUES (3, 7, 'b', 3, 0);
INSERT INTO test_table VALUES (4, 9, 'c', 7.8, NULL);
INSERT INTO test_table VALUES (1314, 2, NULL, -10, 1);

DROP TABLE IF EXISTS test_str;
CREATE TABLE test_str(
    id INTEGER NOT NULL,
    test_language NVARCHAR(max),
    test_hello NVARCHAR(max),
);

INSERT INTO test_str VALUES (0, N'English', N'Hello');
INSERT INTO test_str VALUES (1, N'中文', N'你好');
INSERT INTO test_str VALUES (2, N'日本語', N'こんにちは');
INSERT INTO test_str VALUES (3, N'русский', N'Здра́вствуйте');
INSERT INTO test_str VALUES (4, N'Emoji', N'😁😂😜');
INSERT INTO test_str VALUES (5, N'Latin1', N'¥§¤®ð');
INSERT INTO test_str VALUES (6, N'Extra', N'y̆');
INSERT INTO test_str VALUES (7, N'Mixed', N'Ha好ち😁ðy̆');
INSERT INTO test_str VALUES (8, N'', NULL);


DROP TABLE IF EXISTS test_types;

CREATE TABLE test_types(
    test_int1 TINYINT,
​    test_int2 SMALLINT,
​    test_int4 INT,
​    test_int8 BIGINT,
​    test_float24 REAL,
​    test_float53 DOUBLE PRECISION,
​    test_floatn FLOAT(18),
    test_date DATE,
    test_time TIME,
    test_datetime DATETIMEOFFSET,
    test_smalldatetime SMALLDATETIME,
​    test_naivedatetime DATETIME,
​    test_naivedatetime2 DATETIME2,
    test_new_decimal NUMERIC(5, 2),
    test_decimal DECIMAL,
    test_varchar VARCHAR(15),
    test_char CHAR(10),
    test_varbinary VARBINARY(10),
    test_binary BINARY(5),
    test_nchar NCHAR(4),
    test_text TEXT,
    test_ntext NTEXT,
    test_uuid UNIQUEIDENTIFIER,
    test_money MONEY,
    test_smallmoney SMALLMONEY
);

INSERT INTO test_types VALUES (0, -32768, -2147483648, -9223372036854775808, NULL, NULL, NULL, '1999-07-25', '00:00:00', NULL, '1990-01-01 10:00:00', '1753-01-01 12:00:00', '1900-01-01 12:00:00.12345', 1.1, 1, NULL, NULL, NULL, NULL, '1234', 'text', 'ntext', '86b494cc-96b2-11eb-9298-3e22fbb9fe9d', NULL, NULL);
INSERT INTO test_types VALUES (255, 32767, 2147483647, 9223372036854775807, -1.18E-38, -2.23E-308, 0, NULL, '23:59:59', '2020-12-31 23:59:59 +00:00', NULL, '2038-12-31 01:00:00', NULl, 2.2, 2, 'varchar2', 'char2', CONVERT(VARBINARY(10), '1234'), CONVERT(BINARY(5), '12'), NULL, 't', 'nt', NULL, 922337203685477.5807, 214748.3647);
INSERT INTO test_types VALUES (NULL, NULL, NULL, NULL, 3.40E+38, 1.79E+308, 123.1234567, '2021-01-28', NULL, '2021-01-28 12:30:30 +01:00', '2079-06-05 23:00:00', NULL, '2027-03-18 14:30:30.54321', NULL, NULL, 'varchar3', 'char3', CONVERT(VARBINARY(10), ''), CONVERT(BINARY(5), ''), '12', NULL, NULL, '86b49b84-96b2-11eb-9298-3e22fbb9fe9d', -922337203685477.5808, -214748.3648);

CREATE FUNCTION increment(@val int)  
RETURNS int   
AS   
BEGIN  
    RETURN @val + 1;  
END;