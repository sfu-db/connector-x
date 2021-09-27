DROP TABLE IF EXISTS test_table;

CREATE TABLE IF NOT EXISTS test_table(
    test_int INTEGER,
    test_float DOUBLE
);

INSERT INTO test_table VALUES (1, 1.1);
INSERT INTO test_table VALUES (2, 2.2);
INSERT INTO test_table VALUES (3, 3.3);
INSERT INTO test_table VALUES (4, 4.4);
INSERT INTO test_table VALUES (5, 5.5);
INSERT INTO test_table VALUES (6, 6.6);


DROP TABLE IF EXISTS test_types;

CREATE TABLE IF NOT EXISTS test_types(
    test_date DATE,
    test_time TIME,
    test_datetime DATETIME,
    test_new_decimal DECIMAL(15,2),
    test_decimal DECIMAL,
    test_varchar VARCHAR(15),
    test_char CHAR(10)
);

INSERT INTO test_types VALUES ('1999-07-25', '00:00:00', '1999-07-25 00:00:00', 1.1, 1, NULL, 'char1');
INSERT INTO test_types VALUES ('2020-12-31', '23:59:59', '2020-12-31 23:59:59', NULL, 2, 'varchar2', 'char2');
INSERT INTO test_types VALUES ('2021-01-28', '12:30:30', NULL, 3.3, 3, 'varchar3', 'char3');


DROP TABLE IF EXISTS test_more_types;

CREATE TABLE IF NOT EXISTS test_more_types (
    test_tiny TINYINT,
    test_short SMALLINT,
    test_float FLOAT,
    test_year YEAR,
    test_timestamp TIMESTAMP,
    test_blob BLOB,
    test_enum ENUM('apple', 'banana', 'orange', 'mango'),
    test_json JSON
);

INSERT INTO test_more_types VALUES (0, -28, 1.1, 1901, '1990-01-01 00:00:01', 'blobblobblobblob1', 'apple', '{"name": "piggy", "age": 1}');
INSERT INTO test_more_types VALUES (1, 128, 2.2, 2021, '2021-08-05 12:12:12', 'blobblobblobblob2', 'orange', '{"name": "kitty", "age": 2}');
INSERT INTO test_more_types VALUES (0, 725, 3.3, 2155, '2038-01-19 03:14:07', 'blobblobblobblob3', 'mango', '{"name": "puppy", "age": 3}');
