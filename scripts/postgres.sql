DROP TABLE IF EXISTS test_table;
DROP TABLE IF EXISTS test_str;
DROP TABLE IF EXISTS test_types;
DROP TABLE IF EXISTS test_infinite_values;
DROP TYPE IF EXISTS happiness;
DROP EXTENSION IF EXISTS citext;
DROP EXTENSION IF EXISTS ltree;

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

CREATE TABLE IF NOT EXISTS test_infinite_values(
    test_int INTEGER NOT NULL,
    test_date DATE,
	test_timestamp TIMESTAMP,
	test_real REAL,
	test_timestamp_timezone TIMESTAMP WITH TIME ZONE
);

INSERT INTO test_infinite_values VALUES (1, 'infinity'::DATE, 'infinity'::TIMESTAMP, 'infinity'::REAL, 'infinity'::TIMESTAMP);
INSERT INTO test_infinite_values VALUES (2, '-infinity'::DATE, '-infinity'::TIMESTAMP, '-infinity'::REAL, '-infinity'::TIMESTAMP);
INSERT INTO test_infinite_values VALUES (3,NULL, NULL, NULL, NULL);


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
CREATE EXTENSION ltree;
CREATE TABLE IF NOT EXISTS test_types(
    test_bool BOOLEAN,
    test_date DATE,
    test_timestamp TIMESTAMP,
    test_timestamptz TIMESTAMPTZ,
    test_int2 SMALLINT,
    test_int4 INTEGER,
    test_int8 BIGINT,
    test_float4 REAL,
    test_float8 DOUBLE PRECISION,
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
    test_boolarray BOOLEAN[],
    test_i2array SMALLINT[],
    test_i4array Integer[],
    test_i8array BIGINT[],
    test_citext CITEXT,
    test_ltree ltree,
    test_lquery lquery,
    test_ltxtquery ltxtquery,
    test_varchararray VARCHAR[],
    test_textarray TEXT[],
    test_name NAME,
    test_inet INET
);

/*                              test_bool   test_date       test_timestamp                  test_timestamptz                    test_int2   test_int4       test_int8               test_float4 test_float8     test_numeric    test_bpchar test_char   test_varchar    test_uuid                                   test_time           test_interval               test_json                                                                   test_jsonb                                                                  test_bytea      test_enum       test_f4array                    test_f8array                        test_narray                 test_boolarray          test_i2array        test_i4array                    test_i8array                                    test_citext     test_ltree  test_lquery test_ltxtquery  test_varchararray                                                       test_textarray                                                          test_name                      test_inet        */
INSERT INTO test_types VALUES ( TRUE,       '1970-01-01',   '1970-01-01 00:00:01',          '1970-01-01 00:00:01-00',           -32768,     0,              -9223372036854775808,   -1.1,       -1.1,           .01,            '👨‍🍳',       'a',        'abcdefghij',   'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',     '08:12:40',         '1 year 2 months 3 days',   '{"customer": "John Doe", "items": {"product": "Beer", "qty": 6}}',         '{"customer": "John Doe", "items": {"product": "Beer", "qty": 6}}',         '\010',         'happy',        '{-1.1, 0.00}',                 '{-1.1, 0.00}',                     '{0.01, 521.23}',           '{true, false}',        '{12}',             '{-1}',                         '{-9223372036854775808, 9223372036854775807}',  'str_citext',   'A.B.C.D',  '*.B.*',    'A & B*',       ARRAY['str1','str2'],                                                   ARRAY['str1','str2'],                                                   '0',                        '192.168.1.1'                );
INSERT INTO test_types VALUES ( true,       '2000-02-28',   '2000-02-28 12:00:10',          '2000-02-28 12:00:10-04',           0,          1,              0,                      0.00,       0.0000,         521.34,         'bb',       'ಠ',        '',             'a0ee-bc99-9c0b-4ef8-bb6d-6bb9-bd38-0a11',  '18:30:00',         '2 weeks ago',              '{"customer": "Lily Bush", "items": {"product": "Diaper", "qty": 24}}',     '{"customer": "Lily Bush", "items": {"product": "Diaper", "qty": 24}}',     'Здра́вствуйте', 'very happy',   '{}',                           '{}',                               '{0.12, 333.33, 22.22}',    '{}',                   '{}',               '{}',                           '{}',                                           '',             'A.B.E',    'A.*',      'A | B',        '{"0123456789","abcdefghijklmnopqrstuvwxyz","!@#$%^&*()_-+=~`:;<>?/"}', '{"0123456789","abcdefghijklmnopqrstuvwxyz","!@#$%^&*()_-+=~`:;<>?/"}', '21',                        '10.0.0.0/24'                );
INSERT INTO test_types VALUES ( false,      '2038-01-18',   '2038-01-18 23:59:59',          '2038-01-18 23:59:59+08',           1,          -2147483648,    9223372036854775807,    2.123456,   2.12345678901,  '1e-5',         '',         '😃',       '👨‍🍳👨‍🍳👨‍🍳👨',     'A0EEBC99-9C0B-4EF8-BB6D-6BB9BD380A11',     '23:00:10',         '3 months 2 days ago',      '{"customer": "Josh William", "items": {"product": "Toy Car", "qty": 1}}',  '{"customer": "Josh William", "items": {"product": "Toy Car", "qty": 1}}',  '',             'ecstatic',     '{1, -2, -12345.1}',            '{2.12345678901, -12345678901.1}',  '{}',                       '{true}',               '{-32768, 32767}',  '{-2147483648, 2147483647}',    '{0}',                                          'abcdef',       'A',        '*',        'A@',           ARRAY['','  '],                                                         ARRAY['','  '],                                                         'someName',                  '2001:db8::1'                );
INSERT INTO test_types VALUES ( False,      '1901-12-14',   '1901-12-14 00:00:00.062547',   '1901-12-14 00:00:00.062547-12',    32767,      2147483647,     1,                      -12345.1,   -12345678901.1, -1.123e2,       'ddddd',    '@',        '@',            '{a0eebc999c0b4ef8bb6d6bb9bd380a11}',       '00:00:59.062547',  '1 year 2 months 3 days',   '{}',                                                                       '{}',                                                                       '😜',           'ecstatic',     '{2.123456, NULL, 123.123}',    '{2.123456, NULL, 123.123}',        '{0.0, NULL, -112.1}',      '{true, false, NULL}',  '{-1, 0, 1, NULL}', '{-1, 0, 1123, NULL}',          '{-1, 0, 1, NULL}',                             '1234',         '',         '*.A',      'A & B*',       ARRAY['👨‍🍳👨‍🍳👨‍🍳👨','', NULL],                                             ARRAY['👨‍🍳👨‍🍳👨‍🍳👨','', NULL],                                             '101203203-1212323-22131235', '2001:db8::/32'                );
INSERT INTO test_types VALUES ( NULL,       NULL,           NULL,                           NULL,                               NULL,       NULL,           NULL,                   NULL,       NULL,           NULL,           NULL,       NULL,       NULL,           NULL,                                       NULL,               NULL,                      NULL,                                                                        NULL,                                                                       NULL,           NULL,           NULL,                           NULL,                               NULL,                       NULL,                   NULL,               NULL,                           NULL,                                           NULL,           NULL,       NULL,       NULL,           NULL,                                                                   NULL,                                                                   NULL,                        NULL                );


CREATE OR REPLACE FUNCTION increment(i integer) RETURNS integer AS $$
    BEGIN
        RETURN i + 1;
    END;
$$ LANGUAGE plpgsql;

DROP TABLE IF EXISTS vector_types;
DROP EXTENSION IF EXISTS vector;

CREATE EXTENSION vector;
CREATE TABLE vector_types (
    id SERIAL PRIMARY KEY,
    dense_vector vector(10),      
    half_vector halfvec(10),      
    binary_vector bit(10),     
    sparse_vector sparsevec(5)
);

INSERT INTO vector_types (dense_vector, half_vector, binary_vector, sparse_vector) VALUES
    (
        '[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]',  
        '[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]',  
        '1010101010',                                          
        '{1:1,3:2,5:3}/5'                          
    ),
    (
        NULL,NULL,NULL,NULL
    );