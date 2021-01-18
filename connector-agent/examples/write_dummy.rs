use connector_agent::connections::DummyConnection;
use connector_agent::writers::{dummy::DummyWriter, Writer};
use connector_agent::{Type, Worker};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let mut dw = DummyWriter::allocate(10, vec![Type; 10]);

    let writers = dw.partition_writer(&[4, 6]);

    writers.into_par_iter().for_each(|writer| Worker::new(writer, DummyConnection).run());

    println!("{:?}", dw.buffer);
}
