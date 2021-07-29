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
    test_language nvarchar(max),
    test_hello nvarchar(max),
);

INSERT INTO test_str VALUES (0, 'English', 'Hello');
INSERT INTO test_str VALUES (1, '中文', '你好');
INSERT INTO test_str VALUES (2, '日本語', 'こんにちは');
INSERT INTO test_str VALUES (3, 'русский', 'Здра́вствуйте');
INSERT INTO test_str VALUES (4, 'Emoji', '😁😂😜');
INSERT INTO test_str VALUES (5, 'Latin1', '¥§¤®ð');
INSERT INTO test_str VALUES (6, 'Extra', 'y̆');
INSERT INTO test_str VALUES (7, 'Mixed', 'Ha好ち😁ðy̆');
INSERT INTO test_str VALUES (8, '', NULL);


DROP TABLE IF EXISTS test_types;

CREATE TABLE test_types(
    test_date DATE,
    test_time TIME,
    test_datetime DATETIMEOFFSET,
    test_new_decimal NUMERIC,
    test_decimal DECIMAL,
    test_varchar VARCHAR(15),
    test_char CHAR(10)
);

INSERT INTO test_types VALUES ('1999-07-25', '00:00:00', '1999-07-25 00:00:00', 1.1, 1, 'varchar1', 'char1');
INSERT INTO test_types VALUES ('2020-12-31', '23:59:59', '2020-12-31 23:59:59', 2.2, 2, 'varchar2', 'char2');
INSERT INTO test_types VALUES ('2021-01-28', '12:30:30', '2021-01-28 12:30:30', 3.3, 3, 'varchar3', 'char3');

CREATE FUNCTION increment(@val int)  
RETURNS int   
AS   
BEGIN  
    RETURN @val + 1;  
END;