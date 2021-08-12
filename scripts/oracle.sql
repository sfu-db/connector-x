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
    test_str VARCHAR2(10)
);

INSERT INTO admin.test_types VALUES (1, 1.1, 'a1');
INSERT INTO admin.test_types VALUES (2, 2.2, 'b2');
INSERT INTO admin.test_types VALUES (3, 3.3, 'c3');
