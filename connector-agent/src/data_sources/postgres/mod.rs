mod sql;
mod types;

use crate::data_order::DataOrder;
use crate::data_sources::{Parser, PartitionedSource, Produce, Source};
use crate::errors::{ConnectorAgentError, Result};
use crate::types::DataType;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use fehler::throw;
use postgres::{
    binary_copy::{BinaryCopyOutIter, BinaryCopyOutRow},
    fallible_iterator::FallibleIterator,
    types::Type,
};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use sql::{count_query, limit1_query};
use std::any::type_name;

type PgManager = PostgresConnectionManager<NoTls>;
type PgConn = PooledConnection<PgManager>;

pub struct PostgresSource {
    pool: Pool<PgManager>,
    queries: Vec<String>,
    names: Vec<String>,
    schema: Vec<DataType>,
    pg_schema: Vec<Type>,
}

impl PostgresSource {
    pub fn new(conn: &str) -> Self {
        let manager = PostgresConnectionManager::new(conn.parse().unwrap(), NoTls);
        let pool = Pool::new(manager).unwrap();

        Self {
            pool,
            queries: vec![],
            names: vec![],
            schema: vec![],
            pg_schema: vec![],
        }
    }
}

impl Source for PostgresSource {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = PostgresSourcePartition;
    type TypeSystem = DataType;

    fn set_data_order(&mut self, data_order: DataOrder) -> Result<()> {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorAgentError::UnsupportedDataOrder(data_order));
        }
        Ok(())
    }

    fn set_queries<Q: AsRef<str>>(&mut self, queries: &[Q]) {
        self.queries = queries.iter().map(|q| q.as_ref().to_string()).collect();
    }

    fn fetch_metadata(&mut self) -> Result<()> {
        assert!(self.queries.len() != 0);

        let mut conn = self.pool.get()?;
        for query in &self.queries {
            // assuming all the partition queries yield same schema
            if let Ok(row) = conn.query_one(&limit1_query(query)[..], &[]) {
                let (names, types) = row
                    .columns()
                    .into_iter()
                    .map(|col| (col.name().to_string(), DataType::from(col.type_())))
                    .unzip();
                self.names = names;
                self.schema = types;
                self.pg_schema = row
                    .columns()
                    .into_iter()
                    .map(|col| col.type_().clone())
                    .collect();
                break;
            }
        }

        Ok(())
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    fn schema(&self) -> Vec<Self::TypeSystem> {
        self.schema.clone()
    }

    fn partition(self) -> Result<Vec<Self::Partition>> {
        let mut ret = vec![];
        for query in self.queries {
            let conn = self.pool.get()?;

            ret.push(PostgresSourcePartition::new(conn, &query, &self.pg_schema));
        }
        Ok(ret)
    }
}

pub struct PostgresSourcePartition {
    conn: PgConn,
    query: String,
    pgschema: Vec<Type>,
    nrows: usize,
    ncols: usize,
}

impl PostgresSourcePartition {
    pub fn new(conn: PgConn, query: &str, pgschema: &[Type]) -> Self {
        Self {
            conn,
            query: query.to_string(),
            pgschema: pgschema.to_vec(),
            nrows: 0,
            ncols: pgschema.len(),
        }
    }
}

impl PartitionedSource for PostgresSourcePartition {
    type TypeSystem = DataType;
    type Parser<'a> = PostgresSourceParser<'a>;

    fn prepare(&mut self) -> Result<()> {
        let row = self.conn.query_one(&count_query(&self.query)[..], &[])?;
        self.nrows = row.get::<_, i64>(0) as usize;
        Ok(())
    }

    fn parser(&mut self) -> Result<Self::Parser<'_>> {
        let query = format!("COPY ({}) TO STDOUT WITH BINARY", self.query);
        let reader = self.conn.copy_out(&*query)?; // unless reading the data, it seems like issue the query is fast

        let iter = BinaryCopyOutIter::new(reader, &self.pgschema);

        Ok(PostgresSourceParser::new(iter, &self.pgschema))
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

pub struct PostgresSourceParser<'a> {
    iter: BinaryCopyOutIter<'a>,
    current_row: Option<BinaryCopyOutRow>,
    pgschema: Vec<Type>,
    ncols: usize,
    current_col: usize,
}

impl<'a> PostgresSourceParser<'a> {
    pub fn new(iter: BinaryCopyOutIter<'a>, pgschema: &[Type]) -> Self {
        Self {
            iter,
            current_row: None,
            current_col: 0,
            pgschema: pgschema.to_vec(),
            ncols: pgschema.len(),
        }
    }

    fn next_col_idx(&mut self) -> Result<usize> {
        if self.current_row.is_none() || self.current_col >= self.ncols {
            // first time or new row
            match self.iter.next()? {
                Some(row) => {
                    self.current_row = Some(row);
                    self.current_col = 1;
                    Ok(0)
                }
                None => throw!(anyhow!("Postgres EOF")),
            }
        } else {
            let ret = Ok(self.current_col);
            self.current_col += 1;
            ret
        }
    }
}

impl<'a> Parser<'a> for PostgresSourceParser<'a> {
    type TypeSystem = DataType;
}

impl<'a> Produce<f64> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<f64> {
        let cidx = self.next_col_idx()?;
        match &self.pgschema[cidx] {
            &Type::FLOAT4 => {
                let val: f32 = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val as f64)
            }
            &Type::FLOAT8 => {
                let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val)
            }
            t => {
                throw!(ConnectorAgentError::CannotParse(
                    type_name::<f64>(),
                    t.name().into()
                ))
            }
        }
    }
}

impl<'a> Produce<Option<f64>> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<f64>> {
        let cidx = self.next_col_idx()?;
        match &self.pgschema[cidx] {
            &Type::FLOAT4 => {
                let val: Option<f32> = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val.map(|v| v as _))
            }
            &Type::FLOAT8 | &Type::NUMERIC => {
                let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val)
            }
            t => {
                throw!(ConnectorAgentError::CannotParse(
                    type_name::<Option<f64>>(),
                    t.name().into()
                ))
            }
        }
    }
}

impl<'a> Produce<i64> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<i64> {
        let cidx = self.next_col_idx()?;
        match &self.pgschema[cidx] {
            &Type::INT2 => {
                let val: i16 = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val as _)
            }
            &Type::INT4 => {
                let val: i32 = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val as _)
            }
            &Type::INT8 => {
                let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val)
            }
            t => {
                throw!(ConnectorAgentError::CannotParse(
                    type_name::<i64>(),
                    t.name().into()
                ))
            }
        }
    }
}

impl<'a> Produce<Option<i64>> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<i64>> {
        let cidx = self.next_col_idx()?;
        match &self.pgschema[cidx] {
            &Type::INT2 => {
                let val: Option<i16> = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val.map(|v| v as _))
            }
            &Type::INT4 => {
                let val: Option<i32> = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val.map(|v| v as _))
            }
            &Type::INT8 => {
                let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val)
            }
            t => {
                throw!(ConnectorAgentError::CannotParse(
                    type_name::<Option<i64>>(),
                    t.name().into()
                ))
            }
        }
    }
}

impl<'a> Produce<bool> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<bool> {
        let cidx = self.next_col_idx()?;
        match &self.pgschema[cidx] {
            &Type::BOOL => {
                let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val)
            }
            t => {
                throw!(ConnectorAgentError::CannotParse(
                    type_name::<bool>(),
                    t.name().into()
                ))
            }
        }
    }
}

impl<'a> Produce<Option<bool>> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<bool>> {
        let cidx = self.next_col_idx()?;
        match &self.pgschema[cidx] {
            &Type::BOOL => {
                let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val)
            }
            t => {
                throw!(ConnectorAgentError::CannotParse(
                    type_name::<Option<bool>>(),
                    t.name().into()
                ))
            }
        }
    }
}

impl<'a> Produce<String> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<String> {
        let cidx = self.next_col_idx()?;
        match &self.pgschema[cidx] {
            &Type::TEXT | &Type::VARCHAR | &Type::BPCHAR => {
                let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val)
            }
            t => {
                throw!(ConnectorAgentError::CannotParse(
                    type_name::<String>(),
                    t.name().into()
                ))
            }
        }
    }
}

impl<'a> Produce<Option<String>> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<String>> {
        let cidx = self.next_col_idx()?;
        match &self.pgschema[cidx] {
            &Type::TEXT | &Type::VARCHAR | &Type::BPCHAR => {
                let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val)
            }
            t => {
                throw!(ConnectorAgentError::CannotParse(
                    type_name::<String>(),
                    t.name().into()
                ))
            }
        }
    }
}

impl<'a> Produce<DateTime<Utc>> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<DateTime<Utc>> {
        let cidx = self.next_col_idx()?;
        match &self.pgschema[cidx] {
            &Type::TIMESTAMP | &Type::TIMESTAMPTZ | &Type::DATE => {
                let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val)
            }
            t => {
                throw!(ConnectorAgentError::CannotParse(
                    type_name::<DateTime<Utc>>(),
                    t.name().into()
                ))
            }
        }
    }
}

impl<'a> Produce<Option<DateTime<Utc>>> for PostgresSourceParser<'a> {
    fn produce(&mut self) -> Result<Option<DateTime<Utc>>> {
        let cidx = self.next_col_idx()?;
        match &self.pgschema[cidx] {
            &Type::TIMESTAMP | &Type::TIMESTAMPTZ | &Type::DATE => {
                let val = self.current_row.as_ref().unwrap().try_get(cidx)?;
                Ok(val)
            }
            t => {
                throw!(ConnectorAgentError::CannotParse(
                    type_name::<Option<DateTime<Utc>>>(),
                    t.name().into()
                ))
            }
        }
    }
}
