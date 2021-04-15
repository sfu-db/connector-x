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

CREATE TABLE IF NOT EXISTS test_uuid_char_int16(
    test_int16 SMALLINT,
    test_char CHAR,
    test_uuid UUID NOT NULL
);

INSERT INTO test_uuid_char_int16 VALUES (0, 'a', '86b494cc-96b2-11eb-9298-3e22fbb9fe9d');
INSERT INTO test_uuid_char_int16 VALUES (1, 'b', '86b49b84-96b2-11eb-9298-3e22fbb9fe9d');
INSERT INTO test_uuid_char_int16 VALUES (2, 'c', '86b49c42-96b2-11eb-9298-3e22fbb9fe9d');
INSERT INTO test_uuid_char_int16 VALUES (3, 'd', '86b49cce-96b2-11eb-9298-3e22fbb9fe9d');

