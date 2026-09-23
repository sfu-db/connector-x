//! `mssql-tds`-backed implementation of the MsSQL source.
//!
//! Added in Phase 2 of the tiberius -> mssql-tds migration (see
//! sfu-db/connector-x#942). Compiled only when the `src_mssql_tds` feature
//! is active. This implementation targets the same
//! [`super::typesystem::MsSQLTypeSystem`] and the same set of native Rust
//! produce types as [`super::tiberius_impl`], so
//! [`crate::transports::mssql_arrow::MsSQLArrowTransport`] works unchanged
//! against either backend.
//!
//! Automatic partition-range discovery supports integer and floating-point
//! columns, matching Tiberius's NULL and float-to-integer conversions.
//! Metadata, counts and partition readers share a per-source bounded bb8 pool.
//! Partitions acquire connections only when executing, so their number may
//! exceed the pool size.
//!
//! Known, intentional differences from the Tiberius path (tracked as
//! documented gaps, not bugs):
//! - `trust_server_certificate_ca` (validate the server cert against a
//!   specific CA bundle without disabling hostname/chain checks) is
//!   rejected outright: `mssql-tds`'s `EncryptionOptions::server_certificate`
//!   only supports exact-match certificate pinning, not CA validation.
//! - The default TLS posture differs: Tiberius defaults to
//!   `EncryptionLevel::NotSupported` (advertise TLS as unsupported) when
//!   `encrypt` is unset (prelogin 0x02). `mssql-tds` has no equivalent public
//!   setting: unset and `encrypt=false` both map to `PreferOff` (0x00).
//!   This permits login-only TLS, or full-session TLS if the server requires it.
//!   The driver's login-only TLS unconditionally skips certificate-chain validation,
//!   even when `trust_server_certificate=false`. Use `encrypt=true` for required
//!   full-session TLS (0x03), with certificate-chain validation enabled unless
//!   `trust_server_certificate=true` is explicitly requested.

use super::driver;
use super::errors::MsSQLSourceError;
use super::typesystem::{FloatN, IntN, MsSQLTypeSystem};
use crate::constants::DB_BUFFER_SIZE;
use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{PartitionParser, Produce, Source, SourcePartition},
    sql::{count_query, get_partition_range_query, CXQuery},
};
use anyhow::anyhow;
use bb8::{ManageConnection, Pool, PooledConnection};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use fehler::{throw, throws};
use log::{debug, warn};
use mssql_tds::connection::client_context::{ClientContext, TdsAuthenticationMethod};
use mssql_tds::connection::tds_client::{ResultSet, TdsClient};
use mssql_tds::connection_provider::tds_connection_provider::TdsConnectionProvider;
use mssql_tds::core::{EncryptionOptions, EncryptionSetting};
use mssql_tds::datatypes::column_values::{
    ColumnValues, SqlDate, SqlDateTime, SqlDateTime2, SqlDateTimeOffset, SqlSmallDateTime, SqlTime,
};
use mssql_tds::datatypes::decoder::DecimalParts;
use mssql_tds::datatypes::sqldatatypes::TdsDataType;
use rust_decimal::Decimal;
use sqlparser::dialect::MsSqlDialect;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use url::Url;
use urlencoding::decode;
use uuid_old::Uuid;

/// Builds the `mssql-tds` datasource string and [`ClientContext`] from a
/// ConnectorX MSSQL connection URL. Mirrors
/// [`super::tiberius_impl::mssql_config`]'s parsing, but expressed against
/// `mssql-tds`'s config surface.
#[throws(MsSQLSourceError)]
fn build_client_context(url: &Url) -> (String, ClientContext) {
    let host = decode(url.host_str().unwrap_or("localhost"))?.into_owned();
    let port = url.port().unwrap_or(1433);
    let hosts: Vec<&str> = host.split('\\').collect();
    let datasource = match hosts.len() {
        1 => format!("tcp:{},{}", hosts[0], port),
        // SQL Server named instance: `server\instance`. Let mssql-tds's own
        // SSRP resolution figure out the port, same as Tiberius does today.
        2 => format!("{}\\{}", hosts[0], hosts[1]),
        _ => throw!(anyhow!("MsSQL hostname parse error: {}", host)),
    };

    let mut context = ClientContext::with_data_source(&datasource);
    context.database = decode(&url.path()[1..])?.into_owned();

    let params: HashMap<String, String> = url.query_pairs().into_owned().collect();

    match params.get("trusted_connection") {
        Some(v) if v == "true" && cfg!(any(windows, feature = "integrated-auth-gssapi")) => {
            debug!("mssql-tds auth through integrated (SSPI) authentication");
            context.tds_authentication_method = TdsAuthenticationMethod::SSPI;
        }
        _ => {
            debug!("mssql-tds auth through sqlserver authentication");
            context.tds_authentication_method = TdsAuthenticationMethod::Password;
            context.user_name = decode(url.username())?.into_owned();
            context.password = decode(url.password().unwrap_or(""))?.into_owned();
        }
    }

    if let Some(v) = params.get("trust_server_certificate_ca") {
        // See the module-level doc comment: mssql-tds has no CA-validation
        // equivalent yet, only exact-match certificate pinning. Reject
        // rather than silently downgrade TLS trust.
        throw!(anyhow!(
            "trust_server_certificate_ca={} is not supported with the src_mssql_tds backend: \
             mssql-tds only supports exact-match certificate pinning, not CA validation",
            v
        ));
    }

    let mut encryption = EncryptionOptions::new();
    encryption.mode = EncryptionSetting::PreferOff;
    match params.get("encrypt") {
        Some(v) if v.to_lowercase() == "true" => encryption.mode = EncryptionSetting::Required,
        Some(v) if v.to_lowercase() == "false" => encryption.mode = EncryptionSetting::PreferOff,
        _ => {}
    }
    if let Some(v) = params.get("trust_server_certificate") {
        if v.to_lowercase() == "true" {
            encryption.trust_server_certificate = true;
        }
    }
    context.encryption_options = encryption;

    if let Some(appname) = params.get("appname") {
        context.application_name = decode(appname)?.into_owned();
    }

    (datasource, context)
}

#[cfg(test)]
mod configuration_tests {
    use super::*;

    #[test]
    fn integrated_auth_respects_tiberius_platform_and_feature_gates() {
        for trusted in ["true", "false", "TrUe"] {
            let url = Url::parse(&format!(
                "mssql://test_user:test_password@localhost/db?trusted_connection={trusted}"
            ))
            .unwrap();
            let (_, context) = build_client_context(&url).unwrap();
            if trusted == "true" && cfg!(any(windows, feature = "integrated-auth-gssapi")) {
                assert!(matches!(
                    context.tds_authentication_method,
                    TdsAuthenticationMethod::SSPI
                ));
            } else {
                assert!(matches!(
                    context.tds_authentication_method,
                    TdsAuthenticationMethod::Password
                ));
                assert_eq!(context.user_name, "test_user");
                assert_eq!(context.password, "test_password");
            }
        }
    }

    #[test]
    fn tls_mapping_preserves_explicit_encryption_and_trust() {
        for (encrypt, mode) in [
            ("", EncryptionSetting::PreferOff),
            ("true", EncryptionSetting::Required),
            ("TrUe", EncryptionSetting::Required),
            ("false", EncryptionSetting::PreferOff),
            ("FaLsE", EncryptionSetting::PreferOff),
            ("invalid", EncryptionSetting::PreferOff),
        ] {
            for (trust, expected_trust) in [
                ("", false),
                ("true", true),
                ("TrUe", true),
                ("false", false),
                ("FaLsE", false),
                ("invalid", false),
            ] {
                let mut url = Url::parse("mssql://user:password@localhost/db").unwrap();
                if !encrypt.is_empty() {
                    url.query_pairs_mut().append_pair("encrypt", encrypt);
                }
                if !trust.is_empty() {
                    url.query_pairs_mut()
                        .append_pair("trust_server_certificate", trust);
                }
                let (_, context) = build_client_context(&url).unwrap();
                assert_eq!(
                    context.encryption_options,
                    EncryptionOptions {
                        mode,
                        trust_server_certificate: expected_trust,
                        host_name_in_cert: None,
                        server_certificate: None,
                    },
                    "encrypt={encrypt:?}, trust_server_certificate={trust:?}"
                );
            }
        }
    }

    #[test]
    fn custom_ca_is_rejected_even_when_trust_is_enabled() {
        for trust in ["true", "false"] {
            let url = Url::parse(&format!(
                "mssql://localhost/db?encrypt=true&trust_server_certificate={trust}\
                 &trust_server_certificate_ca=ca.pem"
            ))
            .unwrap();
            let err = build_client_context(&url).err().unwrap();
            assert!(err.to_string().contains("not CA validation"));
        }
    }

    #[test]
    fn pool_size_must_be_positive_and_fit_u32() {
        let rt = Arc::new(Runtime::new().unwrap());
        assert!(MsSQLSource::new(rt.clone(), "mssql://localhost/db", 0).is_err());
        if usize::BITS > 32 {
            assert!(MsSQLSource::new(rt, "mssql://localhost/db", usize::MAX).is_err());
        }
    }
}

#[throws(MsSQLSourceError)]
fn open_connection(rt: &Runtime, url: &Url) -> TdsClient {
    let (datasource, context) = build_client_context(url)?;
    let provider = TdsConnectionProvider::new();
    rt.block_on(provider.create_client(context, &datasource, None))?
}

struct ConnectionManager {
    url: Url,
}

#[async_trait::async_trait]
impl ManageConnection for ConnectionManager {
    type Connection = TdsClient;
    type Error = MsSQLSourceError;

    async fn connect(&self) -> Result<TdsClient, Self::Error> {
        let (datasource, context) = build_client_context(&self.url)?;
        Ok(TdsConnectionProvider::new()
            .create_client(context, &datasource, None)
            .await?)
    }

    async fn is_valid(&self, client: &mut PooledConnection<'_, Self>) -> Result<(), Self::Error> {
        if client.is_connection_dead() {
            return Err(anyhow!("MsSQL pooled connection is closed").into());
        }
        // Reset session settings and transactions before the next borrower's query.
        client.prepare_reset_connection(false);
        Ok(())
    }

    fn has_broken(&self, client: &mut TdsClient) -> bool {
        client.is_connection_dead() || client.has_open_batch()
    }
}

#[throws(MsSQLSourceError)]
fn connection_pool(rt: &Runtime, url: &Url, nconn: usize) -> Pool<ConnectionManager> {
    let max_size = u32::try_from(nconn)
        .ok()
        .filter(|&size| size > 0)
        .ok_or_else(|| anyhow!("MsSQL pool size must be between 1 and {}", u32::MAX))?;
    // Fail on unsupported URL options even before the first checkout.
    build_client_context(url)?;
    rt.block_on(
        Pool::builder()
            .max_size(max_size)
            .test_on_check_out(true)
            .build(ConnectionManager { url: url.clone() }),
    )?
}

/// Owns the lease and keeps its runtime alive through cleanup and pool return.
struct Connection {
    rt: Arc<Runtime>,
    lease: Option<PooledConnection<'static, ConnectionManager>>,
}

impl Connection {
    #[throws(MsSQLSourceError)]
    fn get(rt: Arc<Runtime>, pool: &Pool<ConnectionManager>) -> Self {
        let lease = rt.block_on(pool.get_owned()).map_err(|err| match err {
            bb8::RunError::User(err) => err,
            bb8::RunError::TimedOut => anyhow!("Timed out waiting for an MsSQL connection").into(),
        })?;
        Self {
            rt,
            lease: Some(lease),
        }
    }

    #[throws(MsSQLSourceError)]
    fn finish(&mut self) {
        let client = self.lease.as_mut().unwrap();
        if let Err(err) = self.rt.block_on(client.close_query()) {
            client.mark_connection_dead();
            throw!(err);
        }
    }
}

impl Deref for Connection {
    type Target = TdsClient;

    fn deref(&self) -> &TdsClient {
        self.lease.as_ref().unwrap()
    }
}

impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut TdsClient {
        self.lease.as_mut().unwrap()
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let mut client = self.lease.take().unwrap();
        if !client.is_connection_dead() && client.has_open_batch() {
            // Early parser drops must not return unread protocol data to the pool.
            // Bound destructor work; a timed-out drain is discarded, never reused.
            match self.rt.block_on(async {
                tokio::time::timeout(Duration::from_secs(5), client.close_query()).await
            }) {
                Ok(Ok(())) => {}
                Ok(Err(err)) => {
                    warn!("Discarding MsSQL connection after cleanup failure: {}", err);
                    client.mark_connection_dead();
                }
                Err(_) => {
                    warn!("Discarding MsSQL connection after cleanup timeout");
                    client.mark_connection_dead();
                }
            }
        }
        // bb8 may spawn replacement connections when returning a broken lease.
        let _guard = self.rt.enter();
        drop(client);
    }
}

/// Runs `sql`, returning its first row for `COUNT(*)` probes.
#[throws(MsSQLSourceError)]
fn first_row(rt: &Runtime, client: &mut TdsClient, sql: String) -> Option<Vec<ColumnValues>> {
    rt.block_on(client.execute(sql, ()))?;
    rt.block_on(client.next_row())?
}

#[throws(MsSQLSourceError)]
pub(crate) fn tds_get_partition_range(url: &Url, query: &str, col: &str) -> (i64, i64) {
    let range_query = get_partition_range_query(query, col, &MsSqlDialect {})?;
    let rt = Runtime::new().map_err(anyhow::Error::from)?;
    let mut client = open_connection(&rt, url)?;
    rt.block_on(client.execute(range_query, ()))?;
    let types: Vec<_> = client
        .get_metadata()
        .iter()
        .map(|column| column.data_type)
        .collect();
    let row = rt
        .block_on(client.next_row())?
        .ok_or_else(|| anyhow!("MsSQL partition range query returned no row"))?;
    if row.len() != 2 || types.len() != 2 {
        throw!(anyhow!(
            "MsSQL partition range query must return two columns"
        ));
    }
    (
        partition_range_value(&row[0], types[0])?,
        partition_range_value(&row[1], types[1])?,
    )
}

#[throws(MsSQLSourceError)]
fn partition_range_value(value: &ColumnValues, ty: TdsDataType) -> i64 {
    if !matches!(
        ty,
        TdsDataType::Int1
            | TdsDataType::Int2
            | TdsDataType::Int4
            | TdsDataType::Int8
            | TdsDataType::IntN
            | TdsDataType::Flt4
            | TdsDataType::Flt8
            | TdsDataType::FltN
    ) {
        throw!(anyhow!(
            "Partition can only be done on int or float columns"
        ));
    }
    // Match Tiberius: NULL aggregates become zero and floats truncate to i64.
    match value {
        ColumnValues::Null => 0,
        ColumnValues::TinyInt(n) => i64::from(*n),
        ColumnValues::SmallInt(n) => i64::from(*n),
        ColumnValues::Int(n) => i64::from(*n),
        ColumnValues::BigInt(n) => *n,
        ColumnValues::Real(n) => *n as i64,
        ColumnValues::Float(n) => *n as i64,
        other => throw!(anyhow!(
            "Unexpected MsSQL partition range value: {:?}",
            other
        )),
    }
}

#[cfg(test)]
mod partition_range_tests {
    use super::*;

    #[test]
    fn numeric_bounds_preserve_tiberius_conversions() {
        for (value, ty, expected) in [
            (ColumnValues::TinyInt(255), TdsDataType::Int1, 255),
            (ColumnValues::SmallInt(-32768), TdsDataType::Int2, -32768),
            (
                ColumnValues::Int(i32::MIN),
                TdsDataType::Int4,
                i64::from(i32::MIN),
            ),
            (ColumnValues::BigInt(i64::MIN), TdsDataType::Int8, i64::MIN),
            (ColumnValues::BigInt(i64::MAX), TdsDataType::IntN, i64::MAX),
            (ColumnValues::Real(-12.75), TdsDataType::Flt4, -12),
            (ColumnValues::Float(23.5), TdsDataType::Flt8, 23),
            (ColumnValues::Float(-12.75), TdsDataType::FltN, -12),
            (ColumnValues::Null, TdsDataType::IntN, 0),
            (ColumnValues::Null, TdsDataType::FltN, 0),
        ] {
            assert_eq!(partition_range_value(&value, ty).unwrap(), expected);
        }
    }

    #[test]
    fn unsupported_bounds_are_errors_even_when_null() {
        for ty in [
            TdsDataType::BigVarChar,
            TdsDataType::DecimalN,
            TdsDataType::MoneyN,
        ] {
            let err = partition_range_value(&ColumnValues::Null, ty).unwrap_err();
            assert!(err
                .to_string()
                .contains("Partition can only be done on int or float columns"));
        }
        assert!(partition_range_value(&ColumnValues::Bit(true), TdsDataType::IntN).is_err());
    }
}

pub struct MsSQLSource {
    rt: Arc<Runtime>,
    conn_url: Url,
    pool: Pool<ConnectionManager>,
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<MsSQLTypeSystem>,
}

impl MsSQLSource {
    #[throws(MsSQLSourceError)]
    pub fn new(rt: Arc<Runtime>, conn: &str, nconn: usize) -> Self {
        debug!("mssql source using driver: {:?}", driver::active_driver());
        let conn_url = Url::parse(conn)?;
        let pool = connection_pool(&rt, &conn_url, nconn)?;

        Self {
            rt,
            conn_url,
            pool,
            origin_query: None,
            queries: vec![],
            names: vec![],
            schema: vec![],
        }
    }
}

impl Source for MsSQLSource
where
    MsSQLSourcePartition: SourcePartition<TypeSystem = MsSQLTypeSystem, Error = MsSQLSourceError>,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = MsSQLSourcePartition;
    type TypeSystem = MsSQLTypeSystem;
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn set_data_order(&mut self, data_order: DataOrder) {
        if !matches!(data_order, DataOrder::RowMajor) {
            throw!(ConnectorXError::UnsupportedDataOrder(data_order));
        }
    }

    fn set_queries<Q: ToString>(&mut self, queries: &[CXQuery<Q>]) {
        self.queries = queries.iter().map(|q| q.map(Q::to_string)).collect();
    }

    fn set_origin_query(&mut self, query: Option<String>) {
        self.origin_query = query;
    }

    #[throws(MsSQLSourceError)]
    fn fetch_metadata(&mut self) {
        assert!(!self.queries.is_empty());

        let first_query = self.queries[0].clone();
        let mut client = Connection::get(self.rt.clone(), &self.pool)?;
        self.rt
            .block_on(client.execute(first_query.as_str().to_string(), ()))?;

        let metadata = client.get_metadata();
        if metadata.is_empty() {
            throw!(anyhow!(
                "MsSQL returned no columns for query: {}",
                first_query
            ));
        }

        let (names, types) = metadata
            .iter()
            .map(|col| (col.column_name.clone(), MsSQLTypeSystem::from(col)))
            .unzip();
        client.finish()?;

        self.names = names;
        self.schema = types;
    }

    #[throws(MsSQLSourceError)]
    fn result_rows(&mut self) -> Option<usize> {
        match &self.origin_query {
            Some(q) => {
                let cxq = CXQuery::Naked(q.clone());
                let cquery = count_query(&cxq, &MsSqlDialect {})?;
                let mut client = Connection::get(self.rt.clone(), &self.pool)?;
                let row = first_row(&self.rt, &mut client, cquery.as_str().to_string())?
                    .ok_or_else(|| anyhow!("MsSQL failed to get the count of query: {}", q))?;
                client.finish()?;
                Some(row_count_value(&row, q)?)
            }
            None => None,
        }
    }

    fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    fn schema(&self) -> Vec<Self::TypeSystem> {
        self.schema.clone()
    }

    #[throws(MsSQLSourceError)]
    fn partition(self) -> Vec<Self::Partition> {
        let mut ret = vec![];
        for query in self.queries {
            let mut partition = MsSQLSourcePartition::new(
                self.rt.clone(),
                self.conn_url.clone(),
                &query,
                &self.schema,
            );
            partition.pool = Some(self.pool.clone());
            ret.push(partition);
        }
        ret
    }
}

/// Reads the first column of a `COUNT(*)` result row. SQL Server always
/// returns `COUNT(*)` as `int`.
#[throws(MsSQLSourceError)]
fn row_count_value(row: &[ColumnValues], query: &str) -> usize {
    match row.first() {
        Some(ColumnValues::Int(n)) => *n as usize,
        Some(ColumnValues::BigInt(n)) => *n as usize,
        other => throw!(anyhow!(
            "MsSQL count query for '{}' returned unexpected value: {:?}",
            query,
            other
        )),
    }
}

pub struct MsSQLSourcePartition {
    rt: Arc<Runtime>,
    conn_url: Url,
    pool: Option<Pool<ConnectionManager>>,
    query: CXQuery<String>,
    schema: Vec<MsSQLTypeSystem>,
    nrows: usize,
    ncols: usize,
}

impl MsSQLSourcePartition {
    pub fn new(
        rt: Arc<Runtime>,
        conn_url: Url,
        query: &CXQuery<String>,
        schema: &[MsSQLTypeSystem],
    ) -> Self {
        Self {
            rt,
            conn_url,
            pool: None,
            query: query.clone(),
            schema: schema.to_vec(),
            nrows: 0,
            ncols: schema.len(),
        }
    }

    #[throws(MsSQLSourceError)]
    fn connection(&mut self) -> Connection {
        // Preserve the public, infallible standalone partition constructor.
        if self.pool.is_none() {
            self.pool = Some(connection_pool(&self.rt, &self.conn_url, 1)?);
        }
        Connection::get(self.rt.clone(), self.pool.as_ref().unwrap())?
    }
}

impl SourcePartition for MsSQLSourcePartition {
    type TypeSystem = MsSQLTypeSystem;
    // Owned leases need no borrow from the partition or unsafe self-reference.
    type Parser<'a> = MsSQLSourceParser;
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn result_rows(&mut self) {
        let cquery = count_query(&self.query, &MsSqlDialect {})?;
        let mut client = self.connection()?;
        let row = first_row(&self.rt, &mut client, cquery.as_str().to_string())?
            .ok_or_else(|| anyhow!("MsSQL failed to get the count of query: {}", self.query))?;
        client.finish()?;
        self.nrows = row_count_value(&row, self.query.as_str())?;
    }

    #[throws(MsSQLSourceError)]
    fn parser<'a>(&'a mut self) -> Self::Parser<'a> {
        let mut client = self.connection()?;
        self.rt
            .block_on(client.execute(self.query.as_str().to_string(), ()))?;
        MsSQLSourceParser::new(self.rt.clone(), client, self.schema.len())
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

/// One decoded, native-Rust representation of a single `ColumnValues` cell.
/// Converting eagerly (once, when the row arrives) rather than on every
/// `Produce` call keeps the per-cell conversion logic (decimal, date/time,
/// money math) in one place and lets `&str`/`&[u8]` `Produce` impls borrow
/// directly from this buffer.
#[derive(Debug)]
enum TdsCell {
    Null,
    U8(u8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Str(String),
    Bytes(Vec<u8>),
    Uuid(Uuid),
    Decimal(Decimal),
    NaiveDate(NaiveDate),
    NaiveTime(NaiveTime),
    NaiveDateTime(NaiveDateTime),
    DateTimeUtc(DateTime<Utc>),
}

pub struct MsSQLSourceParser {
    rt: Arc<Runtime>,
    client: Connection,
    rowbuf: Vec<Vec<TdsCell>>,
    ncols: usize,
    current_col: usize,
    current_row: usize,
    is_finished: bool,
}

impl MsSQLSourceParser {
    fn new(rt: Arc<Runtime>, client: Connection, ncols: usize) -> Self {
        Self {
            rt,
            client,
            rowbuf: Vec::with_capacity(DB_BUFFER_SIZE),
            ncols,
            current_row: 0,
            current_col: 0,
            is_finished: false,
        }
    }

    #[throws(MsSQLSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }

    /// Takes ownership of a cell out of the row buffer, leaving `Null`
    /// behind. Safe because each cell is produced exactly once per row (see
    /// `next_loc`).
    fn take_cell(&mut self, ridx: usize, cidx: usize) -> TdsCell {
        std::mem::replace(&mut self.rowbuf[ridx][cidx], TdsCell::Null)
    }
}

impl<'a> PartitionParser<'a> for MsSQLSourceParser {
    type TypeSystem = MsSQLTypeSystem;
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        assert!(self.current_col == 0);
        let remaining_rows = self.rowbuf.len() - self.current_row;
        if remaining_rows > 0 {
            return (remaining_rows, self.is_finished);
        } else if self.is_finished {
            return (0, self.is_finished);
        }

        self.rowbuf.clear();

        for _ in 0..DB_BUFFER_SIZE {
            match self.rt.block_on(self.client.next_row())? {
                Some(row) => {
                    let mut cells = Vec::with_capacity(row.len());
                    for value in row {
                        cells.push(column_value_to_cell(value)?);
                    }
                    self.rowbuf.push(cells);
                }
                None => {
                    self.client.finish()?;
                    self.is_finished = true;
                    break;
                }
            }
        }
        self.current_row = 0;
        self.current_col = 0;
        (self.rowbuf.len(), self.is_finished)
    }
}

#[throws(MsSQLSourceError)]
fn column_value_to_cell(value: ColumnValues) -> TdsCell {
    match value {
        ColumnValues::Null => TdsCell::Null,
        ColumnValues::TinyInt(n) => TdsCell::U8(n),
        ColumnValues::SmallInt(n) => TdsCell::I16(n),
        ColumnValues::Int(n) => TdsCell::I32(n),
        ColumnValues::BigInt(n) => TdsCell::I64(n),
        ColumnValues::Real(f) => TdsCell::F32(f),
        ColumnValues::Float(f) => TdsCell::F64(f),
        ColumnValues::Bit(b) => TdsCell::Bool(b),
        ColumnValues::String(s) => TdsCell::Str(s.to_utf8_string()),
        ColumnValues::Bytes(b) => TdsCell::Bytes(b),
        ColumnValues::Uuid(u) => TdsCell::Uuid(Uuid::from_bytes(*u.as_bytes())),
        ColumnValues::Decimal(parts) | ColumnValues::Numeric(parts) => {
            TdsCell::Decimal(decimal_parts_to_decimal(&parts)?)
        }
        ColumnValues::Date(d) => TdsCell::NaiveDate(sql_date_to_naive_date(&d)?),
        ColumnValues::Time(t) => TdsCell::NaiveTime(sql_time_to_naive_time(&t)),
        ColumnValues::DateTime(dt) => TdsCell::NaiveDateTime(sql_datetime_to_naive_datetime(&dt)?),
        ColumnValues::DateTime2(dt2) => {
            TdsCell::NaiveDateTime(sql_datetime2_to_naive_datetime(&dt2)?)
        }
        ColumnValues::SmallDateTime(sdt) => {
            TdsCell::NaiveDateTime(sql_smalldatetime_to_naive_datetime(&sdt))
        }
        ColumnValues::DateTimeOffset(dto) => {
            TdsCell::DateTimeUtc(sql_datetimeoffset_to_datetime_utc(&dto)?)
        }
        ColumnValues::Money(m) => TdsCell::F64(sql_money_to_f64(&m)),
        // Always widened to f64, never rounded through f32: Tiberius's own
        // money decoder (`money::decode`) computes the value as f64 in every
        // case (`src.read_i32_le() as f64 / 1e4` for the 4-byte form too),
        // regardless of whether the static wire type is the fixed `Money4`
        // or the nullable `MONEYN`. Only the destination typesystem variant
        // (`typesystem.rs`) differs by nullability, and for the nullable
        // case it's `Money`/f64 (see there) - so keep full f64 precision
        // here and let `Produce<f32>` narrow it only if a genuinely
        // non-nullable smallmoney column ever needs it.
        ColumnValues::SmallMoney(sm) => TdsCell::F64((sm.int_val as f64) / 10_000.0),
        other => throw!(anyhow!(
            "MsSQL: unsupported value for src_mssql_tds backend: {:?}",
            other
        )),
    }
}

/// Reassembles a `DecimalParts` value into a `rust_decimal::Decimal` using its
/// public `magnitude()` accessor (already scaled by `10^scale`, unsigned).
///
/// `rust_decimal::Decimal` caps out around a 96-bit mantissa (scale 0-28);
/// this mirrors a limitation ConnectorX's existing Tiberius path already
/// has today (see `utils::decimal_to_i128`), not a new one introduced here.
#[throws(MsSQLSourceError)]
fn decimal_parts_to_decimal(parts: &DecimalParts) -> Decimal {
    let magnitude = parts.magnitude();
    let mantissa = i128::try_from(magnitude)
        .map_err(|_| anyhow!("MsSQL decimal/numeric value overflows 128 bits"))?;
    let mantissa = if parts.is_positive {
        mantissa
    } else {
        -mantissa
    };
    Decimal::try_from_i128_with_scale(mantissa, parts.scale as u32)
        .map_err(|e| anyhow!("MsSQL decimal/numeric value out of range: {}", e))?
}

/// `SqlDate::get_days()` is days since `0001-01-01`, matching chrono's
/// proleptic Gregorian calendar directly (no epoch conversion needed).
#[throws(MsSQLSourceError)]
fn sql_date_to_naive_date(d: &SqlDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(1, 1, 1)
        .unwrap()
        .checked_add_signed(chrono::Duration::days(d.get_days() as i64))
        .ok_or_else(|| {
            anyhow!(
                "MsSQL date out of range: {} days since 0001-01-01",
                d.get_days()
            )
        })?
}

/// Despite its name, `SqlTime::time_nanoseconds` is actually the value in
/// 100-nanosecond ticks since midnight, not raw nanoseconds - confirmed by
/// reading `mssql-tds`'s own `scale_time_value()` decoder helper, which
/// documents "Expands a time value at TDS scale 0..=7 into 100-nanosecond
/// ticks." The field's doc comment ("Nanoseconds since midnight") does not
/// match the actual unit; multiply by 100 to get true nanoseconds.
fn sql_time_to_naive_time(t: &SqlTime) -> NaiveTime {
    let ns = t.time_nanoseconds * 100;
    let secs = (ns / 1_000_000_000) as u32;
    let nanos = (ns % 1_000_000_000) as u32;
    NaiveTime::from_num_seconds_from_midnight_opt(secs, nanos).unwrap_or(NaiveTime::default())
}

#[throws(MsSQLSourceError)]
fn sql_datetime2_to_naive_datetime(dt2: &SqlDateTime2) -> NaiveDateTime {
    let date = NaiveDate::from_ymd_opt(1, 1, 1)
        .unwrap()
        .checked_add_signed(chrono::Duration::days(dt2.days as i64))
        .ok_or_else(|| {
            anyhow!(
                "MsSQL datetime2 out of range: {} days since 0001-01-01",
                dt2.days
            )
        })?;
    date.and_time(sql_time_to_naive_time(&dt2.time))
}

fn sql_smalldatetime_to_naive_datetime(sdt: &SqlSmallDateTime) -> NaiveDateTime {
    // Whole minutes since midnight - no rounding needed.
    let date = NaiveDate::from_ymd_opt(1900, 1, 1)
        .unwrap()
        .checked_add_signed(chrono::Duration::days(sdt.days as i64))
        .expect("smalldatetime day count is always in range (u16 days since 1900-01-01)");
    let time = NaiveTime::from_hms_opt((sdt.time / 60) as u32, (sdt.time % 60) as u32, 0)
        .unwrap_or(NaiveTime::default());
    date.and_time(time)
}

/// The legacy `datetime` type stores 1/300-second ticks since midnight,
/// rounded by SQL Server to the nearest tick when the value was written.
/// Converting back to milliseconds requires the classic
/// `round(ticks * 10 / 3)` algorithm; rounding a tick count near midnight
/// can round up into the next day, which must be reflected in the date.
#[throws(MsSQLSourceError)]
fn sql_datetime_to_naive_datetime(dt: &SqlDateTime) -> NaiveDateTime {
    let total_ms = (dt.time as i64 * 10 + 1) / 3;
    let (extra_days, ms_in_day) = if total_ms >= 86_400_000 {
        (1i64, total_ms - 86_400_000)
    } else {
        (0, total_ms)
    };

    let time = NaiveTime::from_num_seconds_from_midnight_opt(
        (ms_in_day / 1000) as u32,
        ((ms_in_day % 1000) * 1_000_000) as u32,
    )
    .ok_or_else(|| anyhow!("MsSQL datetime: invalid tick count {}", dt.time))?;

    let date = NaiveDate::from_ymd_opt(1900, 1, 1)
        .unwrap()
        .checked_add_signed(chrono::Duration::days(dt.days as i64 + extra_days))
        .ok_or_else(|| {
            anyhow!(
                "MsSQL datetime out of range: {} days since 1900-01-01",
                dt.days
            )
        })?;

    date.and_time(time)
}

#[throws(MsSQLSourceError)]
fn sql_datetimeoffset_to_datetime_utc(dto: &SqlDateTimeOffset) -> DateTime<Utc> {
    // Confirmed against a live container (not the wire-format assumption
    // originally documented here): the datetime2 component of a
    // DATETIMEOFFSET value is transmitted as the original *local* wall-clock
    // time, with `offset` (minutes, e.g. 60 for +01:00) recording how far
    // ahead of UTC it is. UTC = local - offset.
    let naive_local = sql_datetime2_to_naive_datetime(&dto.datetime2)?;
    let naive_utc = naive_local - chrono::Duration::minutes(dto.offset as i64);
    DateTime::<Utc>::from_naive_utc_and_offset(naive_utc, Utc)
}

/// Reassembles the mixed-endian 64-bit integer from `SqlMoney`'s two 32-bit
/// halves and scales it down (money is stored as the value * 10^4).
fn sql_money_to_f64(m: &mssql_tds::datatypes::column_values::SqlMoney) -> f64 {
    let lsb_in_i64 = (m.lsb_part as i64) & 0x0000_0000_FFFF_FFFF;
    let money_val = lsb_in_i64 | ((m.msb_part as i64) << 32);
    (money_val as f64) / 10_000.0
}

macro_rules! impl_produce_direct {
    ($($t:ty => $variant:ident),+ $(,)?) => {
        $(
            impl<'r> Produce<'r, $t> for MsSQLSourceParser {
                type Error = MsSQLSourceError;

                #[throws(MsSQLSourceError)]
                fn produce(&'r mut self) -> $t {
                    let (ridx, cidx) = self.next_loc()?;
                    match self.take_cell(ridx, cidx) {
                        TdsCell::$variant(v) => v,
                        TdsCell::Null => throw!(anyhow!("MsSQL get None at position: ({}, {})", ridx, cidx)),
                        other => throw!(anyhow!("MsSQL type mismatch at ({}, {}): {:?}", ridx, cidx, other)),
                    }
                }
            }

            impl<'r> Produce<'r, Option<$t>> for MsSQLSourceParser {
                type Error = MsSQLSourceError;

                #[throws(MsSQLSourceError)]
                fn produce(&'r mut self) -> Option<$t> {
                    let (ridx, cidx) = self.next_loc()?;
                    match self.take_cell(ridx, cidx) {
                        TdsCell::$variant(v) => Some(v),
                        TdsCell::Null => None,
                        other => throw!(anyhow!("MsSQL type mismatch at ({}, {}): {:?}", ridx, cidx, other)),
                    }
                }
            }
        )+
    };
}

impl_produce_direct!(
    u8 => U8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
    bool => Bool,
    Uuid => Uuid,
    Decimal => Decimal,
    NaiveDateTime => NaiveDateTime,
    NaiveDate => NaiveDate,
    NaiveTime => NaiveTime,
    DateTime<Utc> => DateTimeUtc,
);

/// `f32` needs a hand-written impl (rather than the generic macro) to accept
/// `TdsCell::F64` as well: a genuinely non-nullable `smallmoney` column
/// (fixed `Money4` wire type) maps to the `SmallMoney`/f32 destination, but
/// its value is decoded and stored here as a full-precision `TdsCell::F64`
/// (see `column_value_to_cell`), so it must be narrowed on the way out.
impl<'r> Produce<'r, f32> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> f32 {
        let (ridx, cidx) = self.next_loc()?;
        match self.take_cell(ridx, cidx) {
            TdsCell::F32(v) => v,
            TdsCell::F64(v) => v as f32,
            TdsCell::Null => throw!(anyhow!("MsSQL get None at position: ({}, {})", ridx, cidx)),
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}

impl<'r> Produce<'r, Option<f32>> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> Option<f32> {
        let (ridx, cidx) = self.next_loc()?;
        match self.take_cell(ridx, cidx) {
            TdsCell::F32(v) => Some(v),
            TdsCell::F64(v) => Some(v as f32),
            TdsCell::Null => None,
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}

/// `f64` needs a hand-written impl (rather than the generic macro) because
/// it also has to accept `TdsCell::F32`: a nullable `smallmoney` column
/// decodes its per-row wire value as `ColumnValues::SmallMoney` (4 bytes,
/// stored here as `TdsCell::F32`), yet the typesystem maps it to the same
/// `Money`/f64 destination as regular `money` - matching Tiberius, which
/// always reports its nullable `MONEYN` wire type as F64 regardless of the
/// per-row byte length (see `typesystem.rs`'s `TdsDataType::MoneyN` arm).
impl<'r> Produce<'r, f64> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> f64 {
        let (ridx, cidx) = self.next_loc()?;
        match self.take_cell(ridx, cidx) {
            TdsCell::F64(v) => v,
            TdsCell::F32(v) => v as f64,
            TdsCell::Null => throw!(anyhow!("MsSQL get None at position: ({}, {})", ridx, cidx)),
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}

impl<'r> Produce<'r, Option<f64>> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> Option<f64> {
        let (ridx, cidx) = self.next_loc()?;
        match self.take_cell(ridx, cidx) {
            TdsCell::F64(v) => Some(v),
            TdsCell::F32(v) => Some(v as f64),
            TdsCell::Null => None,
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}

impl<'r> Produce<'r, IntN> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> IntN {
        let (ridx, cidx) = self.next_loc()?;
        match self.take_cell(ridx, cidx) {
            TdsCell::U8(v) => IntN(v as i64),
            TdsCell::I16(v) => IntN(v as i64),
            TdsCell::I32(v) => IntN(v as i64),
            TdsCell::I64(v) => IntN(v),
            TdsCell::Null => throw!(anyhow!("MsSQL get None at position: ({}, {})", ridx, cidx)),
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}

impl<'r> Produce<'r, Option<IntN>> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> Option<IntN> {
        let (ridx, cidx) = self.next_loc()?;
        match self.take_cell(ridx, cidx) {
            TdsCell::U8(v) => Some(IntN(v as i64)),
            TdsCell::I16(v) => Some(IntN(v as i64)),
            TdsCell::I32(v) => Some(IntN(v as i64)),
            TdsCell::I64(v) => Some(IntN(v)),
            TdsCell::Null => None,
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}

impl<'r> Produce<'r, FloatN> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> FloatN {
        let (ridx, cidx) = self.next_loc()?;
        match self.take_cell(ridx, cidx) {
            TdsCell::F32(v) => FloatN(v as f64),
            TdsCell::F64(v) => FloatN(v),
            TdsCell::Null => throw!(anyhow!("MsSQL get None at position: ({}, {})", ridx, cidx)),
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}

impl<'r> Produce<'r, Option<FloatN>> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> Option<FloatN> {
        let (ridx, cidx) = self.next_loc()?;
        match self.take_cell(ridx, cidx) {
            TdsCell::F32(v) => Some(FloatN(v as f64)),
            TdsCell::F64(v) => Some(FloatN(v)),
            TdsCell::Null => None,
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}

impl<'r> Produce<'r, &'r str> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> &'r str {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx] {
            TdsCell::Str(s) => s.as_str(),
            TdsCell::Null => throw!(anyhow!("MsSQL get None at position: ({}, {})", ridx, cidx)),
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}

impl<'r> Produce<'r, Option<&'r str>> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> Option<&'r str> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx] {
            TdsCell::Str(s) => Some(s.as_str()),
            TdsCell::Null => None,
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}

impl<'r> Produce<'r, &'r [u8]> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> &'r [u8] {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx] {
            TdsCell::Bytes(b) => b.as_slice(),
            TdsCell::Null => throw!(anyhow!("MsSQL get None at position: ({}, {})", ridx, cidx)),
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}

impl<'r> Produce<'r, Option<&'r [u8]>> for MsSQLSourceParser {
    type Error = MsSQLSourceError;

    #[throws(MsSQLSourceError)]
    fn produce(&'r mut self) -> Option<&'r [u8]> {
        let (ridx, cidx) = self.next_loc()?;
        match &self.rowbuf[ridx][cidx] {
            TdsCell::Bytes(b) => Some(b.as_slice()),
            TdsCell::Null => None,
            other => throw!(anyhow!(
                "MsSQL type mismatch at ({}, {}): {:?}",
                ridx,
                cidx,
                other
            )),
        }
    }
}
