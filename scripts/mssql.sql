DROP TABLE IF EXISTS test_table;

CREATE TABLE test_table(
    test_int int,
    test_float 	float(53)
);

INSERT INTO test_table VALUES (1, 1.1);
INSERT INTO test_table VALUES (2, 2.2);
INSERT INTO test_table VALUES (3, 3.3);
INSERT INTO test_table VALUES (4, 4.4);
INSERT INTO test_table VALUES (5, 5.5);
INSERT INTO test_table VALUES (6, 6.6);


DROP TABLE IF EXISTS test_types;

CREATE TABLE test_types(
    test_date DATE,
    test_time TIME,
    test_datetime DATETIMEOFFSET,
    test_new_decimal NUMERIC,
    test_decimal DECIMAL,
    test_varchar VARCHAR(15),
    test_char CHAR(10)
);

INSERT INTO test_types VALUES ('1999-07-25', '00:00:00', '1999-07-25 00:00:00', 1.1, 1, 'varchar1', 'char1');
INSERT INTO test_types VALUES ('2020-12-31', '23:59:59', '2020-12-31 23:59:59', 2.2, 2, 'varchar2', 'char2');
INSERT INTO test_types VALUES ('2021-01-28', '12:30:30', '2021-01-28 12:30:30', 3.3, 3, 'varchar3', 'char3');
