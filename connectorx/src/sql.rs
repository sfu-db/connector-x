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

#[derive(Debug, Clone)]
pub enum CXQuery<Q = String> {
    Naked(Q),   // The query directly comes from the user
    Wrapped(Q), // The user query is already wrapped in a subquery
}

impl<Q: std::fmt::Display> std::fmt::Display for CXQuery<Q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CXQuery::Naked(q) => write!(f, "{}", q),
            CXQuery::Wrapped(q) => write!(f, "{}", q),
        }
    }
}

impl<Q: AsRef<str>> CXQuery<Q> {
    pub fn as_str(&self) -> &str {
        match self {
            CXQuery::Naked(q) => q.as_ref(),
            CXQuery::Wrapped(q) => q.as_ref(),
        }
    }
}

impl CXQuery<String> {
    pub fn naked<Q: AsRef<str>>(q: Q) -> Self {
        CXQuery::Naked(q.as_ref().to_string())
    }
}

impl<Q: AsRef<str>> AsRef<str> for CXQuery<Q> {
    fn as_ref(&self) -> &str {
        match self {
            CXQuery::Naked(q) => q.as_ref(),
            CXQuery::Wrapped(q) => q.as_ref(),
        }
    }
}

impl<Q> CXQuery<Q> {
    pub fn map<F, U>(&self, f: F) -> CXQuery<U>
    where
        F: Fn(&Q) -> U,
    {
        match self {
            CXQuery::Naked(q) => CXQuery::Naked(f(q)),
            CXQuery::Wrapped(q) => CXQuery::Wrapped(f(q)),
        }
    }
}

impl<Q, E> CXQuery<Result<Q, E>> {
    pub fn result(self) -> Result<CXQuery<Q>, E> {
        match self {
            CXQuery::Naked(q) => q.map(CXQuery::Naked),
            CXQuery::Wrapped(q) => q.map(CXQuery::Wrapped),
        }
    }
}

#[throws(ConnectorAgentError)]
pub fn get_limit<T: Dialect>(sql: &CXQuery<String>, dialect: &T) -> Option<usize> {
    let mut ast = Parser::parse_sql(dialect, sql.as_str())?;
    if ast.len() != 1 {
        throw!(ConnectorAgentError::SqlQueryNotSupported(sql.to_string()));
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
        _ => throw!(ConnectorAgentError::SqlQueryNotSupported(sql.to_string())),
    };
    None
}

fn wrap_query(
    query: &Query,
    projection: Vec<SelectItem>,
    selection: Option<Expr>,
    tmp_tab_name: &str,
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
                    subquery: Box::new(query.clone()),
                    alias: Some(TableAlias {
                        name: Ident {
                            value: tmp_tab_name.into(),
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

trait StatementExt {
    fn as_query(&self) -> Option<&Query>;
}

impl StatementExt for Statement {
    fn as_query(&self) -> Option<&Query> {
        match self {
            Statement::Query(q) => Some(q),
            _ => None,
        }
    }
}

trait QueryExt {
    fn as_select_mut(&mut self) -> Option<&mut Select>;
}

impl QueryExt for Query {
    fn as_select_mut(&mut self) -> Option<&mut Select> {
        match self.body {
            SetExpr::Select(ref mut select) => Some(select),
            _ => None,
        }
    }
}

#[throws(ConnectorAgentError)]
pub fn count_query<T: Dialect>(sql: &CXQuery<String>, dialect: &T) -> CXQuery<String> {
    trace!("Incoming query: {}", sql);

    let projection = vec![SelectItem::UnnamedExpr(Expr::Function(Function {
        name: ObjectName(vec![Ident {
            value: "count".to_string(),
            quote_style: None,
        }]),
        args: vec![FunctionArg::Unnamed(Expr::Wildcard)],
        over: None,
        distinct: false,
    }))];

    let ast = sql.map(|sql| Parser::parse_sql(dialect, sql)).result()?;

    let ast_count: Statement = match ast {
        CXQuery::Naked(ast) => {
            if ast.len() != 1 {
                throw!(ConnectorAgentError::SqlQueryNotSupported(sql.to_string()));
            }
            let mut query = ast[0]
                .as_query()
                .ok_or_else(|| ConnectorAgentError::SqlQueryNotSupported(sql.to_string()))?
                .clone();
            query.order_by = vec![];
            let select = query
                .as_select_mut()
                .ok_or_else(|| ConnectorAgentError::SqlQueryNotSupported(sql.to_string()))?;
            select.sort_by = vec![];
            wrap_query(&query, projection, None, "CXTMPTAB_COUNT")
        }
        CXQuery::Wrapped(ast) => {
            if ast.len() != 1 {
                throw!(ConnectorAgentError::SqlQueryNotSupported(sql.to_string()));
            }
            let mut query = ast[0]
                .as_query()
                .ok_or_else(|| ConnectorAgentError::SqlQueryNotSupported(sql.to_string()))?
                .clone();
            let select = query
                .as_select_mut()
                .ok_or_else(|| ConnectorAgentError::SqlQueryNotSupported(sql.to_string()))?;
            select.projection = projection;
            Statement::Query(Box::new(query))
        }
    };

    let sql = format!("{}", ast_count);
    debug!("Transformed count query: {}", sql);
    CXQuery::Wrapped(sql)
}

#[throws(ConnectorAgentError)]
pub fn limit1_query<T: Dialect>(sql: &CXQuery<String>, dialect: &T) -> CXQuery<String> {
    trace!("Incoming query: {}", sql);

    let mut ast = Parser::parse_sql(dialect, sql.as_str())?;
    if ast.len() != 1 {
        throw!(ConnectorAgentError::SqlQueryNotSupported(sql.to_string()));
    }

    match &mut ast[0] {
        Statement::Query(q) => {
            q.limit = Some(Expr::Value(Value::Number("1".to_string(), false)));
        }
        _ => throw!(ConnectorAgentError::SqlQueryNotSupported(sql.to_string())),
    };

    let sql = format!("{}", ast[0]);
    debug!("Transformed limit 1 query: {}", sql);
    CXQuery::Wrapped(sql)
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
        throw!(ConnectorAgentError::SqlQueryNotSupported(query.to_string()));
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
                    &q,
                    vec![SelectItem::Wildcard],
                    Some(selection),
                    PART_TMP_TAB_NAME,
                );
            }
            _ => throw!(ConnectorAgentError::SqlQueryNotSupported(query.to_string())),
        },
        _ => throw!(ConnectorAgentError::SqlQueryNotSupported(query.to_string())),
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
        throw!(ConnectorAgentError::SqlQueryNotSupported(query.to_string()));
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
                    ast_range = wrap_query(&q, projection, None, RANGE_TMP_TAB_NAME);
                }
                _ => throw!(ConnectorAgentError::SqlQueryNotSupported(query.to_string())),
            }
        }
        _ => throw!(ConnectorAgentError::SqlQueryNotSupported(query.to_string())),
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
        throw!(ConnectorAgentError::SqlQueryNotSupported(query.to_string()));
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
                                value: RANGE_TMP_TAB_NAME.into(),
                                quote_style: None,
                            },
                            Ident {
                                value: col.into(),
                                quote_style: None,
                            },
                        ]))],
                        over: None,
                        distinct: false,
                    }))];
                    ast_range_min = wrap_query(&q, min_proj, None, RANGE_TMP_TAB_NAME);
                    ast_range_max = wrap_query(&q, max_proj, None, RANGE_TMP_TAB_NAME);
                }
                _ => throw!(ConnectorAgentError::SqlQueryNotSupported(query.to_string())),
            }
        }
        _ => throw!(ConnectorAgentError::SqlQueryNotSupported(query.to_string())),
    };
    let sql_min = format!("{}", ast_range_min);
    let sql_max = format!("{}", ast_range_max);
    debug!(
        "Transformed separated partition range query: {}, {}",
        sql_min, sql_max
    );
    (sql_min, sql_max)
}
