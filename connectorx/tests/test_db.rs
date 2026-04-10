use std::{
    env, fs,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    path::PathBuf,
    sync::Once,
    thread,
    time::{Duration, Instant},
};

use testcontainers::{
    core::{CmdWaitFor, ExecCommand, IntoContainerPort, Mount, WaitFor},
    runners::SyncRunner,
    GenericImage, ImageExt,
};

static POSTGRES_INIT: Once = Once::new();
static MYSQL_INIT: Once = Once::new();
static MSSQL_INIT: Once = Once::new();
static TRINO_INIT: Once = Once::new();
static CLICKHOUSE_INIT: Once = Once::new();
static ORACLE_INIT: Once = Once::new();

fn scripts_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../scripts")
}

fn wait_for_tcp_ready(host: &str, port: u16, timeout: Duration, label: &str) {
    let deadline = Instant::now() + timeout;
    let addr_str = format!("{host}:{port}");
    let addrs: Vec<SocketAddr> = addr_str
        .to_socket_addrs()
        .expect("resolve service address")
        .collect();

    loop {
        for addr in &addrs {
            if TcpStream::connect_timeout(addr, Duration::from_secs(2)).is_ok() {
                return;
            }
        }

        if Instant::now() >= deadline {
            panic!(
                "{label} is not reachable at {host}:{port} after {:?}",
                timeout
            );
        }

        thread::sleep(Duration::from_millis(500));
    }
}

#[cfg(feature = "src_postgres")]
pub fn postgres_url() -> String {
    POSTGRES_INIT.call_once(|| {
        if env::var("POSTGRES_URL").is_ok() {
            return;
        }

        let init_script = scripts_dir().join("postgres.sql");
        let image = GenericImage::new("pgvector/pgvector", "pg17")
            .with_exposed_port(5432.tcp())
            .with_wait_for(WaitFor::message_on_stdout(
                "database system is ready to accept connections",
            ))
            .with_startup_timeout(Duration::from_secs(180))
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "postgres")
            .with_env_var("POSTGRES_DB", "postgres")
            .with_mount(Mount::bind_mount(
                init_script.to_string_lossy().into_owned(),
                "/docker-entrypoint-initdb.d/postgres.sql".to_string(),
            ));

        let container = image.start().expect("start postgres testcontainer");
        let host = container
            .get_host()
            .expect("get postgres container host")
            .to_string();
        let port = container
            .get_host_port_ipv4(5432)
            .expect("get postgres exposed port");

        env::set_var(
            "POSTGRES_URL",
            format!("postgresql://postgres:postgres@{host}:{port}/postgres"),
        );

        // Keep the container alive for the test process lifetime.
        std::mem::forget(container);
    });

    env::var("POSTGRES_URL").expect("POSTGRES_URL must be set")
}

#[cfg(feature = "src_mysql")]
pub fn mysql_url() -> String {
    MYSQL_INIT.call_once(|| {
        if env::var("MYSQL_URL").is_ok() {
            return;
        }

        let init_script = scripts_dir().join("mysql.sql");
        let image = GenericImage::new("ghcr.io/wangxiaoying/mysql", "latest")
            .with_exposed_port(3306.tcp())
            .with_wait_for(WaitFor::message_on_stderr("ready for connections"))
            .with_startup_timeout(Duration::from_secs(180))
            .with_env_var("MYSQL_ROOT_PASSWORD", "mysql")
            .with_env_var("MYSQL_DATABASE", "mysql")
            .with_env_var("LANG", "C.UTF-8")
            .with_mount(Mount::bind_mount(
                init_script.to_string_lossy().into_owned(),
                "/docker-entrypoint-initdb.d/mysql.sql".to_string(),
            ));

        let container = image.start().expect("start mysql testcontainer");
        let host = container
            .get_host()
            .expect("get mysql container host")
            .to_string();
        let port = container
            .get_host_port_ipv4(3306)
            .expect("get mysql exposed port");

        wait_for_tcp_ready(&host, port, Duration::from_secs(180), "mysql");
        let mysql_host_for_url = if host.eq_ignore_ascii_case("localhost") {
            "127.0.0.1"
        } else {
            host.as_str()
        };

        env::set_var(
            "MYSQL_URL",
            format!("mysql://root:mysql@{mysql_host_for_url}:{port}/mysql"),
        );

        // Keep the container alive for the test process lifetime.
        std::mem::forget(container);
    });

    env::var("MYSQL_URL").expect("MYSQL_URL must be set")
}

#[cfg(feature = "src_mssql")]
pub fn mssql_url() -> String {
    MSSQL_INIT.call_once(|| {
        if env::var("MSSQL_URL").is_ok() {
            return;
        }

        let init_script = scripts_dir().join("mssql.sql");
        let patched_script = std::env::temp_dir().join("connectorx-mssql-test.sql");
        let script = fs::read_to_string(&init_script).expect("read mssql.sql");
        // sqlcmd requires CREATE FUNCTION at start of batch.
        fs::write(
            &patched_script,
            script
                .replace('\u{200B}', "")
                .replace("\nCREATE FUNCTION increment", "\nGO\n\nCREATE FUNCTION increment"),
        )
        .expect("write patched mssql sql");

        let image = GenericImage::new("mcr.microsoft.com/mssql/server", "2022-CU12-ubuntu-22.04")
            .with_exposed_port(1433.tcp())
            .with_wait_for(WaitFor::seconds(60))
            .with_startup_timeout(Duration::from_secs(180))
            .with_env_var("ACCEPT_EULA", "Y")
            .with_env_var("SA_PASSWORD", "1Secure*Password1")
            .with_env_var("SQLSERVER_USER", "SA")
            .with_env_var("SQLSERVER_DBNAME", "tempdb")
            .with_mount(Mount::bind_mount(
                patched_script.to_string_lossy().into_owned(),
                "/tmp/mssql.sql".to_string(),
            ));

        let container = image.start().expect("start mssql testcontainer");
        let mut exec = container
            .exec(
                ExecCommand::new([
                    "bash",
                    "-c",
                    "/opt/mssql-tools*/bin/sqlcmd -S localhost -U \"$SQLSERVER_USER\" -P \"$SA_PASSWORD\" -d \"$SQLSERVER_DBNAME\" -C -b -i /tmp/mssql.sql",
                ])
                .with_cmd_ready_condition(CmdWaitFor::exit_code(0)),
            )
            .expect("exec mssql init script");
        let code = exec.exit_code().expect("read mssql init exit code").unwrap_or(-1);
        if code != 0 {
            let stdout =
                String::from_utf8(exec.stdout_to_vec().unwrap_or_default()).unwrap_or_default();
            let stderr =
                String::from_utf8(exec.stderr_to_vec().unwrap_or_default()).unwrap_or_default();
            panic!("mssql init failed: {}\n{}", stdout, stderr);
        }

        let host = container
            .get_host()
            .expect("get mssql container host")
            .to_string();
        let port = container
            .get_host_port_ipv4(1433)
            .expect("get mssql exposed port");
        env::set_var(
            "MSSQL_URL",
            format!(
                "mssql://SA:1Secure%2APassword1@{host}:{port}/tempdb?trust_server_certificate=true"
            ),
        );

        std::mem::forget(container);
    });

    env::var("MSSQL_URL").expect("MSSQL_URL must be set")
}

#[cfg(feature = "src_clickhouse")]
pub fn clickhouse_url() -> String {
    CLICKHOUSE_INIT.call_once(|| {
        if env::var("CLICKHOUSE_URL").is_ok() {
            return;
        }

        let init_script = scripts_dir().join("clickhouse.sql");
        let image = GenericImage::new("clickhouse/clickhouse-server", "latest")
            .with_exposed_port(8123.tcp())
            .with_wait_for(WaitFor::seconds(30))
            .with_startup_timeout(Duration::from_secs(180))
            .with_env_var("CLICKHOUSE_USER", "default")
            .with_env_var("CLICKHOUSE_PASSWORD", "clickhouse")
            .with_mount(Mount::bind_mount(
                init_script.to_string_lossy().into_owned(),
                "/docker-entrypoint-initdb.d/clickhouse.sql".to_string(),
            ));

        let container = image.start().expect("start clickhouse testcontainer");
        let host = container
            .get_host()
            .expect("get clickhouse container host")
            .to_string();
        let port = container
            .get_host_port_ipv4(8123)
            .expect("get clickhouse exposed port");

        env::set_var(
            "CLICKHOUSE_URL",
            format!("clickhouse://default:clickhouse@{host}:{port}/default"),
        );
        std::mem::forget(container);
    });

    env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL must be set")
}

#[cfg(feature = "src_trino")]
fn run_trino_statement(base_url: &str, statement: &str) {
    let stmt = statement.trim();
    if stmt.is_empty() || stmt.to_uppercase().starts_with("DELETE FROM") {
        return;
    }

    let mut payload: serde_json::Value = ureq::post(&format!("{base_url}/v1/statement"))
        .set("X-Trino-User", "test")
        .set("X-Trino-Catalog", "test")
        .set("X-Trino-Schema", "test")
        .send_string(stmt)
        .expect("execute trino statement")
        .into_json()
        .expect("parse trino response");

    while let Some(next) = payload.get("nextUri").and_then(|v| v.as_str()) {
        payload = ureq::get(next)
            .set("X-Trino-User", "test")
            .set("X-Trino-Catalog", "test")
            .set("X-Trino-Schema", "test")
            .call()
            .expect("poll trino query")
            .into_json()
            .expect("parse trino poll response");
        if payload.get("error").is_some() {
            panic!("trino query failed: {}", payload);
        }
    }
}

#[cfg(feature = "src_trino")]
pub fn trino_url() -> String {
    TRINO_INIT.call_once(|| {
        if env::var("TRINO_URL").is_ok() {
            return;
        }

        let catalog_file = std::env::temp_dir().join("connectorx-trino-test.properties");
        fs::write(&catalog_file, "connector.name=memory\n").expect("write trino catalog");

        let image = GenericImage::new("trinodb/trino", "latest")
            .with_exposed_port(8080.tcp())
            .with_wait_for(WaitFor::message_on_stderr("SERVER STARTED"))
            .with_startup_timeout(Duration::from_secs(180))
            .with_mount(Mount::bind_mount(
                catalog_file.to_string_lossy().into_owned(),
                "/etc/trino/catalog/test.properties".to_string(),
            ));

        let container = image.start().expect("start trino testcontainer");
        let host = container
            .get_host()
            .expect("get trino container host")
            .to_string();
        let port = container
            .get_host_port_ipv4(8080)
            .expect("get trino exposed port");
        let base_url = format!("http://{host}:{port}");
        let trino_conn = format!("trino://test@{host}:{port}/test");
        env::set_var("TRINO_URL", trino_conn);

        let init_script = scripts_dir().join("trino.sql");
        let script = fs::read_to_string(init_script).expect("read trino.sql");
        for stmt in script.split(';') {
            run_trino_statement(&base_url, stmt);
        }

        std::mem::forget(container);
    });

    env::var("TRINO_URL").expect("TRINO_URL must be set")
}

#[cfg(feature = "src_oracle")]
pub fn oracle_url() -> String {
    ORACLE_INIT.call_once(|| {
        if env::var("ORACLE_URL").is_ok() {
            return;
        }

        let init_script = fs::read_to_string(scripts_dir().join("oracle.sql")).expect("read oracle.sql");
        let cleaned_sql = init_script
            .lines()
            .filter(|line| !line.trim().to_uppercase().starts_with("DROP TABLE"))
            .collect::<Vec<_>>()
            .join("\n");

        let wrapped_sql = format!(
            "WHENEVER SQLERROR EXIT SQL.SQLCODE\n\
BEGIN EXECUTE IMMEDIATE 'DROP USER admin CASCADE'; EXCEPTION WHEN OTHERS THEN NULL; END;\n\
/\n\
CREATE USER admin IDENTIFIED BY admin;\n\
GRANT CONNECT, RESOURCE TO admin;\n\
ALTER USER admin QUOTA UNLIMITED ON USERS;\n\
CONNECT admin/admin@localhost:1521/FREEPDB1\n\
{cleaned_sql}\n\
EXIT;\n"
        );
        let script_path = std::env::temp_dir().join("connectorx-oracle-init.sql");
        fs::write(&script_path, wrapped_sql).expect("write oracle init script");

        let image = GenericImage::new("gvenzl/oracle-free", "latest")
            .with_exposed_port(1521.tcp())
            .with_wait_for(WaitFor::message_on_stdout("DATABASE IS READY TO USE"))
            .with_startup_timeout(Duration::from_secs(180))
            .with_env_var("ORACLE_PASSWORD", "oracle")
            .with_mount(Mount::bind_mount(
                script_path.to_string_lossy().into_owned(),
                "/tmp/oracle-init.sql".to_string(),
            ));

        let container = image.start().expect("start oracle testcontainer");

        let mut exec = container
            .exec(
                ExecCommand::new([
                    "bash",
                    "-lc",
                    "/opt/oracle/product/26ai/dbhomeFree/bin/sqlplus -s system/oracle@localhost:1521/FREEPDB1 @/tmp/oracle-init.sql",
                ])
                .with_cmd_ready_condition(CmdWaitFor::exit_code(0)),
            )
            .expect("exec oracle init script");
        let code = exec.exit_code().expect("read oracle init exit code").unwrap_or(-1);
        if code != 0 {
            let stdout =
                String::from_utf8(exec.stdout_to_vec().unwrap_or_default()).unwrap_or_default();
            let stderr =
                String::from_utf8(exec.stderr_to_vec().unwrap_or_default()).unwrap_or_default();
            panic!("oracle init failed: {}\n{}", stdout, stderr);
        }

        let host = container.get_host().expect("get oracle container host").to_string();
        let port = container.get_host_port_ipv4(1521).expect("get oracle exposed port");

        wait_for_tcp_ready(&host, port, Duration::from_secs(300), "oracle");

        env::set_var("ORACLE_URL", format!("oracle://admin:admin@{host}:{port}/FREEPDB1"));
        std::mem::forget(container);
    });

    env::var("ORACLE_URL").expect("ORACLE_URL must be set")
}
