DROP TABLE IF EXISTS test_table;

CREATE TABLE test_table(
    test_int INTEGER NOT NULL,
    test_nullint INTEGER,
    test_str TEXT,
    test_float REAL
);

INSERT INTO test_table VALUES (1, 3, 'str1', NULL);
INSERT INTO test_table VALUES (2, NULL, 'str2', 2.2);
INSERT INTO test_table VALUES (0, 5, 'a', 3.1);
INSERT INTO test_table VALUES (3, 7, 'b', 3);
INSERT INTO test_table VALUES (4, 9, 'c', 7.8);
INSERT INTO test_table VALUES (1314, 2, NULL, -10);

