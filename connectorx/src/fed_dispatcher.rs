use crate::{prelude::*, sql::CXQuery};
use arrow::record_batch::RecordBatch;
use datafusion::datasource::MemTable;
use datafusion::prelude::*;
use fehler::throws;
use log::debug;
use rayon::prelude::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::{mpsc::channel, Arc};

#[throws(ConnectorXOutError)]
pub fn run(
    sql: String,
    db_map: HashMap<String, String>,
    j4rs_base: Option<&str>,
) -> Vec<RecordBatch> {
    debug!("federated input sql: {}", sql);
    let mut db_conn_map: HashMap<String, FederatedDataSourceInfo> = HashMap::new();
    for (k, v) in db_map.into_iter() {
        db_conn_map.insert(
            k,
            FederatedDataSourceInfo::new_from_conn_str(
                SourceConn::try_from(v.as_str())?,
                false,
                "",
                "",
            ),
        );
    }
    let fed_plan = rewrite_sql(sql.as_str(), &db_conn_map, j4rs_base)?;

    debug!("fetch queries from remote");
    let (sender, receiver) = channel();
    fed_plan.into_par_iter().enumerate().try_for_each_with(
        sender,
        |s, (i, p)| -> Result<(), ConnectorXOutError> {
            match p.db_name.as_str() {
                "LOCAL" => {
                    s.send((p.sql, None)).expect("send error local");
                }
                _ => {
                    debug!("start query {}: {}", i, p.sql);
                    let mut queries = vec![];
                    p.sql.split(';').for_each(|ss| {
                        queries.push(CXQuery::naked(ss));
                    });
                    let source_conn = &db_conn_map[p.db_name.as_str()]
                        .conn_str_info
                        .as_ref()
                        .unwrap();

                    let destination = get_arrow(source_conn, None, queries.as_slice())?;
                    let rbs = destination.arrow()?;

                    let provider = MemTable::try_new(rbs[0].schema(), vec![rbs])?;
                    s.send((p.db_alias, Some(Arc::new(provider))))
                        .expect(&format!("send error {}", i));
                    debug!("query {} finished", i);
                }
            }
            Ok(())
        },
    )?;

    let ctx = SessionContext::new();
    let mut alias_names: Vec<String> = vec![];
    let mut local_sql = String::new();
    receiver
        .iter()
        .try_for_each(|(alias, provider)| -> Result<(), ConnectorXOutError> {
            match provider {
                Some(p) => {
                    ctx.register_table(alias.as_str(), p)?;
                    alias_names.push(alias);
                }
                None => local_sql = alias,
            }

            Ok(())
        })?;

    debug!("\nexecute query final...\n{}\n", local_sql);
    let rt = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
    // until datafusion fix the bug: https://github.com/apache/arrow-datafusion/issues/2147
    for alias in alias_names {
        local_sql = local_sql.replace(format!("\"{}\"", alias).as_str(), alias.as_str());
    }

    let df = rt.block_on(ctx.sql(local_sql.as_str()))?;
    rt.block_on(df.collect())?
}
