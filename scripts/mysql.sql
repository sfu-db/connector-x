DROP TABLE IF EXISTS test_table;

CREATE TABLE IF NOT EXISTS test_table(
    test_int INTEGER NOT NULL,
    test_str TEXT,
    test_float DOUBLE PRECISION,
);

INSERT INTO test_table VALUES (1, 'str1', 1.0);
INSERT INTO test_table VALUES (2, 'str2', 2.2);
INSERT INTO test_table VALUES (0, 'a', 3.1);
INSERT INTO test_table VALUES (3, 'b', 3.5);
INSERT INTO test_table VALUES (4, 'c', 7.8);
INSERT INTO test_table VALUES (1314, 'd', -10.0);