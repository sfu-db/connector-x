use log::{debug, trace};
use sqlparser::ast::{Expr, SetExpr, Statement, Value};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

pub fn pg_single_col_partition_query(query: &str, col: &str, lower: i64, upper: i64) -> String {
    trace!("Incoming query: {}", query);

    let dialect = PostgreSqlDialect {};

    let mut ast = Parser::parse_sql(&dialect, query).unwrap();

    match &mut ast[0] {
        Statement::Query(q) => match &mut q.body {
            SetExpr::Select(select) => {
                let cur_selection = select.selection.as_ref();
                let mut _partition_query = format!("{} >= {} and {} < {}", col, lower, col, upper);
                if !cur_selection.is_none() {
                    _partition_query = format!(
                        "{} and {} >= {} and {} < {}",
                        cur_selection.unwrap(),
                        col,
                        lower,
                        col,
                        upper
                    );
                }
                select.selection = Some(Expr::Value(Value::Number(_partition_query, false)));
            }
            _ => {}
        },
        _ => {}
    };

    let sql = format!("{}", ast[0]);
    debug!("Transformed query: {}", sql);
    sql
}
