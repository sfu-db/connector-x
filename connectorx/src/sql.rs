use crate::errors::ConnectorXError;
#[cfg(feature = "src_oracle")]
use crate::sources::oracle::OracleDialect;
use anyhow::anyhow;
use fehler::{throw, throws};
use log::{debug, trace, warn};
use sqlparser::ast::{
    BinaryOperator, Expr, Function, FunctionArg, Ident, ObjectName, Query, Select, SelectItem,
    SetExpr, Statement, TableAlias, TableFactor, TableWithJoins, Top, Value,
};
use sqlparser::dialect::{Dialect, MsSqlDialect};
use sqlparser::parser::Parser;
#[cfg(feature = "src_oracle")]
use std::any::Any;

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

impl From<&str> for CXQuery {
    fn from(s: &str) -> CXQuery<String> {
        CXQuery::Naked(s.to_string())
    }
}

impl From<&&str> for CXQuery {
    fn from(s: &&str) -> CXQuery<String> {
        CXQuery::Naked(s.to_string())
    }
}

impl From<&String> for CXQuery {
    fn from(s: &String) -> CXQuery {
        CXQuery::Naked(s.clone())
    }
}

impl From<&CXQuery> for CXQuery {
    fn from(q: &CXQuery) -> CXQuery {
        q.clone()
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

#[throws(ConnectorXError)]
pub fn get_limit<T: Dialect>(sql: &CXQuery<String>, dialect: &T) -> Option<usize> {
    match Parser::parse_sql(dialect, sql.as_str()) {
        Ok(mut ast) => {
            if ast.len() != 1 {
                throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string()));
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
                _ => throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string())),
            };
        }
        Err(e) => {
            warn!("parser error: {:?}, query database for # rows in result", e);
        }
    }
    None
}

#[allow(unreachable_code)] // disable it for now, waiting for the new release of fehler
#[throws(ConnectorXError)]
pub fn get_limit_mssql(sql: &CXQuery<String>) -> Option<usize> {
    let ast = Parser::parse_sql(&MsSqlDialect {}, sql.as_str())?;
    if ast.len() != 1 {
        throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string()));
    }

    if let Statement::Query(q) = &ast[0] {
        if let SetExpr::Select(s) = &q.body {
            if let Some(Top {
                quantity: Some(expr),
                ..
            }) = &s.top
            {
                return Some(
                    expr.to_string()
                        .parse()
                        .map_err(|e: std::num::ParseIntError| anyhow!(e))?,
                );
            }
            return None;
        }
    }

    throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string()))
}

// wrap a query into a derived table
fn wrap_query(
    query: &mut Query,
    projection: Vec<SelectItem>,
    selection: Option<Expr>,
    tmp_tab_name: &str,
) -> Statement {
    let with = query.with.clone();
    query.with = None;
    Statement::Query(Box::new(Query {
        with,
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

#[throws(ConnectorXError)]
pub fn count_query<T: Dialect>(sql: &CXQuery<String>, dialect: &T) -> CXQuery<String> {
    trace!("Incoming query: {}", sql);

    #[allow(unused_mut)]
    let mut sql = match sql.map(|sql| Parser::parse_sql(dialect, sql)).result() {
        Ok(ast) => {
            let projection = vec![SelectItem::UnnamedExpr(Expr::Function(Function {
                name: ObjectName(vec![Ident {
                    value: "count".to_string(),
                    quote_style: None,
                }]),
                args: vec![FunctionArg::Unnamed(Expr::Wildcard)],
                over: None,
                distinct: false,
            }))];
            let ast_count: Statement = match ast {
                CXQuery::Naked(ast) => {
                    if ast.len() != 1 {
                        throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string()));
                    }
                    let mut query = ast[0]
                        .as_query()
                        .ok_or_else(|| ConnectorXError::SqlQueryNotSupported(sql.to_string()))?
                        .clone();
                    query.order_by = vec![];
                    let select = query
                        .as_select_mut()
                        .ok_or_else(|| ConnectorXError::SqlQueryNotSupported(sql.to_string()))?;
                    select.sort_by = vec![];
                    wrap_query(&mut query, projection, None, "CXTMPTAB_COUNT")
                }
                CXQuery::Wrapped(ast) => {
                    if ast.len() != 1 {
                        throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string()));
                    }
                    let mut query = ast[0]
                        .as_query()
                        .ok_or_else(|| ConnectorXError::SqlQueryNotSupported(sql.to_string()))?
                        .clone();
                    let select = query
                        .as_select_mut()
                        .ok_or_else(|| ConnectorXError::SqlQueryNotSupported(sql.to_string()))?;
                    select.projection = projection;
                    Statement::Query(Box::new(query))
                }
            };
            format!("{}", ast_count)
        }
        Err(e) => {
            warn!("parser error: {:?}, manually compose query string", e);
            format!("SELECT COUNT(*) FROM ({}) as CXTMPTAB_COUNT", sql.as_str())
        }
    };

    // HACK: Some dialect (e.g. Oracle) does not support "AS" for alias
    // Hard code "(subquery) alias" instead of output "(subquery) AS alias"
    #[cfg(feature = "src_oracle")]
    if dialect.type_id() == (OracleDialect {}.type_id()) {
        sql = sql.replace(" AS", "");
    }

    debug!("Transformed count query: {}", sql);
    CXQuery::Wrapped(sql)
}

#[throws(ConnectorXError)]
pub fn limit1_query<T: Dialect>(sql: &CXQuery<String>, dialect: &T) -> CXQuery<String> {
    trace!("Incoming query: {}", sql);

    let sql = match Parser::parse_sql(dialect, sql.as_str()) {
        Ok(mut ast) => {
            if ast.len() != 1 {
                throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string()));
            }

            match &mut ast[0] {
                Statement::Query(q) => {
                    q.limit = Some(Expr::Value(Value::Number("1".to_string(), false)));
                }
                _ => throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string())),
            };

            format!("{}", ast[0])
        }
        Err(e) => {
            warn!("parser error: {:?}, manually compose query string", e);
            format!("{} LIMIT 1", sql.as_str())
        }
    };

    debug!("Transformed limit 1 query: {}", sql);
    CXQuery::Wrapped(sql)
}

#[throws(ConnectorXError)]
#[cfg(feature = "src_oracle")]
pub fn limit1_query_oracle(sql: &CXQuery<String>) -> CXQuery<String> {
    trace!("Incoming oracle query: {}", sql);

    let ast = Parser::parse_sql(&OracleDialect {}, sql.as_str())?;
    if ast.len() != 1 {
        throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string()));
    }
    let ast_part: Statement;
    let mut query = ast[0]
        .as_query()
        .ok_or_else(|| ConnectorXError::SqlQueryNotSupported(sql.to_string()))?
        .clone();

    let selection = Expr::BinaryOp {
        left: Box::new(Expr::CompoundIdentifier(vec![Ident {
            value: "rownum".to_string(),
            quote_style: None,
        }])),
        op: BinaryOperator::Eq,
        right: Box::new(Expr::Value(Value::Number("1".to_string(), false))),
    };
    ast_part = wrap_query(&mut query, vec![SelectItem::Wildcard], Some(selection), "");

    let sql = format!("{}", ast_part).replace(" AS", "");
    debug!("Transformed limit 1 query: {}", sql);
    CXQuery::Wrapped(sql)
}

#[throws(ConnectorXError)]
pub fn limit1_query_mssql(sql: &CXQuery<String>) -> CXQuery<String> {
    trace!("Incoming query: {}", sql);

    let mut ast = Parser::parse_sql(&MsSqlDialect {}, sql.as_str())?;
    if ast.len() != 1 {
        throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string()));
    }

    match &mut ast[0] {
        Statement::Query(q) => {
            match q.body {
                SetExpr::Select(ref mut s) => {
                    s.top = Some(Top {
                        with_ties: false,
                        percent: false,
                        quantity: Some(Expr::Value(Value::Number("1".to_string(), false))),
                    })
                }
                _ => throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string())),
            };
        }
        _ => throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string())),
    };

    let sql = format!("{}", ast[0]);
    debug!("Transformed limit 1 query: {}", sql);
    CXQuery::Wrapped(sql)
}

#[throws(ConnectorXError)]
pub fn single_col_partition_query<T: Dialect>(
    sql: &str,
    col: &str,
    lower: i64,
    upper: i64,
    dialect: &T,
) -> String {
    trace!("Incoming query: {}", sql);
    const PART_TMP_TAB_NAME: &str = "CXTMPTAB_PART";

    #[allow(unused_mut)]
    let mut tsql = match Parser::parse_sql(dialect, sql) {
        Ok(mut ast) => {
            if ast.len() != 1 {
                throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string()));
            }

            let mut query = ast[0]
                .as_query()
                .ok_or_else(|| ConnectorXError::SqlQueryNotSupported(sql.to_string()))?
                .clone();

            let select = query
                .as_select_mut()
                .ok_or_else(|| ConnectorXError::SqlQueryNotSupported(sql.to_string()))?
                .clone();

            let ast_part: Statement;

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

            if query.limit.is_none() && select.top.is_none() && !query.order_by.is_empty() {
                // order by in a partition query does not make sense because partition is unordered.
                // clear the order by beceause mssql does not support order by in a derived table.
                // also order by in the derived table does not make any difference.
                query.order_by.clear();
            }

            ast_part = wrap_query(
                &mut query,
                vec![SelectItem::Wildcard],
                Some(selection),
                PART_TMP_TAB_NAME,
            );
            format!("{}", ast_part)
        }
        Err(e) => {
            warn!("parser error: {:?}, manually compose query string", e);
            format!("SELECT * FROM ({}) AS CXTMPTAB_PART WHERE CXTMPTAB_PART.{} >= {} AND CXTMPTAB_PART.{} < {}", sql, col, lower, col, upper)
        }
    };

    // HACK: Some dialect (e.g. Oracle) does not support "AS" for alias
    // Hard code "(subquery) alias" instead of output "(subquery) AS alias"
    #[cfg(feature = "src_oracle")]
    if dialect.type_id() == (OracleDialect {}.type_id()) {
        tsql = tsql.replace(" AS", "")
    }

    debug!("Transformed single column partition query: {}", tsql);
    tsql
}

#[throws(ConnectorXError)]
pub fn get_partition_range_query<T: Dialect>(sql: &str, col: &str, dialect: &T) -> String {
    trace!("Incoming query: {}", sql);
    const RANGE_TMP_TAB_NAME: &str = "CXTMPTAB_RANGE";

    #[allow(unused_mut)]
    let mut tsql = match Parser::parse_sql(dialect, sql) {
        Ok(mut ast) => {
            if ast.len() != 1 {
                throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string()));
            }

            let mut query = ast[0]
                .as_query()
                .ok_or_else(|| ConnectorXError::SqlQueryNotSupported(sql.to_string()))?
                .clone();
            let ast_range: Statement;
            query.order_by = vec![];
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
            ast_range = wrap_query(&mut query, projection, None, RANGE_TMP_TAB_NAME);
            format!("{}", ast_range)
        }
        Err(e) => {
            warn!("parser error: {:?}, manually compose query string", e);
            format!(
                "SELECT MIN({}.{}) as min, MAX({}.{}) as max FROM ({}) AS {}",
                RANGE_TMP_TAB_NAME, col, RANGE_TMP_TAB_NAME, col, sql, RANGE_TMP_TAB_NAME
            )
        }
    };

    // HACK: Some dialect (e.g. Oracle) does not support "AS" for alias
    // Hard code "(subquery) alias" instead of output "(subquery) AS alias"
    #[cfg(feature = "src_oracle")]
    if dialect.type_id() == (OracleDialect {}.type_id()) {
        tsql = tsql.replace(" AS", "")
    }

    debug!("Transformed partition range query: {}", tsql);
    tsql
}

#[throws(ConnectorXError)]
pub fn get_partition_range_query_sep<T: Dialect>(
    sql: &str,
    col: &str,
    dialect: &T,
) -> (String, String) {
    trace!("Incoming query: {}", sql);
    const RANGE_TMP_TAB_NAME: &str = "CXTMPTAB_RANGE";

    let (sql_min, sql_max) = match Parser::parse_sql(dialect, sql) {
        Ok(ast) => {
            if ast.len() != 1 {
                throw!(ConnectorXError::SqlQueryNotSupported(sql.to_string()));
            }

            let mut query = ast[0]
                .as_query()
                .ok_or_else(|| ConnectorXError::SqlQueryNotSupported(sql.to_string()))?
                .clone();

            let ast_range_min: Statement;
            let ast_range_max: Statement;

            query.order_by = vec![];
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
            ast_range_min = wrap_query(&mut query, min_proj, None, RANGE_TMP_TAB_NAME);
            ast_range_max = wrap_query(&mut query, max_proj, None, RANGE_TMP_TAB_NAME);
            (format!("{}", ast_range_min), format!("{}", ast_range_max))
        }
        Err(e) => {
            warn!("parser error: {:?}, manually compose query string", e);
            (
                format!(
                    "SELECT MIN({}.{}) as min FROM ({}) AS {}",
                    RANGE_TMP_TAB_NAME, col, sql, RANGE_TMP_TAB_NAME
                ),
                format!(
                    "SELECT MAX({}.{}) as max FROM ({}) AS {}",
                    RANGE_TMP_TAB_NAME, col, sql, RANGE_TMP_TAB_NAME
                ),
            )
        }
    };
    debug!(
        "Transformed separated partition range query: {}, {}",
        sql_min, sql_max
    );
    (sql_min, sql_max)
}
