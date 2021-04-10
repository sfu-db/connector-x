# Benchmark Setup

## Postgres

1. Download PostgreSQL from docker
```docker pull postgres```

2. Create a directory for mount point (Optional)
```mkdir -p $YOUR_DIR/docker/volumes/postgres```

3. Run PostgreSQL:
```
docker run --rm --name pg-connector -e POSTGRES_USER=postgres -e POSTGRES_DB=tpch -e POSTGRES_PASSWORD=postgres -d -p 5432:5432 -v $YOUR_DIR/docker/volumes/postgres:/var/lib/postgresql/data postgres -c shared_buffers=1024MB
```

## TPC-H

1. Download and compile:
```
git clone https://github.com/gregrahn/tpch-kit.git
cd tpch-kit/dbgen && make MACHINE=LINUX DATABASE=POSTGRESQL
```

2. Generate `LINEITEM` table with scale factor 10
```
# Generate all tables
./dbgen -s 10

# Alternatively you can only generate LINEITEM table using -T
./dbgen -s 10 -T L
```

3. Create table and load schema
```
createdb -h localhost -U postgres -p 5432 tpch
psql -h localhost -p 6666 -U postgres -d tpch < dss.ddl
```

4. Load data into PostgreSQL
```
psql -h localhost -p 5432 -U postgres -d tpch -c "\copy LINEITEM FROM '$YOUR_TPCH_DIR/tpch-kit/dbgen/lineitem.tbl' DELIMITER '|' ENCODING 'LATIN1';"
```

5. Create index for `LINEITEM` on `l_orderkey` (Optional)
```
psql -h localhost -p 5432 -U postgres -d tpch -c "CREATE INDEX lineitem_l_orderkey_idx ON LINEITEM USING btree (l_orderkey);"
```
