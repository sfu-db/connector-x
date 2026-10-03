//! Typed, database-specific options for a read, passed alongside a
//! [`SourceConn`](crate::source_router::SourceConn).
//!
//! This is the single extension point for backend configuration that does not
//! belong in the connection URL (for example secrets such as access tokens).
//! Shared entry points only route the options and reject ones that don't
//! match the source; each backend owns its own option types and validation.

use crate::errors::ConnectorXError;
use crate::source_router::SourceType;
#[cfg(feature = "src_mssql_common")]
pub use crate::sources::mssql::MsSqlOptions;
#[allow(unused_imports)]
use anyhow::anyhow;
#[allow(unused_imports)]
use fehler::{throw, throws};
#[cfg(feature = "src_mssql_common")]
use std::borrow::Cow;

/// Backend-specific options for one read. `Default` means "URL only", which is
/// how every existing entry point behaves.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub enum SourceOptions {
    #[default]
    Default,
    #[cfg(feature = "src_mssql_common")]
    MsSql(MsSqlOptions),
}

impl SourceOptions {
    /// Rejects options meant for a different database than `ty`, so they are
    /// never silently ignored.
    #[throws(ConnectorXError)]
    pub fn check_source_type(&self, ty: &SourceType) {
        match (self, ty) {
            (SourceOptions::Default, _) => {}
            #[cfg(feature = "src_mssql_common")]
            (SourceOptions::MsSql(_), SourceType::MsSQL) => {}
            #[cfg(feature = "src_mssql_common")]
            (SourceOptions::MsSql(_), other) => throw!(anyhow!(
                "MsSqlOptions can only be used with SQL Server (mssql://) connections, got {:?}",
                other
            )),
        }
    }

    /// The SQL Server options to use, defaulting to none set.
    #[cfg(feature = "src_mssql_common")]
    pub fn mssql_or_default(&self) -> Cow<'_, MsSqlOptions> {
        match self {
            SourceOptions::MsSql(options) => Cow::Borrowed(options),
            _ => Cow::Owned(MsSqlOptions::default()),
        }
    }
}

#[cfg(feature = "src_mssql_common")]
impl From<MsSqlOptions> for SourceOptions {
    fn from(options: MsSqlOptions) -> Self {
        SourceOptions::MsSql(options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_accept_every_source() {
        for ty in [SourceType::Postgres, SourceType::MySQL, SourceType::MsSQL] {
            assert!(SourceOptions::default().check_source_type(&ty).is_ok());
        }
    }

    #[cfg(feature = "src_mssql_common")]
    #[test]
    fn mssql_options_are_rejected_for_other_sources() {
        let options = SourceOptions::from(MsSqlOptions::new());
        assert!(options.check_source_type(&SourceType::MsSQL).is_ok());
        for ty in [SourceType::Postgres, SourceType::SQLite, SourceType::Oracle] {
            let err = options.check_source_type(&ty).unwrap_err();
            assert!(err.to_string().contains("only be used with SQL Server"));
        }
    }

    #[cfg(feature = "src_mssql_common")]
    #[test]
    fn debug_output_redacts_access_token() {
        let token = crate::sources::mssql::random_test_token();
        let options = SourceOptions::from(MsSqlOptions::new().with_access_token(token.clone()));
        let debug = format!("{:?}", options);
        assert!(!debug.contains(&token));
        assert!(debug.contains("<redacted>"));
        assert!(!SourceOptions::Default.mssql_or_default().has_access_token());
    }
}
