use connector_agent::data_sources::dummy::U64CounterSource;
use connector_agent::writers::{dummy::U64Writer, Writer};
use connector_agent::{DataType, Worker};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let mut dw = U64Writer::allocate(11, vec![DataType::U64; 5]).unwrap();
    let schema = dw.schema().to_vec();
    let writers = dw.partition_writer(&[4, 7]);

    writers
        .into_par_iter()
        .for_each(|writer| Worker::new(U64CounterSource::new(), writer, schema.clone()).run_safe().expect("Worker failed"));

    println!("{:?}", dw.buffer());
}
