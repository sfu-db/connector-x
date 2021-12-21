show variables like 'char%';

DROP TABLE IF EXISTS test_table;

CREATE TABLE IF NOT EXISTS test_table(
    test_int INTEGER,
    test_float DOUBLE,
    test_enum ENUM('even', 'odd'),
    test_null INTEGER
);

INSERT INTO test_table VALUES (1, 1.1, 'odd', NULL);
INSERT INTO test_table VALUES (2, 2.2,'even',  NULL);
INSERT INTO test_table VALUES (3, 3.3, 'odd', NULL);
INSERT INTO test_table VALUES (4, 4.4, 'even', NULL);
INSERT INTO test_table VALUES (5, 5.5, 'odd', NULL);
INSERT INTO test_table VALUES (6, 6.6, 'even', NULL);


DROP TABLE IF EXISTS test_table_extra;

CREATE TABLE IF NOT EXISTS test_table_extra(
    test_int INTEGER,
    test_str VARCHAR(30)
);

INSERT INTO test_table_extra VALUES (1, 'Ha好ち😁ðy̆');
INSERT INTO test_table_extra VALUES (2, 'こんにちは');
INSERT INTO test_table_extra VALUES (3, 'русский');

DROP TABLE IF EXISTS test_types;

CREATE TABLE IF NOT EXISTS test_types(
    test_timestamp TIMESTAMP NULL,
    test_date DATE,
    test_time TIME,
    test_datetime DATETIME,
    test_new_decimal DECIMAL(15,2),
    test_decimal DECIMAL,
    test_varchar VARCHAR(15),
    test_char CHAR(10),
    test_tiny TINYINT,
    test_short SMALLINT,
    test_int24 MEDIUMINT,
    test_long INT,
    test_longlong BIGINT,
    test_tiny_unsigned TINYINT UNSIGNED,
    test_short_unsigned SMALLINT UNSIGNED,
    test_int24_unsigned MEDIUMINT UNSIGNED,
    test_long_unsigned INT UNSIGNED,
    test_longlong_unsigned BIGINT UNSIGNED,
    test_long_notnull INT NOT NULL,
    test_short_unsigned_notnull SMALLINT UNSIGNED NOT NULL,
    test_float FLOAT,
    test_double DOUBLE,
    test_double_notnull DOUBLE NOT NULL,
    test_year YEAR,
    test_tinyblob TINYBLOB,
    test_blob BLOB,
    test_mediumblob MEDIUMBLOB,
    test_longblob LONGBLOB,
    test_enum ENUM('apple', 'banana', 'orange', 'mango'),
    test_json JSON,
    test_mediumtext MEDIUMTEXT
);

INSERT INTO test_types VALUES ('1970-01-01 00:00:01', NULL, '00:00:00', '1970-01-01 00:00:01', 1.1, 1, NULL, 'char1', -128, -32768, -8388608, -2147483648, -9223372036854775808, NULL, NULL, NULL, NULL, NULL, 1, 1, NULL, -2.2E-308, 1.2345, 1901, NULL, NULL, NULL, NULL, 'apple', '{"name": "piggy", "age": 1}', NULL);
INSERT INTO test_types VALUES ('2038-01-19 00:00:00', '1970-01-01', NULL, '2038-01-19 00:0:00', NULL, 2, 'varchar2', NULL, 127, 32767, 8388607, 2147483647, 9223372036854775807, 255, 65535, 16777215, 4294967295, 1.844674407E19, 2147483647, 65535, -1.1E-38, NULL, -1.1E-3, 2155, 'tinyblob2', 'blobblobblobblob2', 'mediumblob2', 'longblob2', NULL, '{"name": "kitty", "age": 2}', '');
INSERT INTO test_types VALUES (NULL, '2038-01-19', '23:59:59', NULL, 3.3, NULL, 'varchar3', 'char3', NULL, NULL, NULL, NULL, NULL, 0, 0, 0, 0, 0, -2147483648, 0, 3.4E38, 1.7E308, 1.7E30, NULL, 'tinyblob3', 'blobblobblobblob3', 'mediumblob3', 'longblob3', 'mango', NULL, 'medium text!!!!');