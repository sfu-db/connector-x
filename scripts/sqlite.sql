DROP TABLE IF EXISTS test_table;

CREATE TABLE IF NOT EXISTS test_table(
    test_int INTEGER NOT NULL,
    test_nullint INTEGER,
    test_str TEXT,
    test_float REAL,
    test_bool BOOLEAN,
    test_date DATE
);

INSERT INTO test_table VALUES (1, 3, 'str1', NULL, True, '1996-03-13');
INSERT INTO test_table VALUES (2, NULL, 'str2', 2.2, False, '1996-01-30');
INSERT INTO test_table VALUES (0, 5, 'こんにちは', 3.1, NULL, '1996-02-28');
INSERT INTO test_table VALUES (3, 7, 'b', 3, False, '2020-01-12');
INSERT INTO test_table VALUES (4, 9, 'Ha好ち😁ðy̆', 7.8, NULL, '1996-04-20');
INSERT INTO test_table VALUES (1314, 2, NULL, -10, True, NULL);
