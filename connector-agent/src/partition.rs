use log::{debug, trace};
use sqlparser::ast::{BinaryOperator, Expr, Ident, SetExpr, Statement, Value};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

pub fn pg_single_col_partition_query(query: &str, col: &str, lower: i64, upper: i64) -> String {
    trace!("Incoming query: {}", query);

    let dialect = PostgreSqlDialect {};

    let mut ast = Parser::parse_sql(&dialect, query).unwrap();

    match &mut ast[0] {
        Statement::Query(q) => match &mut q.body {
            SetExpr::Select(select) => {
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
            _ => {}
        },
        _ => {}
    };

    let sql = format!("{}", ast[0]);
    debug!("Transformed query: {}", sql);
    sql
}
