//! Source implementation for ClickHouse using the native protocol.

mod errors;
mod typesystem;

pub use self::errors::ClickHouseSourceError;
pub use self::typesystem::{ClickHouseTypeSystem, TypeMetadata};

use crate::{
    data_order::DataOrder,
    errors::ConnectorXError,
    sources::{
        clickhouse::typesystem::DataType, PartitionParser, Produce, Source, SourcePartition,
    },
    sql::{count_query, limit1_query, CXQuery},
};
use anyhow::anyhow;
use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};
use chrono_tz::Tz;
use clickhouse::Client;
use fehler::{throw, throws};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use sqlparser::dialect::{ClickHouseDialect, GenericDialect};
use std::io::{Cursor, Read};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::sync::Arc;
use tokio::runtime::Runtime;
use uuid::Uuid;

/// ClickHouse source that uses the HTTP protocol.
pub struct ClickHouseSource {
    rt: Arc<Runtime>,
    pub client: Client,
    origin_query: Option<String>,
    queries: Vec<CXQuery<String>>,
    names: Vec<String>,
    schema: Vec<ClickHouseTypeSystem>,
    metadata: Vec<TypeMetadata>,
}

impl ClickHouseSource {
    #[throws(ClickHouseSourceError)]
    pub fn new(rt: Arc<Runtime>, conn: &str) -> Self {
        let url = url::Url::parse(conn)?;

        let use_https = url
            .query_pairs()
            .find(|(k, v)| k == "protocol" && v == "https")
            .is_some();

        let base_url = format!(
            "{}://{}:{}",
            if use_https { "https" } else { "http" },
            url.host_str().unwrap_or("localhost"),
            url.port().unwrap_or(8123)
        );

        let mut client = Client::default().with_url(&base_url);

        let database = url.path().trim_start_matches('/');
        if !database.is_empty() {
            client = client.with_database(database);
        }

        let username = url.username();
        if !username.is_empty() {
            client = client.with_user(username);
        }

        let password = url.password().unwrap_or("");
        if !password.is_empty() {
            client = client.with_password(password);
        }

        Self {
            rt,
            client,
            origin_query: None,
            queries: vec![],
            names: vec![],
            schema: vec![],
            metadata: vec![],
        }
    }
}

impl Source for ClickHouseSource
where
    ClickHouseSourcePartition:
        SourcePartition<TypeSystem = ClickHouseTypeSystem, Error = ClickHouseSourceError>,
{
    const DATA_ORDERS: &'static [DataOrder] = &[DataOrder::RowMajor];
    type Partition = ClickHouseSourcePartition;
    type TypeSystem = ClickHouseTypeSystem;
    type Error = ClickHouseSourceError;

    #[throws(ClickHouseSourceError)]
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

    #[throws(ClickHouseSourceError)]
    fn fetch_metadata(&mut self) {
        assert!(!self.queries.is_empty());

        let first_query = &self.queries[0];
        let l1query = limit1_query(first_query, &ClickHouseDialect {})?;

        let describe_query = format!("DESCRIBE ({})", l1query.as_str());

        let response = self.rt.block_on(async {
            let mut cursor = self
                .client
                .query(&describe_query)
                .fetch_bytes("JSONCompact")
                .map_err(|e| anyhow!("ClickHouse error: {}", e))?;
            let bytes = cursor
                .collect()
                .await
                .map_err(|e| anyhow!("ClickHouse error: {}", e))?;
            Ok::<_, ClickHouseSourceError>(bytes)
        })?;

        #[derive(Debug, Deserialize)]
        struct DescribeResponse {
            data: Vec<Vec<JsonValue>>,
        }

        let parsed: DescribeResponse = serde_json::from_slice(&response)
            .map_err(|e| anyhow!("Failed to parse DESCRIBE response: {}", e))?;

        let mut names = Vec::new();
        let mut types = Vec::new();
        let mut metadata = Vec::new();

        for row in parsed.data {
            if row.len() >= 2 {
                let name = row[0].as_str().unwrap_or("").to_string();
                let type_str = row[1].as_str().unwrap_or("String");
                let (ts, meta) = ClickHouseTypeSystem::from_type_str_with_metadata(type_str);
                names.push(name);
                types.push(ts);
                metadata.push(meta);
            }
        }

        self.names = names;
        self.schema = types;
        self.metadata = metadata;
    }

    #[throws(ClickHouseSourceError)]
    fn result_rows(&mut self) -> Option<usize> {
        match &self.origin_query {
            Some(q) => {
                let cxq = CXQuery::Naked(q.clone());
                let cquery = count_query(&cxq, &ClickHouseDialect {})?;

                let response = self.rt.block_on(async {
                    let mut cursor = self
                        .client
                        .query(cquery.as_str())
                        .fetch_bytes("JSONCompact")
                        .map_err(|e| anyhow!("ClickHouse error: {}", e))?;
                    let bytes = cursor
                        .collect()
                        .await
                        .map_err(|e| anyhow!("ClickHouse error: {}", e))?;
                    Ok::<_, ClickHouseSourceError>(bytes)
                })?;

                #[derive(Debug, Deserialize)]
                struct CountResponse {
                    data: Vec<Vec<JsonValue>>,
                }

                let parsed: CountResponse = serde_json::from_slice(&response)
                    .map_err(|e| anyhow!("Failed to parse count response: {}", e))?;

                if let Some(row) = parsed.data.first() {
                    if let Some(count_val) = row.first() {
                        let count = match count_val {
                            JsonValue::Number(n) => n.as_u64().unwrap_or(0),
                            JsonValue::String(s) => s.parse().unwrap_or(0),
                            _ => 0,
                        };
                        return Some(count as usize);
                    }
                }
                None
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

    #[throws(ClickHouseSourceError)]
    fn partition(self) -> Vec<Self::Partition> {
        let mut ret = vec![];
        for query in self.queries {
            ret.push(ClickHouseSourcePartition::new(
                self.rt.clone(),
                self.client.clone(),
                &query,
                &self.schema,
                &self.metadata,
            ));
        }
        ret
    }
}

pub struct ClickHouseSourcePartition {
    rt: Arc<Runtime>,
    client: Client,
    query: CXQuery<String>,
    schema: Vec<ClickHouseTypeSystem>,
    metadata: Vec<TypeMetadata>,
    nrows: usize,
    ncols: usize,
}

impl ClickHouseSourcePartition {
    pub fn new(
        rt: Arc<Runtime>,
        client: Client,
        query: &CXQuery<String>,
        schema: &[ClickHouseTypeSystem],
        metadata: &[TypeMetadata],
    ) -> Self {
        Self {
            rt,
            client,
            query: query.clone(),
            schema: schema.to_vec(),
            metadata: metadata.to_vec(),
            nrows: 0,
            ncols: schema.len(),
        }
    }
}

impl SourcePartition for ClickHouseSourcePartition {
    type TypeSystem = ClickHouseTypeSystem;
    type Parser<'a> = ClickHouseSourceParser<'a>;
    type Error = ClickHouseSourceError;

    #[throws(ClickHouseSourceError)]
    fn result_rows(&mut self) {
        let cquery = count_query(&self.query, &GenericDialect {})?;

        let response = self.rt.block_on(async {
            let mut cursor = self
                .client
                .query(cquery.as_str())
                .fetch_bytes("JSONCompact")
                .map_err(|e| anyhow!("ClickHouse error: {}", e))?;
            let bytes = cursor
                .collect()
                .await
                .map_err(|e| anyhow!("ClickHouse error: {}", e))?;
            Ok::<_, ClickHouseSourceError>(bytes)
        })?;

        #[derive(Debug, Deserialize)]
        struct CountResponse {
            data: Vec<Vec<JsonValue>>,
        }

        let parsed: CountResponse = serde_json::from_slice(&response)
            .map_err(|e| anyhow!("Failed to parse count response: {}", e))?;

        if let Some(row) = parsed.data.first() {
            if let Some(count_val) = row.first() {
                let count = match count_val {
                    JsonValue::Number(n) => n.as_u64().unwrap_or(0),
                    JsonValue::String(s) => s.parse().unwrap_or(0),
                    _ => 0,
                };
                self.nrows = count as usize;
            }
        }
    }

    #[throws(ClickHouseSourceError)]
    fn parser(&mut self) -> Self::Parser<'_> {
        ClickHouseSourceParser::new(
            self.rt.clone(),
            self.client.clone(),
            self.query.clone(),
            &self.schema,
            &self.metadata,
        )?
    }

    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

struct BinaryReader<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> BinaryReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], ClickHouseSourceError> {
        let mut buf = [0u8; N];
        self.cursor
            .read_exact(&mut buf)
            .map_err(|e| anyhow!("Failed to read {} bytes: {}", N, e))?;
        Ok(buf)
    }

    fn read_u8(&mut self) -> Result<u8, ClickHouseSourceError> {
        Ok(self.read_bytes::<1>()?[0])
    }

    fn read_i8(&mut self) -> Result<i8, ClickHouseSourceError> {
        Ok(self.read_u8()? as i8)
    }

    fn read_u16(&mut self) -> Result<u16, ClickHouseSourceError> {
        Ok(u16::from_le_bytes(self.read_bytes()?))
    }

    fn read_i16(&mut self) -> Result<i16, ClickHouseSourceError> {
        Ok(i16::from_le_bytes(self.read_bytes()?))
    }

    fn read_u32(&mut self) -> Result<u32, ClickHouseSourceError> {
        Ok(u32::from_le_bytes(self.read_bytes()?))
    }

    fn read_i32(&mut self) -> Result<i32, ClickHouseSourceError> {
        Ok(i32::from_le_bytes(self.read_bytes()?))
    }

    fn read_u64(&mut self) -> Result<u64, ClickHouseSourceError> {
        Ok(u64::from_le_bytes(self.read_bytes()?))
    }

    fn read_i64(&mut self) -> Result<i64, ClickHouseSourceError> {
        Ok(i64::from_le_bytes(self.read_bytes()?))
    }

    fn read_f32(&mut self) -> Result<f32, ClickHouseSourceError> {
        Ok(f32::from_le_bytes(self.read_bytes()?))
    }

    fn read_f64(&mut self) -> Result<f64, ClickHouseSourceError> {
        Ok(f64::from_le_bytes(self.read_bytes()?))
    }

    fn read_varint(&mut self) -> Result<u64, ClickHouseSourceError> {
        let mut result: u64 = 0;
        let mut shift = 0;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7f) as u64) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= 64 {
                return Err(anyhow!("Varint too long").into());
            }
        }
        Ok(result)
    }

    fn read_fixed_string(&mut self, len: usize) -> Result<Vec<u8>, ClickHouseSourceError> {
        let mut buf = vec![0u8; len];
        self.cursor
            .read_exact(&mut buf)
            .map_err(|e| anyhow!("Failed to read FixedString: {}", e))?;
        Ok(buf)
    }

    fn read_string(&mut self) -> Result<String, ClickHouseSourceError> {
        let len = self.read_varint()? as usize;
        let mut buf = vec![0u8; len];
        self.cursor
            .read_exact(&mut buf)
            .map_err(|e| anyhow!("Failed to read string: {}", e))?;
        String::from_utf8(buf).map_err(|e| anyhow!("Invalid UTF-8: {}", e).into())
    }

    fn read_uuid(&mut self) -> Result<Uuid, ClickHouseSourceError> {
        // ClickHouse stores UUID as two UInt64 in big-endian order
        let high = self.read_u64()?;
        let low = self.read_u64()?;

        Ok(Uuid::from_u64_pair(high, low))
    }

    fn read_bool(&mut self) -> Result<bool, ClickHouseSourceError> {
        Ok(self.read_u8()? != 0)
    }

    fn read_date(&mut self) -> Result<NaiveDate, ClickHouseSourceError> {
        // ClickHouse stores Date as UInt16 representing days since 1970-01-01
        let days = self.read_u16()? as i64;
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        epoch
            .checked_add_signed(Duration::days(days))
            .ok_or_else(|| anyhow!("Invalid date value: {} days since epoch", days).into())
    }

    fn read_date32(&mut self) -> Result<NaiveDate, ClickHouseSourceError> {
        // ClickHouse stores Date32 as Int32 representing days since 1970-01-01
        // negative values represent dates before 1970-01-01
        let days = self.read_i32()? as i64;
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        epoch
            .checked_add_signed(Duration::days(days))
            .ok_or_else(|| anyhow!("Invalid date32 value: {} days since epoch", days).into())
    }

    fn read_datetime(&mut self, tz: Option<&Tz>) -> Result<DateTime<Utc>, ClickHouseSourceError> {
        let seconds = self.read_u32()? as i64;
        chrono::DateTime::from_timestamp(seconds, 0)
            .map(|dt| dt.with_timezone(tz.unwrap_or(&Tz::UTC)).to_utc())
            .ok_or_else(|| {
                anyhow!("Invalid datetime value: {} seconds since epoch", seconds).into()
            })
    }

    fn read_datetime64(
        &mut self,
        precision: u8,
        tz: Option<&Tz>,
    ) -> Result<DateTime<Utc>, ClickHouseSourceError> {
        if precision > 9 {
            return Err(anyhow!("Unsupported DateTime64 precision: {}", precision).into());
        }
        let ticks = self.read_i64()?;
        let nanos = ticks * 10_i64.pow(9 - precision as u32);

        Ok(DateTime::from_timestamp_nanos(nanos)
            .with_timezone(tz.unwrap_or(&Tz::UTC))
            .to_utc())
    }

    fn read_decimal(&mut self, precision: u8, scale: u8) -> Result<Decimal, ClickHouseSourceError> {
        match precision {
            1..=9 => {
                let value = self.read_i32()?;
                Ok(Decimal::new(value as i64, scale as u32))
            }
            10..=18 => {
                let value = self.read_i64()?;
                Ok(Decimal::new(value, scale as u32))
            }
            _ => Err(anyhow!("Unsupported Decimal precision: {}", precision).into()),
        }
    }

    fn read_time(&mut self) -> Result<NaiveTime, ClickHouseSourceError> {
        let seconds = self.read_u32()? as i64;
        Ok(
            NaiveTime::from_num_seconds_from_midnight_opt(seconds as u32, 0)
                .ok_or_else(|| anyhow!("Invalid time value: {} seconds since midnight", seconds))?,
        )
    }

    fn read_time64(&mut self, precision: u8) -> Result<NaiveTime, ClickHouseSourceError> {
        if precision > 9 {
            return Err(anyhow!("Unsupported Time64 precision: {}", precision).into());
        }
        let ticks = self.read_i64()?;
        let nanos = ticks * 10_i64.pow(9 - precision as u32);
        Ok(NaiveTime::from_num_seconds_from_midnight_opt(
            (nanos / 1_000_000_000) as u32,
            (nanos % 1_000_000_000) as u32,
        )
        .ok_or_else(|| anyhow!("Invalid time64 value: {} ticks since midnight", ticks))?)
    }

    fn read_ipv4(&mut self) -> Result<IpAddr, ClickHouseSourceError> {
        let bytes = self.read_u32()?;

        Ok(IpAddr::V4(Ipv4Addr::from_bits(bytes)))
    }

    fn read_ipv6(&mut self) -> Result<IpAddr, ClickHouseSourceError> {
        let seg1 = u16::from_be(self.read_u16()?);
        let seg2 = u16::from_be(self.read_u16()?);
        let seg3 = u16::from_be(self.read_u16()?);
        let seg4 = u16::from_be(self.read_u16()?);
        let seg5 = u16::from_be(self.read_u16()?);
        let seg6 = u16::from_be(self.read_u16()?);
        let seg7 = u16::from_be(self.read_u16()?);
        let seg8 = u16::from_be(self.read_u16()?);

        Ok(IpAddr::V6(Ipv6Addr::from_segments([
            seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8,
        ])))
    }

    fn read_enum8(&mut self) -> Result<i8, ClickHouseSourceError> {
        self.read_i8()
    }

    fn read_enum16(&mut self) -> Result<i16, ClickHouseSourceError> {
        self.read_i16()
    }

    fn read_array<T, F>(&mut self, read_elem: F) -> Result<Vec<Option<T>>, ClickHouseSourceError>
    where
        F: Fn(&mut Self) -> Result<T, ClickHouseSourceError>,
    {
        let len = self.read_varint()? as usize;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(Some(read_elem(self)?));
        }
        Ok(result)
    }

    fn is_empty(&self) -> bool {
        self.cursor.position() as usize >= self.cursor.get_ref().len()
    }
}

pub struct ClickHouseSourceParser<'a> {
    rt: Arc<Runtime>,
    client: Client,
    query: CXQuery<String>,
    schema: Vec<ClickHouseTypeSystem>,
    metadata: Vec<TypeMetadata>,
    rowbuf: Vec<Vec<DataType>>,
    ncols: usize,
    current_row: usize,
    current_col: usize,
    is_finished: bool,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> ClickHouseSourceParser<'a> {
    #[throws(ClickHouseSourceError)]
    pub fn new(
        rt: Arc<Runtime>,
        client: Client,
        query: CXQuery<String>,
        schema: &[ClickHouseTypeSystem],
        metadata: &[TypeMetadata],
    ) -> Self {
        Self {
            rt,
            client,
            query,
            schema: schema.to_vec(),
            metadata: metadata.to_vec(),
            rowbuf: Vec::new(),
            ncols: schema.len(),
            current_row: 0,
            current_col: 0,
            is_finished: false,
            _phantom: std::marker::PhantomData,
        }
    }

    fn parse_row_binary(
        &self,
        reader: &mut BinaryReader,
    ) -> Result<Vec<DataType>, ClickHouseSourceError> {
        let mut row = Vec::with_capacity(self.ncols);

        for (col_idx, col_type) in self.schema.iter().enumerate() {
            let is_nullable = col_type.is_nullable();
            let meta = &self.metadata[col_idx];

            if is_nullable {
                let null_flag = reader.read_u8()?;
                if null_flag == 1 {
                    row.push(DataType::Null);
                    continue;
                }
            }

            let value = match col_type {
                ClickHouseTypeSystem::Int8(_) => DataType::Int8(reader.read_i8()?),
                ClickHouseTypeSystem::Int16(_) => DataType::Int16(reader.read_i16()?),
                ClickHouseTypeSystem::Int32(_) => DataType::Int32(reader.read_i32()?),
                ClickHouseTypeSystem::Int64(_) => DataType::Int64(reader.read_i64()?),
                ClickHouseTypeSystem::UInt8(_) => DataType::UInt8(reader.read_u8()?),
                ClickHouseTypeSystem::UInt16(_) => DataType::UInt16(reader.read_u16()?),
                ClickHouseTypeSystem::UInt32(_) => DataType::UInt32(reader.read_u32()?),
                ClickHouseTypeSystem::UInt64(_) => DataType::UInt64(reader.read_u64()?),

                ClickHouseTypeSystem::Float32(_) => DataType::Float32(reader.read_f32()?),
                ClickHouseTypeSystem::Float64(_) => DataType::Float64(reader.read_f64()?),

                ClickHouseTypeSystem::Decimal(_) => {
                    DataType::Decimal(reader.read_decimal(meta.precision, meta.scale)?)
                }

                ClickHouseTypeSystem::String(_) => DataType::String(reader.read_string()?),

                ClickHouseTypeSystem::FixedString(_) => {
                    DataType::FixedString(reader.read_fixed_string(meta.length)?)
                }

                ClickHouseTypeSystem::Date(_) => DataType::Date(reader.read_date()?),
                ClickHouseTypeSystem::Date32(_) => DataType::Date32(reader.read_date32()?),

                ClickHouseTypeSystem::DateTime(_) => {
                    DataType::DateTime(reader.read_datetime(meta.timezone.as_ref())?)
                }
                ClickHouseTypeSystem::DateTime64(_) => DataType::DateTime64(
                    reader.read_datetime64(meta.precision, meta.timezone.as_ref())?,
                ),

                ClickHouseTypeSystem::Time(_) => DataType::Time(reader.read_time()?),
                ClickHouseTypeSystem::Time64(_) => {
                    DataType::Time64(reader.read_time64(meta.precision)?)
                }

                ClickHouseTypeSystem::Enum8(_) => {
                    let enum_value = reader.read_enum8()?;
                    let enum_str = meta
                        .named_values
                        .as_ref()
                        .and_then(|h| h.get(&(enum_value as i16)));
                    DataType::Enum8(enum_str.cloned().unwrap_or_default())
                }
                ClickHouseTypeSystem::Enum16(_) => {
                    let enum_value = reader.read_enum16()?;
                    let enum_str = meta.named_values.as_ref().and_then(|h| h.get(&enum_value));
                    DataType::Enum16(enum_str.cloned().unwrap_or_default())
                }

                ClickHouseTypeSystem::UUID(_) => DataType::UUID(reader.read_uuid()?),

                ClickHouseTypeSystem::IPv4(_) => DataType::IPv4(reader.read_ipv4()?),
                ClickHouseTypeSystem::IPv6(_) => DataType::IPv6(reader.read_ipv6()?),

                ClickHouseTypeSystem::Bool(_) => DataType::Bool(reader.read_bool()?),

                ClickHouseTypeSystem::ArrayBool(_) => {
                    DataType::ArrayBool(reader.read_array(BinaryReader::read_bool)?)
                }
                ClickHouseTypeSystem::ArrayString(_) => {
                    DataType::ArrayString(reader.read_array(BinaryReader::read_string)?)
                }
                ClickHouseTypeSystem::ArrayInt8(_) => {
                    DataType::ArrayInt8(reader.read_array(BinaryReader::read_i8)?)
                }
                ClickHouseTypeSystem::ArrayInt16(_) => {
                    DataType::ArrayInt16(reader.read_array(BinaryReader::read_i16)?)
                }
                ClickHouseTypeSystem::ArrayInt32(_) => {
                    DataType::ArrayInt32(reader.read_array(BinaryReader::read_i32)?)
                }
                ClickHouseTypeSystem::ArrayInt64(_) => {
                    DataType::ArrayInt64(reader.read_array(BinaryReader::read_i64)?)
                }
                ClickHouseTypeSystem::ArrayUInt8(_) => {
                    DataType::ArrayUInt8(reader.read_array(BinaryReader::read_u8)?)
                }
                ClickHouseTypeSystem::ArrayUInt16(_) => {
                    DataType::ArrayUInt16(reader.read_array(BinaryReader::read_u16)?)
                }
                ClickHouseTypeSystem::ArrayUInt32(_) => {
                    DataType::ArrayUInt32(reader.read_array(BinaryReader::read_u32)?)
                }
                ClickHouseTypeSystem::ArrayUInt64(_) => {
                    DataType::ArrayUInt64(reader.read_array(BinaryReader::read_u64)?)
                }
                ClickHouseTypeSystem::ArrayFloat32(_) => {
                    DataType::ArrayFloat32(reader.read_array(BinaryReader::read_f32)?)
                }
                ClickHouseTypeSystem::ArrayFloat64(_) => {
                    DataType::ArrayFloat64(reader.read_array(BinaryReader::read_f64)?)
                }
                ClickHouseTypeSystem::ArrayDecimal(_) => {
                    let precision = meta.precision;
                    let scale = meta.scale;
                    DataType::ArrayDecimal(reader.read_array(|r| r.read_decimal(precision, scale))?)
                }
            };

            row.push(value);
        }

        Ok(row)
    }

    #[throws(ClickHouseSourceError)]
    fn next_loc(&mut self) -> (usize, usize) {
        let ret = (self.current_row, self.current_col);
        self.current_row += (self.current_col + 1) / self.ncols;
        self.current_col = (self.current_col + 1) % self.ncols;
        ret
    }
}

impl<'a> PartitionParser<'a> for ClickHouseSourceParser<'a> {
    type TypeSystem = ClickHouseTypeSystem;
    type Error = ClickHouseSourceError;

    #[throws(ClickHouseSourceError)]
    fn fetch_next(&mut self) -> (usize, bool) {
        assert!(self.current_col == 0);

        if self.is_finished {
            return (0, true);
        }

        let response = self.rt.block_on(async {
            let mut cursor = self
                .client
                .query(self.query.as_str())
                .fetch_bytes("RowBinary")
                .map_err(|e| anyhow!("ClickHouse error: {}", e))?;
            let bytes = cursor
                .collect()
                .await
                .map_err(|e| anyhow!("ClickHouse error: {}", e))?;
            Ok::<_, ClickHouseSourceError>(bytes)
        })?;
        let mut reader = BinaryReader::new(&response);
        let mut rows = Vec::new();

        while !reader.is_empty() {
            match self.parse_row_binary(&mut reader) {
                Ok(row) => rows.push(row),
                Err(_) => break,
            }
        }

        self.rowbuf = rows;
        self.current_row = 0;
        self.is_finished = true;

        (self.rowbuf.len(), true)
    }
}

macro_rules! impl_produce {
    ($rust_type:ty, [$($variant:ident),+]) => {
        impl<'r, 'a> Produce<'r, $rust_type> for ClickHouseSourceParser<'a> {
            type Error = ClickHouseSourceError;

            #[throws(ClickHouseSourceError)]
            fn produce(&'r mut self) -> $rust_type {
                let (ridx, cidx) = self.next_loc()?;
                let value = &self.rowbuf[ridx][cidx];

                match value {
                    $(DataType::$variant(v) => *v as $rust_type,)+
                    _ => throw!(ConnectorXError::cannot_produce::<$rust_type>(Some(
                        format!("{:?}", value)
                    ))),
                }
            }
        }

        impl<'r, 'a> Produce<'r, Option<$rust_type>> for ClickHouseSourceParser<'a> {
            type Error = ClickHouseSourceError;

            #[throws(ClickHouseSourceError)]
            fn produce(&'r mut self) -> Option<$rust_type> {
                let (ridx, cidx) = self.next_loc()?;
                let value = &self.rowbuf[ridx][cidx];

                match value {
                    DataType::Null => None,
                    $(DataType::$variant(v) => Some(*v as $rust_type),)+
                    _ => throw!(ConnectorXError::cannot_produce::<$rust_type>(Some(
                        format!("{:?}", value)
                    ))),
                }
            }
        }
    };
}

macro_rules! impl_produce_with_clone {
    ($rust_type:ty, [$($variant:ident),+]) => {
        impl<'r, 'a> Produce<'r, $rust_type> for ClickHouseSourceParser<'a> {
            type Error = ClickHouseSourceError;

            #[throws(ClickHouseSourceError)]
            fn produce(&'r mut self) -> $rust_type {
                let (ridx, cidx) = self.next_loc()?;
                let value = &self.rowbuf[ridx][cidx];

                match value {
                    $(DataType::$variant(v) => v.clone() as $rust_type,)+
                    _ => throw!(ConnectorXError::cannot_produce::<$rust_type>(Some(
                        format!("{:?}", value)
                    ))),
                }
            }
        }

        impl<'r, 'a> Produce<'r, Option<$rust_type>> for ClickHouseSourceParser<'a> {
            type Error = ClickHouseSourceError;

            #[throws(ClickHouseSourceError)]
            fn produce(&'r mut self) -> Option<$rust_type> {
                let (ridx, cidx) = self.next_loc()?;
                let value = &self.rowbuf[ridx][cidx];

                match value {
                    DataType::Null => None,
                    $(DataType::$variant(v) => Some(v.clone() as $rust_type),)+
                    _ => throw!(ConnectorXError::cannot_produce::<$rust_type>(Some(
                        format!("{:?}", value)
                    ))),
                }
            }
        }
    };
}

macro_rules! impl_produce_vec {
    ($rust_type:ty, [$($variant:ident),+]) => {
        impl<'r, 'a> Produce<'r, Vec<Option<$rust_type>>> for ClickHouseSourceParser<'a> {
            type Error = ClickHouseSourceError;

            #[throws(ClickHouseSourceError)]
            fn produce(&'r mut self) -> Vec<Option<$rust_type>> {
                let (ridx, cidx) = self.next_loc()?;
                let value = &self.rowbuf[ridx][cidx];

                match value {
                    $(DataType::$variant(v) => v.clone(),)+
                    _ => throw!(ConnectorXError::cannot_produce::<Vec<Option<$rust_type>>>(Some(
                        format!("{:?}", value)
                    ))),
                }
            }
        }

        impl<'r, 'a> Produce<'r, Option<Vec<Option<$rust_type>>>> for ClickHouseSourceParser<'a> {
            type Error = ClickHouseSourceError;

            #[throws(ClickHouseSourceError)]
            fn produce(&'r mut self) -> Option<Vec<Option<$rust_type>>> {
                let (ridx, cidx) = self.next_loc()?;
                let value = &self.rowbuf[ridx][cidx];

                match value {
                    DataType::Null => None,
                    $(DataType::$variant(v) => Some(v.clone()),)+
                    _ => throw!(ConnectorXError::cannot_produce::<Option<Vec<Option<$rust_type>>>>(Some(
                        format!("{:?}", value)
                    ))),
                }
            }
        }
    };
}

impl_produce!(i8, [Int8]);
impl_produce!(i16, [Int16]);
impl_produce!(i32, [Int32]);
impl_produce!(i64, [Int64]);
impl_produce!(u8, [UInt8]);
impl_produce!(u16, [UInt16]);
impl_produce!(u32, [UInt32]);
impl_produce!(u64, [UInt64]);
impl_produce!(f32, [Float32]);
impl_produce!(f64, [Float64]);
impl_produce!(Decimal, [Decimal]);
impl_produce_with_clone!(String, [String, Enum8, Enum16]);
impl_produce_with_clone!(Vec<u8>, [FixedString]);
impl_produce!(NaiveDate, [Date, Date32]);
impl_produce!(DateTime<Utc>, [DateTime, DateTime64]);
impl_produce!(NaiveTime, [Time, Time64]);
impl_produce!(Uuid, [UUID]);
impl_produce!(IpAddr, [IPv4, IPv6]);
impl_produce!(bool, [Bool]);
impl_produce_vec!(bool, [ArrayBool]);
impl_produce_vec!(String, [ArrayString]);
impl_produce_vec!(i8, [ArrayInt8]);
impl_produce_vec!(i16, [ArrayInt16]);
impl_produce_vec!(i32, [ArrayInt32]);
impl_produce_vec!(i64, [ArrayInt64]);
impl_produce_vec!(u8, [ArrayUInt8]);
impl_produce_vec!(u16, [ArrayUInt16]);
impl_produce_vec!(u32, [ArrayUInt32]);
impl_produce_vec!(u64, [ArrayUInt64]);
impl_produce_vec!(f32, [ArrayFloat32]);
impl_produce_vec!(f64, [ArrayFloat64]);
impl_produce_vec!(Decimal, [ArrayDecimal]);
