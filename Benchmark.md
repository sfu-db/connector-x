# Benchmark Setup

## Postgres (Docker)

1. Download PostgreSQL from docker
```
docker pull postgres
```

2. Create a directory for mount point (Optional)
```
mkdir -p $YOUR_DOCKER_DIR/docker/volumes/postgres
```

3. Run PostgreSQL:
```
# With local mount point
docker run --rm --name pg-connector -e POSTGRES_USER=postgres -e POSTGRES_DB=tpch -e POSTGRES_PASSWORD=postgres -d -p 5432:5432 -v $YOUR_DOCKER_DIR/docker/volumes/postgres:/var/lib/postgresql/data postgres -c shared_buffers=1024MB

# Without local mount point
docker run --rm --name pg-connector -e POSTGRES_USER=postgres -e POSTGRES_DB=tpch -e POSTGRES_PASSWORD=postgres -d -p 5432:5432 -c shared_buffers=1024MB
```

## TPC-H

1. Download TPC-H toolkit and compile:
```
git clone https://github.com/gregrahn/tpch-kit.git
cd tpch-kit/dbgen && make MACHINE=LINUX DATABASE=POSTGRESQL
```

2. Generate `LINEITEM` table with scale factor 10
```
# Generate all tables
./dbgen -s 10

# Alternatively you can only generate LINEITEM table using -T option
./dbgen -s 10 -T L
```

3. Create table and load schema
```
createdb -h localhost -U postgres tpch
psql -h localhost -U postgres -d tpch < dss.ddl
```

4. Load data into PostgreSQL
```
psql -h localhost -U postgres -d tpch -c "\copy LINEITEM FROM '$YOUR_TPCH_DIR/tpch-kit/dbgen/lineitem.tbl' DELIMITER '|' ENCODING 'LATIN1';"
```

5. Create index for `LINEITEM` on `l_orderkey`
```
psql -h localhost -U postgres -d tpch -c "CREATE INDEX lineitem_l_orderkey_idx ON LINEITEM USING btree (l_orderkey);"
```
