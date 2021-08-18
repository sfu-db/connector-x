declare
      num number;
begin
    select count(1) into num from user_tables where table_name = upper('test_table') ;
    if num > 0 then
        execute immediate 'drop table test_table' ;
    end if;
end;

CREATE TABLE admin.test_table(
    test_num_int NUMBER(8),
    test_int INTEGER,
    test_num_float NUMBER(10,1),
    test_float FLOAT(53),
    test_varchar VARCHAR2(10),
    test_char CHAR(5),
    test_nvarchar NVARCHAR2(10),
    test_nchar NCHAR(6)
);

INSERT INTO admin.test_table VALUES (1, 1, 1.1, 1.1, 'varchar1', 'char1', 'nvarchar1', 'nchar1');
INSERT INTO admin.test_table VALUES (2, 2, 2.2, 2.2, 'varchar2', 'char2', 'nvarchar2', 'nchar2');
INSERT INTO admin.test_table VALUES (3, 3, 3.3, 3.3, 'varchar3', 'char3', 'nvarchar3', 'nchar3');

declare
      num number;
begin
    select count(1) into num from user_tables where table_name = upper('test_types') ;
    if num > 0 then
        execute immediate 'drop table test_types' ;
    end if;
end;

CREATE TABLE admin.test_types(
    test_int NUMBER(8),
    test_float NUMBER(10,1),
    test_varchar VARCHAR2(10),
    test_char CHAR(5),
    test_date DATE,
    test_timestamp TIMESTAMP
);

INSERT INTO admin.test_types VALUES (1, 1.1, 'varchar1', 'char1', '2019-05-21', '2019-05-21');
INSERT INTO admin.test_types VALUES (2, 2.2, 'varchar2', 'char2', '2020-05-21', '2020-05-21');
INSERT INTO admin.test_types VALUES (3, 3.3, 'varchar3', 'char3', '2021-05-21', '2021-05-21');

declare
      num number;
begin
    select count(1) into num from user_tables where table_name = upper('test_partition') ;
    if num > 0 then
        execute immediate 'drop table test_partition' ;
    end if;
end;

CREATE TABLE admin.test_partition(
    test_int NUMBER(8),
    test_float NUMBER(10,1)
);

INSERT INTO admin.test_partition VALUES (1, 1.1);
INSERT INTO admin.test_partition VALUES (2, 2.2);
INSERT INTO admin.test_partition VALUES (3, 3.3);
INSERT INTO admin.test_partition VALUES (4, 4.4);
INSERT INTO admin.test_partition VALUES (5, 5.5);
INSERT INTO admin.test_partition VALUES (6, 6.6);
