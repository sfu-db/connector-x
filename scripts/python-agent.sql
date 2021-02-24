create table IF NOT EXISTS example(
    id SERIAL PRIMARY KEY, 
    ii INTEGER,
    ff DOUBLE PRECISION,
    ss TEXT
);

INSERT INTO example(ii, ff, ss) VALUES (0, 3.1, 'a'),(null, 3,'b'),(4, 'NaN', 'c'),(1314,-10, null);
