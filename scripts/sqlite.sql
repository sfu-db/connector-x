DROP TABLE IF EXISTS test_table;

CREATE TABLE IF NOT EXISTS test_table(
    test_int INTEGER NOT NULL,
    test_nullint INTEGER,
    test_str TEXT,
    test_float REAL,
    test_bool BOOLEAN,
    test_date DATE,
    test_time TIME,
    test_datetime DATETIME
);

INSERT INTO test_table VALUES (1, 3, 'str1', NULL, True, '1996-03-13', '08:12:40', '2007-01-01 10:00:19');
INSERT INTO test_table VALUES (2, NULL, 'str2', 2.2, False, '1996-01-30', '10:03:00', '2005-01-01 22:03:00');
INSERT INTO test_table VALUES (0, 5, 'こんにちは', 3.1, NULL, '1996-02-28', '23:00:10', NULL);
INSERT INTO test_table VALUES (3, 7, 'b', 3, False, '2020-01-12', '23:00:10', '1987-01-01 11:00:00');
INSERT INTO test_table VALUES (4, 9, 'Ha好ち😁ðy̆', 7.8, NULL, '1996-04-20', '18:30:00', NULL);
INSERT INTO test_table VALUES (1314, 2, NULL, -10, True, NULL, '18:30:00', '2007-10-01 10:32:00');