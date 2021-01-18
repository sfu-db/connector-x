use connector_agent::data_sources::dummy::DummySource;
use connector_agent::writers::{dummy::DummyWriter, Writer};
use connector_agent::{DataType, Worker};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let mut dw = DummyWriter::allocate(11, vec![DataType::U64; 5]);
    let schema = dw.schema().to_vec();
    let writers = dw.partition_writer(&[4, 7]);

    writers
        .into_par_iter()
        .for_each(|writer| Worker::new(DummySource::new(), writer, schema.clone()).run_safe().expect("Worker failed"));

    println!("{:?}", dw.buffer);
}
