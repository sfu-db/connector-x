DROP TABLE IF EXISTS test_table;

CREATE TABLE IF NOT EXISTS test_table(
    test_int INTEGER,
    test_float DOUBLE
);

INSERT INTO test_table VALUES (1, 1.1);
INSERT INTO test_table VALUES (2, 2.2);
INSERT INTO test_table VALUES (3, 3.3);

