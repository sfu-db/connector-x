use crate::data_sources::DataSource;
use crate::errors::Result;
use crate::types::Transmit;
use crate::writers::PartitionWriter;

pub struct Worker<S, P, TS> {
    partition_writer: P,
    query: String,
    source: S,
    schema: Vec<TS>,
}

impl<S, P, TS> Worker<S, P, TS> {
    pub fn new(source: S, writer: P, schema: Vec<TS>) -> Self {
        Worker {
            partition_writer: writer,
            query: "".to_string(),
            source,
            schema,
        }
    }
}

impl<'a, S, P, TS> Worker<S, P, TS>
where
    P: PartitionWriter<'a, TypeSystem = TS>,
    S: DataSource<TypeSystem = TS>,
    TS: Transmit<S, P>,
{
    pub fn run(mut self) -> Result<()> {
        self.source.run_query(&self.query)?;

        let funcs: Vec<_> = self.schema.iter().map(|ty| ty.transmit()).collect();

        for row in 0..self.partition_writer.nrows() {
            for col in 0..self.partition_writer.ncols() {
                funcs[col](&mut self.source, &mut self.partition_writer, row, col)?;
            }
        }

        Ok(())
    }

    pub fn run_checked(mut self) -> Result<()> {
        self.source.run_query(&self.query)?;

        let funcs: Vec<_> = self.schema.iter().map(|ty| ty.transmit_checked()).collect();

        for row in 0..self.partition_writer.nrows() {
            for col in 0..self.partition_writer.ncols() {
                funcs[col](&mut self.source, &mut self.partition_writer, row, col)?;
            }
        }

        Ok(())
    }
}
