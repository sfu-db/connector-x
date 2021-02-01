use crate::data_sources::{DataSource, SourceBuilder};
use crate::errors::Result;
use crate::typesystem::Transmit;
use crate::writers::{PartitionWriter, Writer};
use rayon::prelude::*;

/// A dispatcher owns a `SourceBuilder` `SB` and a vector of `queries`
/// `schema` is a temporary input before we implement infer schema or get schema from DB.
pub struct Dispatcher<SB, TS> {
    source_builder: SB,
    schema: Vec<TS>,
    queries: Vec<String>,
}

impl<SB, TS> Dispatcher<SB, TS> {
    /// Create a new dispatcher by providing a source builder, schema (temporary) and the queries
    /// to be issued to the data source.
    pub fn new(source_builder: SB, schema: Vec<TS>, queries: Vec<String>) -> Self {
        Dispatcher {
            source_builder,
            schema,
            queries,
        }
    }

    pub fn run_checked<W>(&mut self) -> Result<W>
    where
        SB: SourceBuilder,
        SB::DataSource: Send + Sync,
        W: for<'a> Writer<'a, TypeSystem = TS>,
        TS: for<'a> Transmit<SB::DataSource, <W as Writer<'a>>::PartitionWriter> + Clone,
    {
        self.entry::<W>(true)
    }

    pub fn run<W>(&mut self) -> Result<W>
    where
        SB: SourceBuilder,
        SB::DataSource: Send + Sync,
        W: for<'a> Writer<'a, TypeSystem = TS>,
        TS: for<'a> Transmit<SB::DataSource, <W as Writer<'a>>::PartitionWriter> + Clone,
    {
        self.entry::<W>(false)
    }

    /// Run the dispatcher by specifying the writer, the dispatcher will fetch, parse the data
    /// and return a writer with parsed result
    fn entry<W>(&mut self, checked: bool) -> Result<W>
    where
        SB: SourceBuilder,
        SB::DataSource: Send + Sync,
        W: for<'a> Writer<'a, TypeSystem = TS>,
        TS: for<'a> Transmit<SB::DataSource, <W as Writer<'a>>::PartitionWriter> + Clone,
    {
        // generate sources
        let mut sources: Vec<SB::DataSource> = (0..self.queries.len())
            .map(|_i| self.source_builder.build())
            .collect();

        // run queries
        sources
            .par_iter_mut()
            .zip_eq(self.queries.as_slice())
            .for_each(|(source, query)| source.run_query(query.as_str()).expect("run query"));

        // collect transmit functions for schema
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

        // allocate memory and create one partition writer for each source
        let num_rows: Vec<usize> = sources.iter().map(|source| source.nrows()).collect();
        let mut dw = W::allocate(num_rows.iter().sum(), self.schema.clone())?;
        let writers = dw.partition_writers(num_rows.as_slice());

        // parse and write
        writers
            .into_par_iter()
            .zip_eq(sources)
            .for_each(|(mut writer, mut source)| {
                let f = funcs.clone();
                for row in 0..writer.nrows() {
                    for col in 0..writer.ncols() {
                        f[col](&mut source, &mut writer, row, col).expect("write record");
                    }
                }
            });

        Ok(dw)
    }
}
