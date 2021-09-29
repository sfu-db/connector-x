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

## Redshift: Upload TPC-H
> Note: For Redshift, AWS has already hosted TPC-H data in public s3. We borrow the uploading script from [amazon-redshift-utils](https://github.com/awslabs/amazon-redshift-utils/blob/master/src/CloudDataWarehouseBenchmark/Cloud-DWB-Derived-from-TPCH/3TB/ddl.sql). We only modified `LINEITEM`'s sortkey from `(l_shipdate,l_orderkey)` to `(l_orderkey)`.

1. Make the following changes in the COPY commands of `script/benchmarks/tpch-reshift.sql`:

   1. Change `credentials` accordingly from Redshift.
   2. (Optional) Change TPC-H data size in `from` s3 string. Currently it is 10GB (equivilant to TPC-H scale factor 10). It can be change to 3TB.
 
2. Run modified `tpch-reshift.sql` for Redshift:
```
psql -h <endpoint> -U <userid> -d <databasename> -p <port> -f tpch-reshift.sql
```

# Benchmark result on AWS r5.4xlarge

We load the lineitem table of TPC-H @ scale=10 into a db.m6g.4xlarge RDS cluster on AWS for each database, and then run ConnectorX to download data from the database
on an AWS r5.4xlarge, with the following command:

```python
import connectorx as cx

cx.read_sql("connection string", "SELECT * FROM lineitem", partition_on="l_orderkey", partition_num=4)
```

## Postgres (db.m6g.4xlarge RDS)

## Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/pg-time.png"/></p>

## Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/pg-mem.png"/></p>

In conclusion, ConnectorX uses up to **3.5x** less memory and **21x** less time.

## MySQL (db.m6g.4xlarge RDS)


## Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/mysql-time.png"/></p>

## Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/mysql-mem.png"/></p>

In conclusion, ConnectorX uses up to **3.5x** less memory and **8.7x** less time.


## SQLite (r5.4xlarge EC2 same instance)


## Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/sqlite-time.png"/></p>

## Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-agent/main/assets/sqlite-mem.png"/></p>

In conclusion, ConnectorX uses up to **3.3x** less memory and **11.2x** less time.

## Oracle (db.r5.4xlarge RDS)

**Modin does not support read_sql on Oracle**

## Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/oracle-time.png"/></p>

## Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/oracle-mem.png"/></p>

In conclusion, ConnectorX uses up to **2.5x** less memory and **3.9x** less time.

## Mssql (r5.4xlarge docker in another EC2 instance)

**Modin does not support read_sql on Mssql**

## Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/mssql-time.png"/></p>

## Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/mssql-mem.png"/></p>

In conclusion, ConnectorX uses up to **3.2x** less memory and **11.5x** less time.
