DROP TABLE test_table;
DROP TABLE test_types;
DROP TABLE test_issue;

CREATE TABLE test_table(
    test_int NUMBER(7),
    test_char CHAR(5),
    test_float FLOAT(53)
);

INSERT INTO test_table VALUES (1, 'str1', 1.1);
INSERT INTO test_table VALUES (2, 'str2', 2.2);
INSERT INTO test_table VALUES (2333, NULL, NULL);
INSERT INTO test_table VALUES (4, NULL, -4.44);
INSERT INTO test_table VALUES (5, 'str05', NULL);

CREATE TABLE test_issue(
    v BINARY_FLOAT
);

INSERT INTO test_issue VALUES (1.111);
INSERT INTO test_issue VALUES (2.222);
INSERT INTO test_issue VALUES (3.333);
INSERT INTO test_issue VALUES (NULL);


CREATE TABLE test_types(
    test_num_int NUMBER(8),
    test_int INTEGER,
    test_num_float NUMBER(10,1),
    test_float FLOAT(38),
    test_binary_float BINARY_FLOAT,
    test_binary_double BINARY_DOUBLE,
    test_char CHAR(5),
    test_varchar VARCHAR2(10),
    test_nchar NCHAR(6),
    test_nvarchar NVARCHAR2(20),
    test_date DATE,
    test_timestamp TIMESTAMP,
    test_timestamptz TIMESTAMP WITH TIME ZONE,
    test_clob CLOB,
    test_blob BLOB
);


INSERT INTO test_types VALUES (1, -10, 2.3, 2.34, -3.456, 9999.99991, 'char1', 'varchar1', 'y123', 'aK>?KJ@#$%', TO_DATE('2019-05-21', 'YYYY-MM-DD'), TO_TIMESTAMP('2019-05-21 01:02:33', 'YYYY-MM-DD HH24:MI:SS'), TO_TIMESTAMP_TZ('1999-12-01 11:00:00 -8:00',
   'YYYY-MM-DD HH:MI:SS TZH:TZM'), '13ab', '39af');
INSERT INTO test_types VALUES (5, 22, -0.1, 123.455, 3.1415926535, -111111.2345, 'char2', 'varchar222', 'aab123', ')>KDS)(F*&%J', TO_DATE('2020-05-21', 'YYYY-MM-DD'), TO_TIMESTAMP('2020-05-21 01:02:33', 'YYYY-MM-DD HH24:MI:SS'), TO_TIMESTAMP_TZ('1899-12-01 11:00:00 +1:00',
   'YYYY-MM-DD HH:MI:SS TZH:TZM'), '13ab', '39af');
INSERT INTO test_types VALUES (5, 22, -0.1, 123.455, 3.1415926535, -111111.2345, 'char2', 'varchar222', 'aab123', ')>KDS)(F*&%J', TO_DATE('2020-05-21', 'YYYY-MM-DD'), TO_TIMESTAMP('2020-05-21 01:02:33', 'YYYY-MM-DD HH24:MI:SS'), TO_TIMESTAMP_TZ('1899-12-01 11:00:00 +1:00',
   'YYYY-MM-DD HH:MI:SS TZH:TZM'), '13ab', '39af');
INSERT INTO test_types VALUES (NULL, 100, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);
