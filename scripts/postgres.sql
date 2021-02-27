CREATE TABLE IF NOT EXISTS test_table(
    test_int INTEGER NOT NULL,
    test_nullint INTEGER,
    test_str TEXT,
    test_float DOUBLE PRECISION,
    test_bool BOOLEAN
);


INSERT INTO test_table VALUES (1, 3, 'str1', NULL, TRUE);
INSERT INTO test_table VALUES (2, NULL, 'str2', 2.2, FALSE);
INSERT INTO test_table VALUES (0, 5, 'a', 3.1, NULL);
INSERT INTO test_table VALUES (3, 7, 'b', 3, FALSE);
INSERT INTO test_table VALUES (4, 9, 'c', 7.8, NULL);
INSERT INTO test_table VALUES (1314, 2, NULL, -10, TRUE);