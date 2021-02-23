create table IF NOT EXISTS example(
    id SERIAL PRIMARY KEY, 
    val integer
);

INSERT INTO example(val) VALUES (0);
INSERT INTO example(val) VALUES (1);