CREATE SCHEMA IF NOT EXISTS test;

CREATE TABLE IF NOT EXISTS test.test_table(
    test_int INTEGER,
    test_float DOUBLE,
    test_null INTEGER
);

DELETE FROM test.test_table;
INSERT INTO test.test_table VALUES (1, 1.1, NULL);
INSERT INTO test.test_table VALUES (2, 2.2, NULL);
INSERT INTO test.test_table VALUES (3, 3.3, NULL);
INSERT INTO test.test_table VALUES (4, 4.4, NULL);
INSERT INTO test.test_table VALUES (5, 5.5, NULL);
INSERT INTO test.test_table VALUES (6, 6.6, NULL);

DROP TABLE IF EXISTS test.test_table_extra;

CREATE TABLE IF NOT EXISTS test.test_table_extra(
    test_int INTEGER,
    test_str VARCHAR(30)
);

DELETE FROM test.test_table_extra;
INSERT INTO test.test_table_extra VALUES (1, 'Ha好ち😁ðy̆');
INSERT INTO test.test_table_extra VALUES (2, 'こんにちは');
INSERT INTO test.test_table_extra VALUES (3, 'русский');

DROP TABLE IF EXISTS test.test_types;

CREATE TABLE IF NOT EXISTS test.test_types(
    test_boolean BOOLEAN,
    test_int INT,
    test_bigint BIGINT,
    test_real REAL,
    test_double DOUBLE,
    test_decimal DECIMAL(15,2),
    test_date DATE,
    test_time TIME(6),
    test_timestamp TIMESTAMP(6),
    test_varchar VARCHAR(15),
    test_uuid UUID -- TODO: VARBINARY, ROW, ARRAY, MAP
);

DELETE FROM test.test_types;
INSERT INTO test.test_types (test_boolean, test_int, test_bigint, test_real, test_double, test_decimal, test_date, test_time, test_timestamp, test_varchar, test_uuid) VALUES
(TRUE, 123, 123456789012345, CAST(123.456 AS REAL), CAST(123.4567890123 AS DOUBLE), 1234567890.12, date('2023-01-01'), time '12:00:00', cast(timestamp '2023-01-01 12:00:00.123456' AS timestamp(6)), 'Sample text', UUID()),
(FALSE, 321, 123456789012345, CAST(123.456 AS REAL), CAST(123.4567890123 AS DOUBLE), 1234567890.12, date('2023-01-01'), time '12:00:00', cast(timestamp '2023-01-01 12:00:00.123456' AS timestamp(6)), 'Sample text', UUID());
