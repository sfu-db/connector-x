//! Python wrappers for [`connectorx::source_options::SourceOptions`].
//!
//! Each database gets its own frozen options class (only SQL Server for now),
//! and every public entry point accepts one through a single `source_options`
//! argument instead of growing database-specific parameters.

use connectorx::source_options::{MsSqlOptions, SourceOptions};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

/// SQL Server options that are not part of the connection string.
///
/// `access_token` is a Microsoft Entra ID access token (the raw JWT), used
/// instead of credentials in the connection string. Only the default
/// `mssql-tds` driver supports it.
#[pyclass(name = "MsSqlOptions", module = "connectorx", frozen)]
pub struct PyMsSqlOptions {
    inner: MsSqlOptions,
}

#[pymethods]
impl PyMsSqlOptions {
    #[new]
    #[pyo3(signature = (*, access_token=None))]
    fn new(access_token: Option<String>) -> PyResult<Self> {
        let mut inner = MsSqlOptions::new();
        if let Some(token) = access_token {
            inner = inner.with_access_token(token);
        }
        inner
            .validate()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner })
    }

    // Never echo the token: reprs end up in logs and tracebacks.
    fn __repr__(&self) -> String {
        format!(
            "MsSqlOptions(access_token={})",
            if self.inner.has_access_token() {
                "<redacted>"
            } else {
                "None"
            }
        )
    }
}

/// Converts the optional `source_options` argument into [`SourceOptions`].
pub fn extract_source_options(obj: Option<&Bound<'_, PyAny>>) -> PyResult<SourceOptions> {
    match obj {
        None => Ok(SourceOptions::default()),
        Some(obj) if obj.is_none() => Ok(SourceOptions::default()),
        Some(obj) => match obj.extract::<PyRef<'_, PyMsSqlOptions>>() {
            Ok(options) => Ok(SourceOptions::MsSql(options.inner.clone())),
            Err(_) => Err(PyTypeError::new_err(format!(
                "source_options must be a connectorx.MsSqlOptions instance, got {}",
                obj.get_type().name()?
            ))),
        },
    }
}
