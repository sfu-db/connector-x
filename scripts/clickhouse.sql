DROP TABLE IF EXISTS test_table;

CREATE TABLE IF NOT EXISTS test_table(
    test_int UInt64,
    test_str String
) ENGINE = MergeTree()
PRIMARY KEY test_int;

INSERT INTO test_table VALUES (1, 'abc');
INSERT INTO test_table VALUES (2, 'defg');
INSERT INTO test_table VALUES (3, 'hijkl');
INSERT INTO test_table VALUES (4, 'mnopqr');
INSERT INTO test_table VALUES (5, 'st');
INSERT INTO test_table VALUES (6, 'u');

DROP TABLE IF EXISTS test_types;

CREATE TABLE IF NOT EXISTS test_types(
    test_int Int32,
    test_float Float64,
    test_date DATE,
    test_datetime DATETIME,
    test_decimal DECIMAL(15,2),
    test_varchar VARCHAR(15),
    test_char CHAR(10)
) ENGINE = MergeTree()
PRIMARY KEY test_int;

INSERT INTO test_types VALUES (1, 2.3, '1999-07-25', '1999-07-25 23:14:07', 2.22, 'こんにちは', '0123456789');
INSERT INTO test_types VALUES (2, 3.3, '1979-04-07', '1979-04-07 03:04:37', 3.33, 'Ha好ち😁ðy', 'abcdefghij');
INSERT INTO test_types VALUES (3, 4.3, '1999-09-22', '1999-07-25 20:21:14', 4.44, 'b', '321');