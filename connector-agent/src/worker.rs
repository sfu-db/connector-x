use crate::data_sources::DataSource;
use crate::errors::ConnectorAgentError;
use crate::types::DataType;
use crate::types::TypeInfo;
use crate::writers::PartitionWriter;
use fehler::throws;

pub struct Worker<S, P> {
    partition_writer: P,
    query: String,
    source: S,
    schema: Vec<DataType>,
}

impl<S, P> Worker<S, P> {
    pub fn new(source: S, writer: P, schema: Vec<DataType>) -> Self {
        Worker {
            partition_writer: writer,
            query: "".to_string(),
            source,
            schema,
        }
    }
}

impl<'a, S, P> Worker<S, P>
where
    P: PartitionWriter<'a>,
    S: DataSource,
{
    #[throws(ConnectorAgentError)]
    pub fn run(mut self) {
        self.source.run_query(&self.query)?;

        let funcs: Vec<_> = self
            .schema
            .iter()
            .map(|ty| match ty {
                DataType::F64 => pipe::<S, P, f64>,
                DataType::U64 => pipe::<S, P, u64>,
            })
            .collect();

        for row in 0..self.partition_writer.nrows() {
            for col in 0..self.partition_writer.ncols() {
                funcs[col](&mut self.source, &mut self.partition_writer, row, col)?;
            }
        }
    }

    #[throws(ConnectorAgentError)]
    pub fn run_safe(mut self) {
        self.source.run_query(&self.query)?;

        let funcs: Vec<_> = self
            .schema
            .iter()
            .map(|ty| match ty {
                DataType::F64 => pipe_safe::<S, P, f64>,
                DataType::U64 => pipe_safe::<S, P, u64>,
            })
            .collect();

        for row in 0..self.partition_writer.nrows() {
            for col in 0..self.partition_writer.ncols() {
                funcs[col](&mut self.source, &mut self.partition_writer, row, col)?;
            }
        }
    }
}

#[throws(ConnectorAgentError)]
fn pipe<'a, S, W, T>(source: &mut S, writer: &mut W, row: usize, col: usize)
where
    S: DataSource,
    W: PartitionWriter<'a>,
    T: TypeInfo,
{
    unsafe { writer.write(row, col, source.produce::<T>()?) }
}

#[throws(ConnectorAgentError)]
fn pipe_safe<'a, S, W, T>(source: &mut S, writer: &mut W, row: usize, col: usize)
where
    S: DataSource,
    W: PartitionWriter<'a>,
    T: TypeInfo,
{
    writer.write_safe(row, col, source.produce::<T>()?)?
}
