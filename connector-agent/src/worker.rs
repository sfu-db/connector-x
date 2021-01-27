use crate::data_sources::DataSource;
use crate::errors::Result;
use crate::typesystem::Transmit;
use crate::writers::PartitionWriter;

/// A worker owns a `DataSource` `S` and a `PartitionWriter` `W`. It is also parameterized on a
/// `TypeSystem` TS.
pub struct Worker<S, P, TS> {
    partition_writer: P,
    query: String,
    source: S,
    schema: Vec<TS>,
}

impl<S, P, TS> Worker<S, P, TS> {
    /// Create a new worker by providing the data source, partitioned writer, a schema and the query
    /// to be issued to the data source.
    pub fn new(source: S, writer: P, schema: Vec<TS>, query: &str) -> Self {
        Worker {
            partition_writer: writer,
            query: query.to_string(),
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
    /// Run the worker, this pulls the data from the data source `S` and pump that into the writer `W`.
    pub fn run(self) -> Result<()> {
        self.entry(false)
    }

    /// Same as `Worker::run`, but with additional schema check.
    pub fn run_checked(self) -> Result<()> {
        self.entry(true)
    }

    fn entry(mut self, checked: bool) -> Result<()> {
        self.source.run_query(&self.query)?;

        let funcs: Vec<_> = self
            .schema
            .iter()
            .map(|ty| {
                if checked {
                    ty.transmit_checked()
                } else {
                    ty.transmit()
                }
            })
            .collect();

        for row in 0..self.partition_writer.nrows() {
            for col in 0..self.partition_writer.ncols() {
                funcs[col](&mut self.source, &mut self.partition_writer, row, col)?;
            }
        }

        Ok(())
    }
}
