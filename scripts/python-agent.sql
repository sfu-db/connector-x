create table IF NOT EXISTS example(
    id SERIAL PRIMARY KEY, 
    ii INTEGER,
    ff DOUBLE PRECISION
);

INSERT INTO example(ii, ff) VALUES (0, 3.1),(null, 3),(4, 'NaN');
