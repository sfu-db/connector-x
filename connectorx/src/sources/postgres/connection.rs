use postgres::config::SslMode;
use std::path::PathBuf;

use openssl::ssl::{SslConnector, SslFiletype, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use postgres::Config;
use std::collections::HashMap;
use url::Url;

pub struct TlsConfig {
    /// SSL mode (`sslmode`).
    pub ssl_mode: SslMode,
    /// Location of the client cert and key (`sslcert`, `sslkey`).
    pub client_cert: Option<(PathBuf, PathBuf)>,
    /// Location of the root certificate (`sslrootcert`).
    pub root_cert: Option<PathBuf>,
}


impl From<TlsConfig> for MakeTlsConnector {
    // This function adapted primarily from: 
    // https://github.com/sfackler/rust-postgres/pull/774
    fn from(tls_config: TlsConfig) -> Self {
        let mut builder = SslConnector::builder(SslMethod::tls_client()).unwrap();

        let (verify_mode, verify_hostname) = match tls_config.ssl_mode {
            SslMode::Disable | SslMode::Prefer => (SslVerifyMode::NONE, false),
            SslMode::Require => match tls_config.root_cert {
                // If a root CA file exists, the behavior of sslmode=require will be the same as
                // that of verify-ca, meaning the server certificate is validated against the CA.
                //
                // For more details, check out the note about backwards compatibility in
                // https://postgresql.org/docs/current/libpq-ssl.html#LIBQ-SSL-CERTIFICATES.
                Some(_) => (SslVerifyMode::PEER, false),
                None => (SslVerifyMode::NONE, false),
            },
            // These two modes will not work until upstream rust-postgres supports parsing 
            // them as part of the TLS config.
            //
            // SslMode::VerifyCa => (SslVerifyMode::PEER, false),
            // SslMode::VerifyFull => (SslVerifyMode::PEER, true),
            _ => panic!("unexpected sslmode {:?}", tls_config.ssl_mode),
        };
    
    
        // Configure certificates
        if tls_config.client_cert.is_some() {
            let (cert, key) = tls_config.client_cert.unwrap();
            builder
                .set_certificate_file(cert, SslFiletype::PEM)
                .unwrap();
            builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
        }
        if tls_config.root_cert.is_some() {
            builder.set_ca_file(tls_config.root_cert.unwrap()).unwrap();
        }
    
        let mut tls_connector = MakeTlsConnector::new(builder.build());
    
        // Configure hostname verification
        match (verify_mode, verify_hostname) {
            (SslVerifyMode::PEER, false) => tls_connector.set_callback(|connect, _| {
                connect.set_verify_hostname(false);
                Ok(())
            }),
            _ => {}
        }
    
        tls_connector
    }
}


pub fn get_query_params(url: Url) -> HashMap<String, String> {
    url.query_pairs().into_owned().collect()
}

// Takes the SSL options from the query string, parses and coverts them
pub fn parse_ssl_opts(
    params: HashMap<String, String>,
) -> (Option<(PathBuf, PathBuf)>, Option<PathBuf>) {
    let sslcert = params.get("sslcert").map(|x| PathBuf::from(x));
    let sslkey = params.get("sslkey").map(|x| PathBuf::from(x));
    let sslrootcert = params.get("sslrootcert").map(|x| PathBuf::from(x));

    let opt_client_cert = match (sslcert, sslkey) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    };

    (opt_client_cert, sslrootcert)
}

// Strip URL params not accepted by upstream rust-postgres
pub fn strip_bad_opts(url: Url) -> Url {
    let stripped_query: Vec<(_, _)> = url
        .query_pairs()
        .filter(|p| match &*p.0 {
            "sslkey" | "sslcert" | "sslrootcert" => false,
            _ => true,
        })
        .collect();

    let mut url2 = url.clone();
    url2.set_query(None);

    for pair in stripped_query {
        url2.query_pairs_mut()
            .append_pair(&pair.0.to_string()[..], &pair.1.to_string()[..]);
    }

    url2
}

pub fn rewrite_tls_args(conn: &str) -> (String, MakeTlsConnector){
    // We parse the config, then strip unsupported SSL opts and rewrite the URI 
    // before calling conn.parse().
    // 
    // For more details on this approach, see the conversation here: 
    // https://github.com/sfackler/rust-postgres/pull/774#discussion_r641784774
    let parsed_conn_str = Url::parse(conn).expect("Parsing connection string failed.");

    let params: HashMap<String, String> = get_query_params(parsed_conn_str.clone());
    let (client_cert, root_cert) = parse_ssl_opts(params);

    let stripped_url = strip_bad_opts(parsed_conn_str.clone());
    
    let c: Config = stripped_url.as_str().parse().unwrap();
    let ssl_mode = c.get_ssl_mode();

    let tls_connector = TlsConfig {
        ssl_mode,
        client_cert,
        root_cert,
    }
    .into();

    (stripped_url.as_str().to_owned(), tls_connector)
}