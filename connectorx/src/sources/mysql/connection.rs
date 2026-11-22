use super::errors::MySQLSourceError;
use r2d2_mysql::mysql::{Opts, OptsBuilder, SslOpts};
use std::collections::HashMap;
use std::path::PathBuf;
use url::Url;

/// URL query parameters consumed here and stripped before the URL is handed to the
/// `mysql` crate, which rejects any parameter it does not recognize.
const SSL_PARAMS: &[&str] = &["ssl-mode", "sslmode", "ssl-ca"];

/// Build the connection options for a MySQL URL, translating the `ssl-mode` (and
/// `ssl-ca`) query parameters into a [`SslOpts`].
///
/// The `mysql` crate configures TLS programmatically via [`OptsBuilder::ssl_opts`] and
/// errors on unknown URL parameters, so the SSL parameters are parsed and removed here
/// rather than passed through to [`Opts::from_url`].
///
/// The crate also defaults `prefer_socket` to `true`. After a TCP handshake to a
/// loopback address it reads the server's `socket` variable and reconnects through
/// that Unix socket, so a forwarded port such as `127.0.0.1:9999` lands on the local
/// server. An explicit TCP port disables that automatic preference. A URL that sets
/// `socket` or `prefer_socket`, and a URL with no port, keeps the parsed driver value.
pub fn build_opts(conn: &str) -> Result<OptsBuilder, MySQLSourceError> {
    let url = Url::parse(conn)?;

    let params: HashMap<String, String> = url
        .query_pairs()
        .map(|(k, v)| (k.to_ascii_lowercase(), v.into_owned()))
        .collect();
    let ssl_opts = ssl_opts_from_params(&params)?;

    let mut builder = OptsBuilder::from_opts(Opts::from_url(strip_ssl_params(&url).as_str())?);
    if explicit_tcp_port_without_socket_choice(&url) {
        builder = builder.prefer_socket(false);
    }
    Ok(builder.ssl_opts(ssl_opts))
}

/// An explicit TCP port (in the authority or the `port` query) with no `socket` or
/// `prefer_socket` parameter. Those parameters are the driver's own names, matched
/// case-sensitively the same way [`Opts::from_url`] matches them.
fn explicit_tcp_port_without_socket_choice(url: &Url) -> bool {
    let mut explicit_port = url.port().is_some();
    let mut socket_choice = false;
    for (key, _) in url.query_pairs() {
        match key.as_ref() {
            "port" => explicit_port = true,
            "socket" | "prefer_socket" => socket_choice = true,
            _ => {}
        }
    }
    explicit_port && !socket_choice
}

/// Translate a MySQL `ssl-mode` value into [`SslOpts`], returning `None` when TLS is not
/// requested (no `ssl-mode`, or `ssl-mode=DISABLED`) so that behavior is unchanged for
/// connection strings without SSL parameters.
///
/// `PREFERRED` is treated as `REQUIRED` (encrypt without verification): the `mysql` crate
/// has no opportunistic mode that falls back to plaintext when the server lacks TLS.
fn ssl_opts_from_params(
    params: &HashMap<String, String>,
) -> Result<Option<SslOpts>, MySQLSourceError> {
    let mode = match params.get("ssl-mode").or_else(|| params.get("sslmode")) {
        Some(mode) => mode,
        None => return Ok(None),
    };

    let (accept_invalid_certs, skip_domain_validation) =
        match mode.trim().to_ascii_uppercase().replace('-', "_").as_str() {
            "DISABLED" => return Ok(None),
            "PREFERRED" | "REQUIRED" => (true, true),
            "VERIFY_CA" => (false, true),
            "VERIFY_IDENTITY" => (false, false),
            _ => return Err(MySQLSourceError::InvalidSslMode(mode.clone())),
        };

    let ssl_opts = SslOpts::default()
        .with_danger_accept_invalid_certs(accept_invalid_certs)
        .with_danger_skip_domain_validation(skip_domain_validation)
        .with_root_cert_path(params.get("ssl-ca").map(PathBuf::from));
    Ok(Some(ssl_opts))
}

fn strip_ssl_params(url: &Url) -> Url {
    let kept: Vec<(String, String)> = url
        .query_pairs()
        .filter(|(k, _)| !SSL_PARAMS.contains(&k.to_ascii_lowercase().as_str()))
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();

    let mut stripped = url.clone();
    stripped.set_query(None);
    for (k, v) in kept {
        stripped.query_pairs_mut().append_pair(&k, &v);
    }
    stripped
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ssl_opts(conn: &str) -> Option<SslOpts> {
        let opts: Opts = build_opts(conn).unwrap().into();
        opts.get_ssl_opts().cloned()
    }

    #[test]
    fn no_ssl_mode_disables_tls() {
        assert!(ssl_opts("mysql://user:pass@host:3306/db").is_none());
        assert!(ssl_opts("mysql://user:pass@host:3306/db?ssl-mode=DISABLED").is_none());
    }

    #[test]
    fn required_encrypts_without_verification() {
        let opts = ssl_opts("mysql://user:pass@host:3306/db?ssl-mode=REQUIRED").unwrap();
        assert!(opts.accept_invalid_certs());
        assert!(opts.skip_domain_validation());
        assert!(opts.root_cert_path().is_none());
    }

    #[test]
    fn preferred_is_treated_as_required() {
        assert_eq!(
            ssl_opts("mysql://user:pass@host:3306/db?ssl-mode=PREFERRED"),
            ssl_opts("mysql://user:pass@host:3306/db?ssl-mode=REQUIRED"),
        );
    }

    #[test]
    fn verify_ca_checks_chain_but_not_hostname() {
        let opts = ssl_opts("mysql://user:pass@host:3306/db?ssl-mode=VERIFY_CA&ssl-ca=/tmp/ca.pem")
            .unwrap();
        assert!(!opts.accept_invalid_certs());
        assert!(opts.skip_domain_validation());
        assert_eq!(
            opts.root_cert_path(),
            Some(std::path::Path::new("/tmp/ca.pem"))
        );
    }

    #[test]
    fn verify_identity_checks_chain_and_hostname() {
        let opts = ssl_opts("mysql://user:pass@host:3306/db?ssl-mode=VERIFY_IDENTITY").unwrap();
        assert!(!opts.accept_invalid_certs());
        assert!(!opts.skip_domain_validation());
    }

    #[test]
    fn ssl_mode_is_case_insensitive_and_accepts_sslmode_alias() {
        assert!(ssl_opts("mysql://user:pass@host:3306/db?ssl-mode=required").is_some());
        assert!(ssl_opts("mysql://user:pass@host:3306/db?sslmode=Required").is_some());
    }

    #[test]
    fn invalid_ssl_mode_errors() {
        assert!(matches!(
            build_opts("mysql://user:pass@host:3306/db?ssl-mode=bogus"),
            Err(MySQLSourceError::InvalidSslMode(_))
        ));
    }

    #[test]
    fn ssl_params_are_stripped_so_the_url_parses_and_other_params_survive() {
        // `ssl-ca` is not a parameter the mysql crate recognizes, so build_opts must
        // strip it (otherwise the URL is rejected) while leaving non-SSL params intact.
        let opts: Opts = build_opts(
            "mysql://user:pass@host:3306/db?ssl-mode=REQUIRED&ssl-ca=/tmp/ca.pem&prefer_socket=false",
        )
        .unwrap()
        .into();
        assert!(opts.get_ssl_opts().is_some());
        assert!(!opts.get_prefer_socket());
    }

    #[test]
    fn explicit_loopback_port_stays_on_that_port() {
        // Automatic socket preference would drop 127.0.0.1:9999 onto the local socket.
        let opts: Opts = build_opts("mysql://user:pass@127.0.0.1:9999/db")
            .unwrap()
            .into();
        assert_eq!(opts.get_tcp_port(), 9999);
        assert!(!opts.get_prefer_socket());
    }

    #[test]
    fn portless_url_keeps_driver_socket_preference() {
        let conn = "mysql://user:pass@127.0.0.1/db";
        let ours: Opts = build_opts(conn).unwrap().into();
        let driver: Opts = Opts::from_url(conn).unwrap();
        assert_eq!(ours.get_tcp_port(), driver.get_tcp_port());
        assert_eq!(ours.get_prefer_socket(), driver.get_prefer_socket());
    }

    #[test]
    fn explicit_socket_choice_survives_on_a_url_with_a_port() {
        let prefer_on: Opts = build_opts("mysql://user:pass@127.0.0.1:9999/db?prefer_socket=true")
            .unwrap()
            .into();
        assert_eq!(prefer_on.get_tcp_port(), 9999);
        assert!(prefer_on.get_prefer_socket());

        let prefer_off: Opts =
            build_opts("mysql://user:pass@127.0.0.1:9999/db?prefer_socket=false")
                .unwrap()
                .into();
        assert_eq!(prefer_off.get_tcp_port(), 9999);
        assert!(!prefer_off.get_prefer_socket());

        let socket: Opts = build_opts("mysql://user:pass@127.0.0.1:9999/db?socket=/tmp/mysql.sock")
            .unwrap()
            .into();
        let driver: Opts =
            Opts::from_url("mysql://user:pass@127.0.0.1:9999/db?socket=/tmp/mysql.sock").unwrap();
        assert_eq!(socket.get_tcp_port(), 9999);
        assert_eq!(socket.get_socket(), Some("/tmp/mysql.sock"));
        assert_eq!(socket.get_prefer_socket(), driver.get_prefer_socket());
    }
}
