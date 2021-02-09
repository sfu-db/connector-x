use crate::{
    data_order::{coordinate, DataOrder},
    data_sources::{DataSource, SourceBuilder},
    errors::Result,
    types::{Transmit, TransmitChecked},
    typesystem::{Realize, TypeSystem},
    writers::{PartitionWriter, Writer},
};
use rayon::prelude::*;

/// A dispatcher owns a `SourceBuilder` `SB` and a vector of `queries`
/// `schema` is a temporary input before we implement infer schema or get schema from DB.
pub struct Dispatcher<SB, WT, TS> {
    source_builder: SB,
    writer: WT,
    schema: Vec<TS>,
    queries: Vec<String>,
}

impl<SB, WT, TS> Dispatcher<SB, WT, TS>
where
    SB: SourceBuilder,
    SB::DataSource: Send,
    TS: TypeSystem,
    WT: for<'a> Writer<'a, TypeSystem = TS>,
    TS: for<'a> Realize<Transmit<'a, SB::DataSource, <WT as Writer<'a>>::PartitionWriter>>
        + for<'a> Realize<TransmitChecked<'a, SB::DataSource, <WT as Writer<'a>>::PartitionWriter>>,
{
    /// Create a new dispatcher by providing a source builder, schema (temporary) and the queries
    /// to be issued to the data source.
    pub fn new(source_builder: SB, writer: WT, schema: Vec<TS>, queries: Vec<String>) -> Self {
        Dispatcher {
            source_builder,
            writer,
            schema,
            queries,
        }
    }

    pub fn run_checked(self) -> Result<WT> {
        self.entry(true)
    }

    pub fn run(self) -> Result<WT> {
        self.entry(false)
    }

    /// Run the dispatcher by specifying the writer, the dispatcher will fetch, parse the data
    /// and return a writer with parsed result
    fn entry(mut self, checked: bool) -> Result<WT> {
        let dorder = coordinate(SB::DATA_ORDERS, WT::DATA_ORDERS)?;
        self.source_builder.set_data_order(dorder)?;

        // generate sources
        let mut sources: Vec<SB::DataSource> = (0..self.queries.len())
            .map(|_i| self.source_builder.build())
            .collect();

        // run queries
        sources
            .par_iter_mut()
            .zip_eq(self.queries.as_slice())
            .for_each(|(source, query)| source.run_query(query.as_str()).expect("run query"));

        // infer schema if not given
        // let self.schema = sources[0].infer_schema();

        // collect transmit functions for schema
        let funcs: Vec<_> = self
            .schema
            .iter()
            .map(|&ty| {
                if checked {
                    Realize::<TransmitChecked<_, _>>::realize(ty)
                } else {
                    Realize::<Transmit<_, _>>::realize(ty)
                }
            })
            .collect();

        // allocate memory and create one partition writer for each source
        let num_rows: Vec<usize> = sources.iter().map(|source| source.nrows()).collect();
        self.writer
            .allocate(num_rows.iter().sum(), self.schema.clone(), dorder)?;

        // parse and write
        self.writer
            .partition_writers(num_rows.as_slice())
            .into_par_iter()
            .zip_eq(sources)
            .for_each(|(mut writer, mut source)| {
                let f = funcs.clone();
                match dorder {
                    DataOrder::RowMajor => {
                        for row in 0..writer.nrows() {
                            for col in 0..writer.ncols() {
                                f[col](&mut source, &mut writer, row, col).expect("write record");
                            }
                        }
                    }
                    DataOrder::ColumnMajor => {
                        for col in 0..writer.ncols() {
                            for row in 0..writer.nrows() {
                                f[col](&mut source, &mut writer, row, col).expect("write record");
                            }
                        }
                    }
                }
            });

        Ok(self.writer)
    }
}
