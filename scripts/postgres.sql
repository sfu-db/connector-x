DROP TABLE IF EXISTS test_table;
DROP TABLE IF EXISTS test_str;
DROP TABLE IF EXISTS test_types;
DROP TYPE IF EXISTS happiness;

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
INSERT INTO test_str VALUES (8, '', NULL);

CREATE TYPE happiness AS ENUM ('happy', 'very happy', 'ecstatic');
CREATE TABLE IF NOT EXISTS test_types(
    test_int16 SMALLINT,
    test_char CHAR,
    test_uuid UUID NOT NULL,
    test_time TIME,
    test_interval INTERVAL,
    test_json JSON,
    test_jsonb JSONB,
    test_bytea BYTEA,
    test_enum happiness,
    test_farray DOUBLE PRECISION[],
    test_iarray Integer[]
);

INSERT INTO test_types VALUES (0, 'a', '86b494cc-96b2-11eb-9298-3e22fbb9fe9d', '08:12:40', '1 year 2 months 3 days', '{"customer": "John Doe", "items": {"product": "Beer","qty": 6}}', '{"product": "Beer","qty": 6}', NULL, 'happy', '{}', '{-1, 0, 1123}');
INSERT INTO test_types VALUES (1, 'b', '86b49b84-96b2-11eb-9298-3e22fbb9fe9d', '10:03:00', '2 weeks ago', '{"customer": "Lily Bush", "items": {"product": "Diaper","qty": 24}}', '{"product": "Diaper","qty": 24}', 'Здра́вствуйте', 'very happy', NULL, '{}');
INSERT INTO test_types VALUES (2, NULL, '86b49c42-96b2-11eb-9298-3e22fbb9fe9d', '23:00:10', '3 months 2 days ago', '{"customer": "Josh William", "items": {"product": "Toy Car","qty": 1}}', '{"product": "Toy Car","qty": 1}', '', 'ecstatic', '{0.0123}', '{-324324}');
INSERT INTO test_types VALUES (3, 'd', '86b49cce-96b2-11eb-9298-3e22fbb9fe9d', '18:30:00', '3 year', '{"customer": "Mary Clark", "items": {"product": "Toy Train","qty": 2}}', '{"product": "Toy Train","qty": 2}', '😜', 'ecstatic', '{0.000234, -12.987654321}', NULL);

CREATE OR REPLACE FUNCTION increment(i integer) RETURNS integer AS $$
    BEGIN
        RETURN i + 1;
    END;
$$ LANGUAGE plpgsql;