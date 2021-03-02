mod pandas_columns;
mod pystring;
mod writers;

use crate::errors::ConnectorAgentPythonError;
use anyhow::anyhow;
use connector_agent::{
    data_sources::{
        postgres::{PostgresDTypes, PostgresSource, PostgresSourceParser},
        Source,
    },
    transmit::{Transmit, TransmitChecked},
    typesystem::TypeSystemConversion,
    writers::{pandas::PandasTypes, Writer},
    Dispatcher, Realize, TypeSystem,
};
use fehler::throws;
use log::debug;
use pyo3::{PyAny, Python};
pub use writers::{PandasPartitionWriter, PandasWriter};

#[throws(ConnectorAgentPythonError)]
pub fn write_pandas<'a>(py: Python<'a>, conn: &str, queries: &[&str], checked: bool) -> &'a PyAny {
    let mut writer = PandasWriter::new(py);
    let sb = PostgresSource::new(conn, queries.len());

    // TODO: unlock gil for these two line
    let dispatcher = Dispatcher::new(sb, &mut writer, queries);

    debug!("Running dispatcher");
    if checked {
        dispatcher.run_checked()?;
    } else {
        dispatcher.run()?;
    }
    foo();
    writer.result().ok_or(anyhow!("writer not run"))?
}

fn foo<'py>()
where
    PostgresDTypes: TypeSystem,
    PandasTypes: TypeSystem + TypeSystemConversion<PostgresDTypes>,
    PostgresSource: Source<TypeSystem = PostgresDTypes>,
    PandasWriter<'py>: Writer<TypeSystem = PandasTypes>,
    (PostgresDTypes, PandasTypes):
        for<'r, 's> Realize<TransmitChecked<PostgresSourceParser<'s>, PandasPartitionWriter<'r>>>,
    (PostgresDTypes, PandasTypes):
        for<'r, 's> Realize<Transmit<PostgresSourceParser<'s>, PandasPartitionWriter<'r>>>,
{
}
