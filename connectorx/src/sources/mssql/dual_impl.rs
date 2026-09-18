//! Runtime-switchable MSSQL source, used when both `src_mssql_tiberius` and
//! `src_mssql_tds` are compiled in (this is how the Python bindings build,
//! so `cx.mssql_driver` can flip backends at runtime — see
//! `driver::set_active_driver`).
//!
//! [`MsSQLSource::new`] reads [`driver::active_driver`] once and picks the
//! matching inner backend; every trait method below is a thin match that
//! delegates to whichever variant is active. There is no cross-backend
//! logic here, just plumbing, so both backends keep behaving exactly as
//! they do when compiled alone (see `tiberius_impl.rs` / `tds_impl.rs`).

use super::driver::{self, MsSQLDriverKind};
use super::errors::MsSQLSourceError;
use super::typesystem::{FloatN, IntN, MsSQLTypeSystem};
use super::{tds_impl, tiberius_impl};
use crate::{
    data_order::DataOrder,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::CXQuery,
};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rust_decimal::Decimal;
use std::sync::Arc;
use tokio::runtime::Runtime;
use uuid_old::Uuid;

pub enum MsSQLSource {
    Tiberius(tiberius_impl::MsSQLSource),
    MssqlTds(tds_impl::MsSQLSource),
}

impl MsSQLSource {
    pub fn new(rt: Arc<Runtime>, conn: &str, nconn: usize) -> Result<Self, MsSQLSourceError> {
        match driver::active_driver() {
            MsSQLDriverKind::Tiberius => Ok(MsSQLSource::Tiberius(tiberius_impl::MsSQLSource::new(
                rt, conn, nconn,
            )?)),
            MsSQLDriverKind::MssqlTds => Ok(MsSQLSource::MssqlTds(tds_impl::MsSQLSource::new(
                rt, conn, nconn,
            )?)),
        }
    }
}

impl Source for MsSQLSource {
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = MsSQLSourcePartition;
    type TypeSystem = MsSQLTypeSystem;
    type Error = MsSQLSourceError;

    fn set_data_order(&mut self, data_order: DataOrder) -> Result<(), Self::Error> {
        match self {
            MsSQLSource::Tiberius(s) => s.set_data_order(data_order),
            MsSQLSource::MssqlTds(s) => s.set_data_order(data_order),
        }
    }

    fn set_queries<Q: ToString>(&mut self, queries: &[CXQuery<Q>]) {
        match self {
            MsSQLSource::Tiberius(s) => s.set_queries(queries),
            MsSQLSource::MssqlTds(s) => s.set_queries(queries),
        }
    }

    fn set_origin_query(&mut self, query: Option<String>) {
        match self {
            MsSQLSource::Tiberius(s) => s.set_origin_query(query),
            MsSQLSource::MssqlTds(s) => s.set_origin_query(query),
        }
    }

    fn fetch_metadata(&mut self) -> Result<(), Self::Error> {
        match self {
            MsSQLSource::Tiberius(s) => s.fetch_metadata(),
            MsSQLSource::MssqlTds(s) => s.fetch_metadata(),
        }
    }

    fn result_rows(&mut self) -> Result<Option<usize>, Self::Error> {
        match self {
            MsSQLSource::Tiberius(s) => s.result_rows(),
            MsSQLSource::MssqlTds(s) => s.result_rows(),
        }
    }

    fn names(&self) -> Vec<String> {
        match self {
            MsSQLSource::Tiberius(s) => s.names(),
            MsSQLSource::MssqlTds(s) => s.names(),
        }
    }

    fn schema(&self) -> Vec<Self::TypeSystem> {
        match self {
            MsSQLSource::Tiberius(s) => s.schema(),
            MsSQLSource::MssqlTds(s) => s.schema(),
        }
    }

    fn partition(self) -> Result<Vec<Self::Partition>, Self::Error> {
        match self {
            MsSQLSource::Tiberius(s) => Ok(s
                .partition()?
                .into_iter()
                .map(MsSQLSourcePartition::Tiberius)
                .collect()),
            MsSQLSource::MssqlTds(s) => Ok(s
                .partition()?
                .into_iter()
                .map(MsSQLSourcePartition::MssqlTds)
                .collect()),
        }
    }
}

pub enum MsSQLSourcePartition {
    Tiberius(tiberius_impl::MsSQLSourcePartition),
    MssqlTds(tds_impl::MsSQLSourcePartition),
}

impl SourcePartition for MsSQLSourcePartition {
    type TypeSystem = MsSQLTypeSystem;
    type Parser<'a> = MsSQLSourceParser<'a>;
    type Error = MsSQLSourceError;

    fn result_rows(&mut self) -> Result<(), Self::Error> {
        match self {
            MsSQLSourcePartition::Tiberius(p) => p.result_rows(),
            MsSQLSourcePartition::MssqlTds(p) => p.result_rows(),
        }
    }

    fn parser(&mut self) -> Result<Self::Parser<'_>, Self::Error> {
        match self {
            MsSQLSourcePartition::Tiberius(p) => Ok(MsSQLSourceParser::Tiberius(p.parser()?)),
            MsSQLSourcePartition::MssqlTds(p) => Ok(MsSQLSourceParser::MssqlTds(p.parser()?)),
        }
    }

    fn nrows(&self) -> usize {
        match self {
            MsSQLSourcePartition::Tiberius(p) => p.nrows(),
            MsSQLSourcePartition::MssqlTds(p) => p.nrows(),
        }
    }

    fn ncols(&self) -> usize {
        match self {
            MsSQLSourcePartition::Tiberius(p) => p.ncols(),
            MsSQLSourcePartition::MssqlTds(p) => p.ncols(),
        }
    }
}

pub enum MsSQLSourceParser<'a> {
    Tiberius(tiberius_impl::MsSQLSourceParser<'a>),
    // mssql-tds's `TdsClient` owns its connection outright, so this variant
    // carries no lifetime of its own (see tds_impl.rs).
    MssqlTds(tds_impl::MsSQLSourceParser),
}

impl<'a> PartitionParser<'a> for MsSQLSourceParser<'a> {
    type TypeSystem = MsSQLTypeSystem;
    type Error = MsSQLSourceError;

    fn fetch_next(&mut self) -> Result<(usize, bool), Self::Error> {
        match self {
            MsSQLSourceParser::Tiberius(p) => p.fetch_next(),
            MsSQLSourceParser::MssqlTds(p) => p.fetch_next(),
        }
    }
}

macro_rules! impl_produce_enum {
    ($($t: ty,)+) => {
        $(
            impl<'r, 'a> Produce<'r, $t> for MsSQLSourceParser<'a> {
                type Error = MsSQLSourceError;

                fn produce(&'r mut self) -> Result<$t, Self::Error> {
                    match self {
                        MsSQLSourceParser::Tiberius(p) => Produce::<$t>::produce(p),
                        MsSQLSourceParser::MssqlTds(p) => Produce::<$t>::produce(p),
                    }
                }
            }

            impl<'r, 'a> Produce<'r, Option<$t>> for MsSQLSourceParser<'a> {
                type Error = MsSQLSourceError;

                fn produce(&'r mut self) -> Result<Option<$t>, Self::Error> {
                    match self {
                        MsSQLSourceParser::Tiberius(p) => Produce::<Option<$t>>::produce(p),
                        MsSQLSourceParser::MssqlTds(p) => Produce::<Option<$t>>::produce(p),
                    }
                }
            }
        )+
    };
}

impl_produce_enum!(
    u8,
    i16,
    i32,
    i64,
    IntN,
    f32,
    f64,
    FloatN,
    bool,
    &'r str,
    &'r [u8],
    Uuid,
    Decimal,
    NaiveDateTime,
    NaiveDate,
    NaiveTime,
    DateTime<Utc>,
);
