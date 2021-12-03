use std::collections::HashMap;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use url::Url;
use urlencoding::decode;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::new();

    let conn =
        "mssql://sa:mssql!Password@mssql.weiyuan.svc.cluster.dsl:1433/tpch?trusted_connection=true";
    let url = Url::parse(conn)?;
    // let host = decode(url.host_str().unwrap()).unwrap().into_owned();
    // let hosts: Vec<&str> = host.split('\\').collect();
    // println!("hosts: {:?}", hosts);

    // let conn = "mssql://user_name:pass\\word123321@servername:1433/dbname";
    // let url = Url::parse(conn)?;
    // let host = decode(url.host_str().unwrap()).unwrap().into_owned();
    // let hosts: Vec<&str> = host.split('\\').collect();
    // println!("hosts: {:?}", hosts);

    let username = url.username();
    let password = decode(url.password().unwrap()).unwrap().into_owned();
    let p = url.query();
    println!("username: {}", username);
    println!("password: {}", password);
    println!("query: {:?}", p);

    let params: HashMap<String, String> = url.query_pairs().into_owned().collect();
    let tc = params.get("trusted_connection");
    println!("tc: {:?}", tc);

    config.host("mssql.weiyuan.svc.cluster.dsl");
    config.port(1433);
    config.authentication(AuthMethod::sql_server(username, password));
    config.database(&url.path()[1..]);
    config.trust_cert(); // on production, it is not a good idea to do this

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // Client is ready to use.
    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let row = client
        .simple_query("SELECT * FROM test_table")
        .await?
        .into_row()
        .await?
        .unwrap();

    let v: Option<i32> = row.get(0);
    println!("row: {:?}", v);

    Ok(())
}
