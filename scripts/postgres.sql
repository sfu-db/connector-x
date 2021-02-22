CREATE TABLE IF NOT EXISTS test_postgres_conn(
    test_int integer,
    test_str text,
    test_float real,
    test_bool boolean
);
INSERT INTO test_postgres_conn VALUES (1, 'str1', 1.1, true);
INSERT INTO test_postgres_conn VALUES (2, 'str2', 2.2, false);