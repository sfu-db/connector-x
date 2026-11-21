//! Typed, SQL Server-specific options that are not part of the connection URL.
//!
//! Passed to the MSSQL source via [`crate::source_options::SourceOptions::MsSql`]
//! so that database-neutral interfaces (`SourceConn`, `get_arrow`, `partition`)
//! carry backend configuration without knowing its details. Validation that
//! depends on the URL or on the driver lives with each driver.

use super::errors::MsSQLSourceError;
use anyhow::anyhow;
use fehler::{throw, throws};

/// SQL Server options that cannot (or should not) be expressed in the URL.
///
/// Fields are private so new options can be added without breaking callers;
/// build values with [`MsSqlOptions::new`] and the `with_*` methods.
#[derive(Clone, Default)]
pub struct MsSqlOptions {
    // A secret: kept out of URLs, redacted from `Debug`, and only readable
    // inside the crate.
    access_token: Option<String>,
}

impl MsSqlOptions {
    /// Creates options with nothing set, equivalent to [`Default::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Authenticates with a Microsoft Entra ID access token (the raw JWT,
    /// e.g. `azure_identity`'s `AccessToken::token`) instead of credentials in
    /// the connection URL. Only the mssql-tds driver supports this.
    pub fn with_access_token(mut self, token: impl Into<String>) -> Self {
        self.access_token = Some(token.into());
        self
    }

    pub(crate) fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }

    /// Whether an access token is set, without exposing it.
    pub fn has_access_token(&self) -> bool {
        self.access_token.is_some()
    }

    /// Checks the options on their own, independent of URL and driver.
    #[throws(MsSQLSourceError)]
    pub fn validate(&self) {
        if self
            .access_token
            .as_deref()
            .is_some_and(|token| token.trim().is_empty())
        {
            throw!(anyhow!("access_token must not be empty"));
        }
    }
}

impl std::fmt::Debug for MsSqlOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MsSqlOptions")
            .field(
                "access_token",
                &self.access_token.as_ref().map(|_| "<redacted>"),
            )
            .finish()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    /// A random, non-JWT-shaped placeholder so tests don't contain anything a
    /// credential scanner would flag.
    pub(crate) fn random_test_token() -> String {
        let mut hasher = RandomState::new().build_hasher();
        hasher.write_u64(std::process::id() as u64);
        format!("test-token-{:016x}", hasher.finish())
    }

    #[test]
    fn default_has_no_access_token() {
        let options = MsSqlOptions::new();
        assert_eq!(options.access_token(), None);
        assert!(!options.has_access_token());
        assert!(options.validate().is_ok());
    }

    #[test]
    fn access_token_is_stored_and_validated() {
        let token = random_test_token();
        let options = MsSqlOptions::new().with_access_token(token.clone());
        assert_eq!(options.access_token(), Some(token.as_str()));
        assert!(options.has_access_token());
        assert!(options.validate().is_ok());

        for empty in ["", "   "] {
            let err = MsSqlOptions::new()
                .with_access_token(empty)
                .validate()
                .unwrap_err();
            assert!(err.to_string().contains("must not be empty"));
        }
    }

    #[test]
    fn debug_output_redacts_access_token() {
        let token = random_test_token();
        let debug = format!("{:?}", MsSqlOptions::new().with_access_token(token.clone()));
        assert!(!debug.contains(&token));
        assert!(debug.contains("<redacted>"));
        assert_eq!(
            format!("{:?}", MsSqlOptions::new()),
            "MsSqlOptions { access_token: None }"
        );
    }
}
