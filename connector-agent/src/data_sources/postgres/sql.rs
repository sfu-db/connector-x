use log::debug;
use sqlparser::ast::{
    Expr, Function, FunctionArg, Ident, ObjectName, SelectItem, SetExpr, Statement, Value,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub fn count_query(sql: &str) -> String {
    debug!("Incoming query: {}", sql);

    let dialect = GenericDialect {};

    let mut ast = Parser::parse_sql(&dialect, sql).unwrap();

    match &mut ast[0] {
        Statement::Query(q) => {
            q.order_by = vec![];
            match &mut q.body {
                SetExpr::Select(select) => {
                    select.distinct = false;
                    select.top = None;
                    select.projection = vec![SelectItem::UnnamedExpr(Expr::Function(Function {
                        name: ObjectName(vec![Ident {
                            value: "count".to_string(),
                            quote_style: None,
                        }]),
                        args: vec![FunctionArg::Unnamed(Expr::Wildcard)],
                        over: None,
                        distinct: false,
                    }))];
                    select.sort_by = vec![];
                }
                _ => {}
            }
        }
        _ => {}
    };

    let sql = format!("{}", ast[0]);
    debug!("Transformed query: {}", sql);
    sql
}

pub fn limit1_query(sql: &str) -> String {
    debug!("Incoming query: {}", sql);

    let dialect = GenericDialect {};

    let mut ast = Parser::parse_sql(&dialect, sql).unwrap();

    match &mut ast[0] {
        Statement::Query(q) => {
            q.limit = Some(Expr::Value(Value::Number("1".to_string(), false)));
        }
        _ => {}
    };

    let sql = format!("{}", ast[0]);
    debug!("Transformed query: {}", sql);
    sql
}
