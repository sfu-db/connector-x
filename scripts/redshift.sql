DROP TABLE IF EXISTS test_table;
DROP TABLE IF EXISTS test_str;
DROP TABLE IF EXISTS test_types;

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

CREATE TABLE IF NOT EXISTS test_str(
    id INTEGER NOT NULL,
    test_language TEXT,
    test_hello TEXT
);

INSERT INTO test_str VALUES (0, 'English', 'Hello');
INSERT INTO test_str VALUES (1, '中文', '你好');
INSERT INTO test_str VALUES (2, '日本語', 'こんにちは');
INSERT INTO test_str VALUES (3, 'русский', 'Здра́вствуйте');
INSERT INTO test_str VALUES (4, 'Emoji', '😁😂😜');
INSERT INTO test_str VALUES (5, 'Latin1', '¥§¤®ð');
INSERT INTO test_str VALUES (6, 'Extra', 'y̆');
INSERT INTO test_str VALUES (7, 'Mixed', 'Ha好ち😁ðy̆');

CREATE TABLE IF NOT EXISTS test_types(
    test_int16 SMALLINT,
    test_char CHAR,
    test_time TIME,
    test_datetime DATETIME 
);

INSERT INTO test_types VALUES (0, 'a', '08:12:40', '2007-01-01 10:00:19');
INSERT INTO test_types VALUES (1, 'b', '10:03:00', '2005-01-01 22:03:00');
INSERT INTO test_types VALUES (2, 'c', '23:00:10', NULL);
INSERT INTO test_types VALUES (3, 'd', '18:30:00', '1987-01-01 11:00:00');
