DROP TABLE IF EXISTS test_table_duckdb;
DROP TABLE IF EXISTS test_str_duckdb;

CREATE TABLE IF NOT EXISTS test_table_duckdb(
    test_int INTEGER NOT NULL,
    test_nullint INTEGER,
    test_str TEXT,
    test_float DOUBLE PRECISION,
    test_bool BOOLEAN
);
INSERT INTO test_table_duckdb VALUES (1, 3, 'str1', NULL, TRUE);
INSERT INTO test_table_duckdb VALUES (2, NULL, 'str2', 2.2, FALSE);
INSERT INTO test_table_duckdb VALUES (0, 5, 'a', 3.1, NULL);
INSERT INTO test_table_duckdb VALUES (3, 7, 'b', 3, FALSE);
INSERT INTO test_table_duckdb VALUES (4, 9, 'c', 7.8, NULL);
INSERT INTO test_table_duckdb VALUES (1314, 2, NULL, -10, TRUE);
CREATE TABLE IF NOT EXISTS test_str_duckdb(
    id INTEGER NOT NULL,
    test_language TEXT,
    test_hello TEXT
);
INSERT INTO test_str_duckdb VALUES (0, 'English', 'Hello');
INSERT INTO test_str_duckdb VALUES (1, '中文', '你好');
INSERT INTO test_str_duckdb VALUES (2, '日本語', 'こんにちは');
INSERT INTO test_str_duckdb VALUES (3, 'русский', 'Здра́вствуйте');
INSERT INTO test_str_duckdb VALUES (4, 'Emoji', '😁😂😜');
INSERT INTO test_str_duckdb VALUES (5, 'Latin1', '¥§¤®ð');
INSERT INTO test_str_duckdb VALUES (6, 'Extra', 'y̆');
INSERT INTO test_str_duckdb VALUES (7, 'Mixed', 'Ha好ち😁ðy̆');
INSERT INTO test_str_duckdb VALUES (8, '', NULL);
