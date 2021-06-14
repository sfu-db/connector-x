pub enum SourceType {
    Postgres,
    Sqlite,
}

impl From<&str> for SourceType {
    fn from(conn: &str) -> SourceType {
        if conn.starts_with("postgresql") {
            return SourceType::Postgres;
        }
        SourceType::Sqlite
    }
}
