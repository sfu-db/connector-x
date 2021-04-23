use crate::errors::ConnectorAgentError;
use anyhow::anyhow;
use fehler::{throw, throws};
use log::{debug, trace};
use sqlparser::ast::{
    Expr, Function, FunctionArg, Ident, ObjectName, Query, Select, SelectItem, SetExpr, Statement,
    TableAlias, TableFactor, TableWithJoins, Value,
};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

#[throws(ConnectorAgentError)]
pub fn get_limit(sql: &str) -> Option<usize> {
    let dialect = PostgreSqlDialect {};
    let mut ast = Parser::parse_sql(&dialect, sql)?;
    if ast.len() != 1 {
        throw!(ConnectorAgentError::SQLQueryNotSupported(sql.to_string()));
    }

    match &mut ast[0] {
        Statement::Query(q) => match &q.limit {
            Some(expr) => {
                return Some(
                    expr.to_string()
                        .parse()
                        .map_err(|e: std::num::ParseIntError| anyhow!(e))?,
                )
            }
            _ => {}
        },
        _ => throw!(ConnectorAgentError::SQLQueryNotSupported(sql.to_string())),
    };
    None
}

#[throws(ConnectorAgentError)]
pub fn count_query(sql: &str) -> String {
    trace!("Incoming query: {}", sql);

    let dialect = PostgreSqlDialect {};

    let mut ast = Parser::parse_sql(&dialect, sql)?;
    if ast.len() != 1 {
        throw!(ConnectorAgentError::SQLQueryNotSupported(sql.to_string()));
    }

    let ast_count: Statement;

    match &mut ast[0] {
        Statement::Query(q) => {
            q.order_by = vec![];
            match &mut q.body {
                SetExpr::Select(select) => {
                    select.sort_by = vec![];

                    ast_count = Statement::Query(Box::new(Query {
                        with: None,
                        body: SetExpr::Select(Box::new(Select {
                            distinct: false,
                            top: None,
                            projection: vec![SelectItem::UnnamedExpr(Expr::Function(Function {
                                name: ObjectName(vec![Ident {
                                    value: "count".to_string(),
                                    quote_style: None,
                                }]),
                                args: vec![FunctionArg::Unnamed(Expr::Wildcard)],
                                over: None,
                                distinct: false,
                            }))],
                            from: vec![TableWithJoins {
                                relation: TableFactor::Derived {
                                    lateral: false,
                                    subquery: q.clone(),
                                    alias: Some(TableAlias {
                                        name: Ident {
                                            value: "CX_TMP_TABLE".to_string(),
                                            quote_style: None,
                                        },
                                        columns: vec![],
                                    }),
                                },
                                joins: vec![],
                            }],
                            lateral_views: vec![],
                            selection: None,
                            group_by: vec![],
                            cluster_by: vec![],
                            distribute_by: vec![],
                            sort_by: vec![],
                            having: None,
                        })),
                        order_by: vec![],
                        limit: None,
                        offset: None,
                        fetch: None,
                    }))
                }
                _ => throw!(ConnectorAgentError::SQLQueryNotSupported(sql.to_string())),
            }
        }
        _ => throw!(ConnectorAgentError::SQLQueryNotSupported(sql.to_string())),
    };

    let sql = format!("{}", ast_count);
    debug!("Transformed query: {}", sql);
    sql
}

#[throws(ConnectorAgentError)]
pub fn limit1_query(sql: &str) -> String {
    trace!("Incoming query: {}", sql);

    let dialect = PostgreSqlDialect {};

    let mut ast = Parser::parse_sql(&dialect, sql)?;
    if ast.len() != 1 {
        throw!(ConnectorAgentError::SQLQueryNotSupported(sql.to_string()));
    }

    match &mut ast[0] {
        Statement::Query(q) => {
            q.limit = Some(Expr::Value(Value::Number("1".to_string(), false)));
        }
        _ => throw!(ConnectorAgentError::SQLQueryNotSupported(sql.to_string())),
    };

    let sql = format!("{}", ast[0]);
    debug!("Transformed query: {}", sql);
    sql
}
