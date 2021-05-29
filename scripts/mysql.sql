DROP TABLE IF EXISTS test_table;

CREATE TABLE IF NOT EXISTS test_table(
    test_int INTEGER NOT NULL,
    test_float DOUBLE PRECISION,
    test_str TEXT,
);

INSERT INTO test_table VALUES (1, 1.1, 'str1');
INSERT INTO test_table VALUES (2, 2.2, 'str2');
