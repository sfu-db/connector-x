use crate::sources::postgres::errors::PostgresSourceError;
use openssl::ssl::{SslConnector, SslFiletype, SslMethod, SslVerifyMode};
use postgres::{config::SslMode, Config};
use postgres_openssl::MakeTlsConnector;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;
use url::Url;

#[derive(Clone, Debug)]
pub struct TlsConfig {
    /// Postgres config, pg_config.sslmode (`sslmode`).
    pub pg_config: Config,
    /// Location of the client cert and key (`sslcert`, `sslkey`).
    pub client_cert: Option<(PathBuf, PathBuf)>,
    /// Location of the root certificate (`sslrootcert`).
    pub root_cert: Option<PathBuf>,
}

impl TryFrom<TlsConfig> for MakeTlsConnector {
    type Error = PostgresSourceError;
    // The logic of this function adapted primarily from:
    // https://github.com/sfackler/rust-postgres/pull/774
    // We only support server side authentication (`sslrootcert`) for now
    fn try_from(tls_config: TlsConfig) -> Result<Self, Self::Error> {
        let mut builder = SslConnector::builder(SslMethod::tls_client())?;
        let ssl_mode = tls_config.pg_config.get_ssl_mode();
        let (verify_ca, verify_hostname) = match ssl_mode {
            SslMode::Disable | SslMode::Prefer => (false, false),
            SslMode::Require => match tls_config.root_cert {
                // If a root CA file exists, the behavior of sslmode=require will be the same as
                // that of verify-ca, meaning the server certificate is validated against the CA.
                //
                // For more details, check out the note about backwards compatibility in
                // https://postgresql.org/docs/current/libpq-ssl.html#LIBQ-SSL-CERTIFICATES.
                Some(_) => (true, false),
                None => (false, false),
            },
            // These two modes will not work until upstream rust-postgres supports parsing
            // them as part of the TLS config.
            //
            // SslMode::VerifyCa => (true, false),
            // SslMode::VerifyFull => (true, true),
            _ => panic!("unexpected sslmode {:?}", ssl_mode),
        };

        if let Some((cert, key)) = tls_config.client_cert {
            builder.set_certificate_file(cert, SslFiletype::PEM)?;
            builder.set_private_key_file(key, SslFiletype::PEM)?;
        }

        if let Some(root_cert) = tls_config.root_cert {
            builder.set_ca_file(root_cert)?;
        }

        if !verify_ca {
            builder.set_verify(SslVerifyMode::NONE); // do not verify CA
        }

        let mut tls_connector = MakeTlsConnector::new(builder.build());

        if !verify_hostname {
            tls_connector.set_callback(|connect, _| {
                connect.set_verify_hostname(false);
                Ok(())
            });
        }

        Ok(tls_connector)
    }
}

// Strip URL params not accepted by upstream rust-postgres
fn strip_bad_opts(url: &Url) -> Url {
    let stripped_query: Vec<(_, _)> = url
        .query_pairs()
        .filter(|p| !matches!(&*p.0, "sslkey" | "sslcert" | "sslrootcert"))
        .collect();

    let mut url2 = url.clone();
    url2.set_query(None);

    for pair in stripped_query {
        url2.query_pairs_mut()
            .append_pair(&pair.0.to_string()[..], &pair.1.to_string()[..]);
    }

    url2
}

pub fn rewrite_tls_args(
    conn: &Url,
) -> Result<(Config, Option<MakeTlsConnector>), PostgresSourceError> {
    // We parse the config, then strip unsupported SSL opts and rewrite the URI
    // before calling conn.parse().
    //
    // For more details on this approach, see the conversation here:
    // https://github.com/sfackler/rust-postgres/pull/774#discussion_r641784774

    let params: HashMap<String, String> = conn.query_pairs().into_owned().collect();

    let sslcert = params.get("sslcert").map(PathBuf::from);
    let sslkey = params.get("sslkey").map(PathBuf::from);
    let root_cert = params.get("sslrootcert").map(PathBuf::from);
    let client_cert = match (sslcert, sslkey) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    };

    let stripped_url = strip_bad_opts(conn);
    let pg_config: Config = stripped_url.as_str().parse().unwrap();

    let tls_config = TlsConfig {
        pg_config: pg_config.clone(),
        client_cert,
        root_cert,
    };

    let tls_connector = match pg_config.get_ssl_mode() {
        SslMode::Disable => None,
        _ => Some(MakeTlsConnector::try_from(tls_config)?),
    };

    Ok((pg_config, tls_connector))
}
