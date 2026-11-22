// A minimal in-process stand-in for a Trino coordinator, so the Trino source,
// its transports and the Trino branches of `get_arrow` and `partition` can be
// exercised by `cargo test --lib` without Docker.
//
// It speaks just enough of the client protocol that prusto uses: the SQL is
// POSTed to `/v1/statement`, and the client follows `nextUri` with GETs until
// a response has none. As with a real coordinator, the first response is a
// queued page without data, data arrives in the pages after it, and the last
// page carries neither columns nor data.

use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

/// The single table served by a [`FakeTrino`]: column `(name, trino type)`
/// pairs and the rows. The first column must be an integer id, which is what
/// the count, min/max and partition queries are answered from.
pub(crate) struct Table {
    pub columns: Vec<(&'static str, &'static str)>,
    pub rows: Vec<Vec<Value>>,
}

/// A table with one column of every Trino type the source maps, served in
/// row order: ids 0 to 4, where id 2 is NULL in every other column.
pub(crate) fn every_type_table() -> Table {
    Table {
        columns: vec![
            ("id", "bigint"),
            ("i", "integer"),
            ("s", "smallint"),
            ("t", "tinyint"),
            ("d", "double"),
            ("r", "real"),
            ("dec", "decimal(10,2)"),
            ("b", "boolean"),
            ("v", "varchar"),
            ("c", "char(1)"),
            ("arr", "array(integer)"),
            ("dt", "date"),
            ("tm", "time(3)"),
            ("ts", "timestamp(3)"),
        ],
        rows: vec![
            vec![
                json!(0),
                json!(1),
                json!(2),
                json!(3),
                json!(1.5),
                json!(2.5),
                json!("12.34"),
                json!(true),
                json!("a"),
                json!("x"),
                json!([1, 2]),
                json!("2021-01-02"),
                json!("01:02:03.000"),
                json!("2021-01-02 03:04:05.000"),
            ],
            vec![
                json!(1),
                json!(-1),
                json!(-2),
                json!(-3),
                json!(-1.5),
                json!(0.25),
                json!("-0.50"),
                json!(false),
                json!("bb"),
                json!("y"),
                json!([]),
                json!("1999-12-31"),
                json!("23:59:59.999"),
                json!("1999-12-31 23:59:59.999"),
            ],
            std::iter::once(json!(2))
                .chain(std::iter::repeat_n(Value::Null, 13))
                .collect(),
            vec![
                json!(3),
                json!(2147483647),
                json!(32767),
                json!(127),
                json!(1e300),
                json!(-0.5),
                json!("0.00"),
                json!(true),
                json!(""),
                json!(" "),
                json!([3]),
                json!("1970-01-01"),
                json!("00:00:00.000"),
                json!("1970-01-01 00:00:00.000"),
            ],
            vec![
                json!(4),
                json!(-2147483648),
                json!(-32768),
                json!(-128),
                json!(0.125),
                json!(8.5),
                json!("99999999.99"),
                json!(false),
                json!("üñí"),
                json!("z"),
                json!([4, 5, 6]),
                json!("2038-01-19"),
                json!("12:00:00.500"),
                json!("2038-01-19 03:14:07.500"),
            ],
        ],
    }
}

struct State {
    table: Table,
    page_size: usize,
    port: u16,
    next_id: AtomicUsize,
    pages: Mutex<HashMap<usize, Vec<String>>>,
    statements: Mutex<Vec<String>>,
}

/// A fake Trino server listening on a free loopback port for the rest of the
/// test process.
pub(crate) struct FakeTrino {
    state: Arc<State>,
}

impl FakeTrino {
    /// Serve `table`, returning at most `page_size` rows per page.
    pub(crate) fn start(table: Table, page_size: usize) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let state = Arc::new(State {
            table,
            page_size,
            port: listener.local_addr().unwrap().port(),
            next_id: AtomicUsize::new(0),
            pages: Mutex::new(HashMap::new()),
            statements: Mutex::new(vec![]),
        });
        let server = state.clone();
        thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let server = server.clone();
                thread::spawn(move || handle(&server, stream));
            }
        });
        FakeTrino { state }
    }

    /// A `trino://` connection string for this server.
    pub(crate) fn url(&self) -> String {
        format!("trino://test@127.0.0.1:{}/memory", self.state.port)
    }

    /// Every SQL statement received so far, in order.
    pub(crate) fn statements(&self) -> Vec<String> {
        self.state.statements.lock().unwrap().clone()
    }
}

fn handle(state: &State, stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).unwrap_or(0) == 0 {
        return;
    }
    let mut content_length = 0;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.trim().parse().unwrap();
            }
        }
    }
    let mut body = vec![0; content_length];
    reader.read_exact(&mut body).unwrap();

    let mut parts = request_line.split_whitespace();
    let (method, path) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
    let (status, body) = match (method, path) {
        ("POST", "/v1/statement") => statement(state, String::from_utf8(body).unwrap()),
        ("GET", path) if path.starts_with("/v1/next/") => next_page(state, path),
        _ => ("404 Not Found", "not found".to_string()),
    };
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let mut stream = stream;
    let _ = stream.write_all(response.as_bytes());
}

fn statement(state: &State, sql: String) -> (&'static str, String) {
    state.statements.lock().unwrap().push(sql.clone());
    if sql.contains("no_such_table") {
        return ("400 Bad Request", "line 1:15: Table does not exist".into());
    }

    let upper = sql.to_uppercase();
    let rows = filter(&state.table.rows, &sql);
    let ids = rows.iter().filter_map(|r| r[0].as_i64());
    let (columns, data): (Vec<(&str, &str)>, Vec<Vec<Value>>) = if upper.contains("LIMIT 0") {
        (state.table.columns.clone(), vec![])
    } else if upper.contains("COUNT(*)") {
        (vec![("_col0", "bigint")], vec![vec![json!(rows.len())]])
    } else if upper.contains("MIN(") {
        let (min, max) = (ids.clone().min(), ids.max());
        (
            vec![("_col0", "bigint"), ("_col1", "bigint")],
            vec![vec![json!(min), json!(max)]],
        )
    } else {
        (state.table.columns.clone(), rows)
    };

    let id = state.next_id.fetch_add(1, Ordering::SeqCst);
    let mut pages = vec![];
    let chunks: Vec<&[Vec<Value>]> = if data.is_empty() {
        vec![&[]]
    } else {
        data.chunks(state.page_size).collect()
    };
    for chunk in chunks {
        pages.push(json!({ "columns": column_json(&columns), "data": chunk }));
    }
    pages.push(json!({}));

    let n = pages.len();
    let pages: Vec<String> = pages
        .into_iter()
        .enumerate()
        .map(|(i, page)| result(state, id, i + 1, i + 1 < n, page, "RUNNING"))
        .collect();
    state.pages.lock().unwrap().insert(id, pages);
    ("200 OK", result(state, id, 0, true, json!({}), "QUEUED"))
}

fn next_page(state: &State, path: &str) -> (&'static str, String) {
    let mut parts = path.trim_start_matches("/v1/next/").split('/');
    let id: usize = parts.next().unwrap().parse().unwrap();
    let page: usize = parts.next().unwrap().parse().unwrap();
    let pages = state.pages.lock().unwrap();
    ("200 OK", pages[&id][page - 1].clone())
}

/// Wraps `page` (columns and data, or nothing) in a QueryResults document.
/// The page at index `page` links to index `page + 1` when `has_next`.
fn result(
    state: &State,
    id: usize,
    page: usize,
    has_next: bool,
    extra: Value,
    state_name: &str,
) -> String {
    let mut doc = json!({
        "id": format!("q{id}"),
        "infoUri": format!("http://127.0.0.1:{}/ui/q{id}", state.port),
        "stats": {
            "state": if has_next { state_name } else { "FINISHED" },
            "queued": state_name == "QUEUED", "scheduled": true, "nodes": 1,
            "totalSplits": 0, "queuedSplits": 0, "runningSplits": 0, "completedSplits": 0,
            "cpuTimeMillis": 0, "wallTimeMillis": 0, "queuedTimeMillis": 0,
            "elapsedTimeMillis": 0, "processedRows": 0, "processedBytes": 0,
            "peakMemoryBytes": 0, "spilledBytes": 0
        },
        "warnings": []
    });
    if has_next {
        doc["nextUri"] = json!(format!(
            "http://127.0.0.1:{}/v1/next/{id}/{}",
            state.port,
            page + 1
        ));
    }
    if let Value::Object(extra) = extra {
        doc.as_object_mut().unwrap().extend(extra);
    }
    doc.to_string()
}

fn column_json(columns: &[(&str, &str)]) -> Value {
    Value::Array(
        columns
            .iter()
            .map(|(name, ty)| json!({ "name": name, "type": ty, "typeSignature": signature(ty) }))
            .collect(),
    )
}

/// The typeSignature for a type such as `bigint`, `char(1)` or `decimal(10,2)`.
fn signature(ty: &str) -> Value {
    let (raw, args) = match ty.split_once('(') {
        Some((raw, args)) => (raw, args.trim_end_matches(')')),
        None => (ty, ""),
    };
    let arguments: Vec<Value> = match raw {
        "array" => vec![json!({ "kind": "TYPE", "value": signature(args) })],
        "time" | "timestamp" => vec![],
        _ => args
            .split(',')
            .filter(|a| !a.is_empty())
            .map(|a| json!({ "kind": "LONG", "value": a.trim().parse::<u64>().unwrap() }))
            .collect(),
    };
    json!({ "rawType": raw, "arguments": arguments })
}

/// Applies the `id >= lower AND id < upper` filter of a partition query
/// (see `single_col_partition_query`); other queries return every row.
fn filter(rows: &[Vec<Value>], sql: &str) -> Vec<Vec<Value>> {
    let Some((_, cond)) = sql.rsplit_once("WHERE") else {
        return rows.to_vec();
    };
    let bounds: Vec<i64> = cond
        .split(|c: char| !(c.is_ascii_digit() || c == '-'))
        .filter_map(|t| t.parse().ok())
        .collect();
    let (lower, upper) = (bounds[0], bounds[bounds.len() - 1]);
    rows.iter()
        .filter(|r| r[0].as_i64().is_some_and(|id| lower <= id && id < upper))
        .cloned()
        .collect()
}
