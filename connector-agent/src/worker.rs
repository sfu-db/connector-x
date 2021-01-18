use crate::connections::Connection;
use crate::types::Type;
use crate::writers::PartitionWriter;

pub struct Worker<P, C> {
    partition_writer: P,
    query: String,
    conn: C,
    type_info: Vec<Type>,
}

impl<P, C> Worker<P, C> {
    pub fn new(writer: P, conn: C) -> Self {
        Worker {
            partition_writer: writer,
            query: "".to_string(),
            conn: conn,
            type_info: vec![],
        }
    }
}

impl<'a, P, C> Worker<P, C>
where
    P: PartitionWriter<'a>,
    C: Connection,
{
    pub fn run(mut self) {
        let _ = self.conn.query(&self.query);
        for row in 0..self.partition_writer.nrows() {
            for col in 0..self.partition_writer.ncols() {
                unsafe { self.partition_writer.write(row, col, row * self.partition_writer.ncols() + col) }
            }
        }
    }
}
