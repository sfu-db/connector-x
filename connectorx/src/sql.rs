use crate::errors::ConnectorAgentError;
use anyhow::anyhow;
use fehler::{throw, throws};
use log::{debug, trace};
use sqlparser::ast::{
    BinaryOperator, Expr, Function, FunctionArg, Ident, ObjectName, Query, Select, SelectItem,
    SetExpr, Statement, TableAlias, TableFactor, TableWithJoins, Value,
};
use sqlparser::dialect::Dialect;
use sqlparser::parser::Parser;

#[throws(ConnectorAgentError)]
pub fn get_limit<T: Dialect>(sql: &str, dialect: &T) -> Option<usize> {
    let mut ast = Parser::parse_sql(dialect, sql)?;
    if ast.len() != 1 {
        throw!(ConnectorAgentError::SQLQueryNotSupported(sql.to_string()));
    }

    match &mut ast[0] {
        Statement::Query(q) => {
            if let Some(expr) = &q.limit {
                return Some(
                    expr.to_string()
                        .parse()
                        .map_err(|e: std::num::ParseIntError| anyhow!(e))?,
                );
            }
        }
        _ => throw!(ConnectorAgentError::SQLQueryNotSupported(sql.to_string())),
    };
    None
}

fn wrap_query(
    query: Box<Query>,
    projection: Vec<SelectItem>,
    selection: Option<Expr>,
    tmp_tab_name: String,
) -> Statement {
    Statement::Query(Box::new(Query {
        with: None,
        body: SetExpr::Select(Box::new(Select {
            distinct: false,
            top: None,
            projection,
            from: vec![TableWithJoins {
                relation: TableFactor::Derived {
                    lateral: false,
                    subquery: query,
                    alias: Some(TableAlias {
                        name: Ident {
                            value: tmp_tab_name,
                            quote_style: None,
                        },
                        columns: vec![],
                    }),
                },
                joins: vec![],
            }],
            lateral_views: vec![],
            selection,
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

#[throws(ConnectorAgentError)]
pub fn count_query<T: Dialect>(sql: &str, dialect: &T) -> String {
    trace!("Incoming query: {}", sql);

    let mut ast = Parser::parse_sql(dialect, sql)?;
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
                    let projection = vec![SelectItem::UnnamedExpr(Expr::Function(Function {
                        name: ObjectName(vec![Ident {
                            value: "count".to_string(),
                            quote_style: None,
                        }]),
                        args: vec![FunctionArg::Unnamed(Expr::Wildcard)],
                        over: None,
                        distinct: false,
                    }))];
                    ast_count =
                        wrap_query(q.clone(), projection, None, String::from("CXTMPTAB_COUNT"));
                }
                _ => throw!(ConnectorAgentError::SQLQueryNotSupported(sql.to_string())),
            }
        }
        _ => throw!(ConnectorAgentError::SQLQueryNotSupported(sql.to_string())),
    };

    let sql = format!("{}", ast_count);
    debug!("Transformed count query: {}", sql);
    sql
}

#[throws(ConnectorAgentError)]
pub fn limit1_query<T: Dialect>(sql: &str, dialect: &T) -> String {
    trace!("Incoming query: {}", sql);

    let mut ast = Parser::parse_sql(dialect, sql)?;
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
    debug!("Transformed limit 1 query: {}", sql);
    sql
}

#[throws(ConnectorAgentError)]
pub fn single_col_partition_query<T: Dialect>(
    query: &str,
    col: &str,
    lower: i64,
    upper: i64,
    dialect: &T,
) -> String {
    trace!("Incoming query: {}", query);
    const PART_TMP_TAB_NAME: &str = "CXTMPTAB_PART";

    let mut ast = Parser::parse_sql(dialect, query)?;
    if ast.len() != 1 {
        throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string()));
    }

    let ast_part: Statement;

    match &mut ast[0] {
        Statement::Query(q) => match &mut q.body {
            SetExpr::Select(_select) => {
                let lb = Expr::BinaryOp {
                    left: Box::new(Expr::Value(Value::Number(lower.to_string(), false))),
                    op: BinaryOperator::LtEq,
                    right: Box::new(Expr::CompoundIdentifier(vec![
                        Ident {
                            value: PART_TMP_TAB_NAME.to_string(),
                            quote_style: None,
                        },
                        Ident {
                            value: col.to_string(),
                            quote_style: None,
                        },
                    ])),
                };

                let ub = Expr::BinaryOp {
                    left: Box::new(Expr::CompoundIdentifier(vec![
                        Ident {
                            value: PART_TMP_TAB_NAME.to_string(),
                            quote_style: None,
                        },
                        Ident {
                            value: col.to_string(),
                            quote_style: None,
                        },
                    ])),
                    op: BinaryOperator::Lt,
                    right: Box::new(Expr::Value(Value::Number(upper.to_string(), false))),
                };

                let selection = Expr::BinaryOp {
                    left: Box::new(lb),
                    op: BinaryOperator::And,
                    right: Box::new(ub),
                };

                ast_part = wrap_query(
                    q.clone(),
                    vec![SelectItem::Wildcard],
                    Some(selection),
                    PART_TMP_TAB_NAME.to_string(),
                );
            }
            _ => throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string())),
        },
        _ => throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string())),
    };

    let sql = format!("{}", ast_part);
    debug!("Transformed single column partition query: {}", sql);
    sql
}

#[throws(ConnectorAgentError)]
pub fn get_partition_range_query<T: Dialect>(query: &str, col: &str, dialect: &T) -> String {
    trace!("Incoming query: {}", query);
    const RANGE_TMP_TAB_NAME: &str = "CXTMPTAB_RANGE";
    let mut ast = Parser::parse_sql(dialect, query)?;
    if ast.len() != 1 {
        throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string()));
    }

    let ast_range: Statement;

    match &mut ast[0] {
        Statement::Query(q) => {
            q.order_by = vec![];
            match &mut q.body {
                SetExpr::Select(_select) => {
                    let projection = vec![
                        SelectItem::UnnamedExpr(Expr::Function(Function {
                            name: ObjectName(vec![Ident {
                                value: "min".to_string(),
                                quote_style: None,
                            }]),
                            args: vec![FunctionArg::Unnamed(Expr::CompoundIdentifier(vec![
                                Ident {
                                    value: RANGE_TMP_TAB_NAME.to_string(),
                                    quote_style: None,
                                },
                                Ident {
                                    value: col.to_string(),
                                    quote_style: None,
                                },
                            ]))],
                            over: None,
                            distinct: false,
                        })),
                        SelectItem::UnnamedExpr(Expr::Function(Function {
                            name: ObjectName(vec![Ident {
                                value: "max".to_string(),
                                quote_style: None,
                            }]),
                            args: vec![FunctionArg::Unnamed(Expr::CompoundIdentifier(vec![
                                Ident {
                                    value: RANGE_TMP_TAB_NAME.to_string(),
                                    quote_style: None,
                                },
                                Ident {
                                    value: col.to_string(),
                                    quote_style: None,
                                },
                            ]))],
                            over: None,
                            distinct: false,
                        })),
                    ];
                    ast_range =
                        wrap_query(q.clone(), projection, None, RANGE_TMP_TAB_NAME.to_string());
                }
                _ => throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string())),
            }
        }
        _ => throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string())),
    };
    let sql = format!("{}", ast_range);
    debug!("Transformed partition range query: {}", sql);
    sql
}

#[throws(ConnectorAgentError)]
pub fn get_partition_range_query_sep<T: Dialect>(
    query: &str,
    col: &str,
    dialect: &T,
) -> (String, String) {
    trace!("Incoming query: {}", query);
    const RANGE_TMP_TAB_NAME: &str = "CXTMPTAB_RANGE";
    let mut ast = Parser::parse_sql(dialect, query)?;
    if ast.len() != 1 {
        throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string()));
    }

    let ast_range_min: Statement;
    let ast_range_max: Statement;

    match &mut ast[0] {
        Statement::Query(q) => {
            q.order_by = vec![];
            match &mut q.body {
                SetExpr::Select(_select) => {
                    let min_proj = vec![SelectItem::UnnamedExpr(Expr::Function(Function {
                        name: ObjectName(vec![Ident {
                            value: "min".to_string(),
                            quote_style: None,
                        }]),
                        args: vec![FunctionArg::Unnamed(Expr::CompoundIdentifier(vec![
                            Ident {
                                value: RANGE_TMP_TAB_NAME.to_string(),
                                quote_style: None,
                            },
                            Ident {
                                value: col.to_string(),
                                quote_style: None,
                            },
                        ]))],
                        over: None,
                        distinct: false,
                    }))];
                    let max_proj = vec![SelectItem::UnnamedExpr(Expr::Function(Function {
                        name: ObjectName(vec![Ident {
                            value: "max".to_string(),
                            quote_style: None,
                        }]),
                        args: vec![FunctionArg::Unnamed(Expr::CompoundIdentifier(vec![
                            Ident {
                                value: RANGE_TMP_TAB_NAME.to_string(),
                                quote_style: None,
                            },
                            Ident {
                                value: col.to_string(),
                                quote_style: None,
                            },
                        ]))],
                        over: None,
                        distinct: false,
                    }))];
                    ast_range_min =
                        wrap_query(q.clone(), min_proj, None, RANGE_TMP_TAB_NAME.to_string());
                    ast_range_max =
                        wrap_query(q.clone(), max_proj, None, RANGE_TMP_TAB_NAME.to_string());
                }
                _ => throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string())),
            }
        }
        _ => throw!(ConnectorAgentError::SQLQueryNotSupported(query.to_string())),
    };
    let sql_min = format!("{}", ast_range_min);
    let sql_max = format!("{}", ast_range_max);
    debug!(
        "Transformed separated partition range query: {}, {}",
        sql_min, sql_max
    );
    (sql_min, sql_max)
}
