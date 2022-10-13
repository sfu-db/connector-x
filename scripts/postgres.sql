DROP TABLE IF EXISTS test_table;
DROP TABLE IF EXISTS test_str;
DROP TABLE IF EXISTS test_types;
DROP TYPE IF EXISTS happiness;
DROP EXTENSION IF EXISTS citext;

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
CREATE EXTENSION citext;
CREATE TABLE IF NOT EXISTS test_types(
    test_date DATE,
    test_timestamp TIMESTAMP,
    test_timestamptz TIMESTAMPTZ,
    test_int16 SMALLINT,
    test_int64 BIGINT,
    test_float32 REAL,
    test_numeric NUMERIC(5,2),
    test_bpchar BPCHAR(5),
    test_char CHAR,
    test_varchar VARCHAR(10),
    test_uuid UUID,
    test_time TIME,
    test_interval INTERVAL,
    test_json JSON,
    test_jsonb JSONB,
    test_bytea BYTEA,
    test_enum happiness,
    test_f4array REAL[],
    test_f8array DOUBLE PRECISION[],
    test_narray NUMERIC(5,2)[],
    test_i2array SMALLINT[],
    test_i4array Integer[],
    test_i8array BIGINT[],
    test_citext CITEXT
);

INSERT INTO test_types VALUES ('1970-01-01', '1970-01-01 00:00:01', '1970-01-01 00:00:01-00', 0, -9223372036854775808, NULL, NULL, 'a', 'a', NULL, '86b494cc-96b2-11eb-9298-3e22fbb9fe9d', '08:12:40', '1 year 2 months 3 days', '{"customer": "John Doe", "items": {"product": "Beer","qty": 6}}', '{"product": "Beer","qty": 6}', NULL, 'happy','{}', '{}', '{}', '{-1, 0, 1}', '{-1, 0, 1123}', '{-9223372036854775808, 9223372036854775807}', 'str_citext');
INSERT INTO test_types VALUES ('2000-02-28', '2000-02-28 12:00:10', '2000-02-28 12:00:10-04', 1, 0, 3.1415926535, 521.34, 'bb', 'b', 'bb', '86b49b84-96b2-11eb-9298-3e22fbb9fe9d', NULL, '2 weeks ago', '{"customer": "Lily Bush", "items": {"product": "Diaper","qty": 24}}', '{"product": "Diaper","qty": 24}', 'Здра́вствуйте', 'very happy', NULL, NULL, NULL, '{}', '{}', '{}', '');
INSERT INTO test_types VALUES ('2038-01-18', '2038-01-18 23:59:59', '2038-01-18 23:59:59+08', 2, 9223372036854775807, 2.71, 999.99, 'ccc', NULL, 'c', '86b49c42-96b2-11eb-9298-3e22fbb9fe9d', '23:00:10', '3 months 2 days ago', '{"customer": "Josh William", "items": {"product": "Toy Car","qty": 1}}', '{"product": "Toy Car","qty": 1}', '', 'ecstatic', '{123.123}', '{-1e-307, 1e308}', '{521.34}', '{-32768, 32767}', '{-2147483648, 2147483647}', '{0}', 's');
INSERT INTO test_types VALUES (NULL, NULL, NULL, 3, NULL, 0.00, -1e-37, NULL, 'd', 'defghijklm', NULL, '18:30:00', '3 year', NULL, NULL, '😜', NULL, '{-1e-37, 1e37}', '{0.000234, -12.987654321}', '{0.12, 333.33, 22.22}', NULL, NULL, NULL, NULL);

CREATE OR REPLACE FUNCTION increment(i integer) RETURNS integer AS $$
    BEGIN
        RETURN i + 1;
    END;
$$ LANGUAGE plpgsql;