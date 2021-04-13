use crate::errors::ConnectorAgentError;
use crate::sources::postgres::PostgresTypeSystem;
use anyhow::anyhow;
use fehler::{throw, throws};
use log::{debug, trace};
use postgres::{Client, NoTls};
use sqlparser::ast::Expr::Identifier;
use sqlparser::ast::{
    BinaryOperator, Expr, Function, FunctionArg, Ident, ObjectName, SelectItem, SetExpr, Statement,
    Value,
};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

#[throws(ConnectorAgentError)]
pub fn pg_single_col_partition_query(query: &str, col: &str, lower: i64, upper: i64) -> String {
    trace!("Incoming query: {}", query);

    let dialect = PostgreSqlDialect {};

    let mut ast = Parser::parse_sql(&dialect, query)?;

    match &mut ast[0] {
        Statement::Query(q) => match &mut q.body {
            SetExpr::Select(select) => {
                if select.group_by.len() > 0 {
                    throw!(ConnectorAgentError::SQLQueryPartitionNotSupported(
                        query.to_string()
                    ))
                }

                let lb = Expr::BinaryOp {
                    left: Box::new(Expr::Value(Value::Number(lower.to_string(), false))),
                    op: BinaryOperator::LtEq,
                    right: Box::new(Expr::Identifier(Ident {
                        value: col.to_string(),
                        quote_style: None,
                    })),
                };

                let ub = Expr::BinaryOp {
                    left: Box::new(Expr::Identifier(Ident {
                        value: col.to_string(),
                        quote_style: None,
                    })),
                    op: BinaryOperator::Lt,
                    right: Box::new(Expr::Value(Value::Number(upper.to_string(), false))),
                };

                let mut selection = Expr::BinaryOp {
                    left: Box::new(lb),
                    op: BinaryOperator::And,
                    right: Box::new(ub),
                };

                if let Some(exist_selection) = select.selection.take() {
                    selection = Expr::BinaryOp {
                        left: Box::new(Expr::Nested(Box::new(exist_selection))),
                        op: BinaryOperator::And,
                        right: Box::new(Expr::Nested(Box::new(selection))),
                    };
                }

                select.selection.replace(selection);
            }
            _ => throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string())),
        },
        _ => throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string())),
    };

    let sql = format!("{}", ast[0]);
    debug!("Transformed query: {}", sql);
    sql
}

#[throws(ConnectorAgentError)]
fn pg_get_parition_range_query(query: &str, col: &str) -> String {
    trace!("Incoming query: {}", query);
    let dialect = PostgreSqlDialect {};
    let mut ast = Parser::parse_sql(&dialect, query)?;
    match &mut ast[0] {
        Statement::Query(q) => {
            q.order_by = vec![];
            match &mut q.body {
                SetExpr::Select(select) => {
                    if select.group_by.len() > 0 {
                        throw!(ConnectorAgentError::SQLQueryPartitionNotSupported(
                            query.to_string()
                        ))
                    }
                    select.distinct = false;
                    select.top = None;
                    select.projection = vec![
                        SelectItem::UnnamedExpr(Expr::Function(Function {
                            name: ObjectName(vec![Ident {
                                value: "min".to_string(),
                                quote_style: None,
                            }]),
                            args: vec![FunctionArg::Unnamed(Identifier(Ident {
                                value: col.to_string(),
                                quote_style: None,
                            }))],
                            over: None,
                            distinct: false,
                        })),
                        SelectItem::UnnamedExpr(Expr::Function(Function {
                            name: ObjectName(vec![Ident {
                                value: "max".to_string(),
                                quote_style: None,
                            }]),
                            args: vec![FunctionArg::Unnamed(Identifier(Ident {
                                value: col.to_string(),
                                quote_style: None,
                            }))],
                            over: None,
                            distinct: false,
                        })),
                    ];
                    select.sort_by = vec![];
                }
                _ => throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string())),
            }
        }
        _ => throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string())),
    };
    let sql = format!("{}", ast[0]);
    sql
}

#[throws(ConnectorAgentError)]
pub fn pg_get_partition_range(conn: &str, query: &str, col: &str) -> (i64, i64) {
    let mut client = Client::connect(conn, NoTls)?;
    let range_query = pg_get_parition_range_query(query.clone(), col.clone())?;
    let row = client.query_one(range_query.as_str(), &[])?;

    let col_type = PostgresTypeSystem::from(row.columns()[0].type_());
    let (min_v, max_v) = match col_type {
        PostgresTypeSystem::Int4(_) => {
            let min_v: i32 = row.get(0);
            let max_v: i32 = row.get(1);
            (min_v as i64, max_v as i64)
        }
        PostgresTypeSystem::Int8(_) => {
            let min_v: i64 = row.get(0);
            let max_v: i64 = row.get(1);
            (min_v, max_v)
        }
        PostgresTypeSystem::Float4(_) => {
            let min_v: f32 = row.get(0);
            let max_v: f32 = row.get(1);
            (min_v as i64, max_v as i64)
        }
        PostgresTypeSystem::Float8(_) => {
            let min_v: f64 = row.get(0);
            let max_v: f64 = row.get(1);
            (min_v as i64, max_v as i64)
        }
        _ => throw!(anyhow!(
            "Partition can only be done on int or float columns"
        )),
    };

    (min_v, max_v)
}
