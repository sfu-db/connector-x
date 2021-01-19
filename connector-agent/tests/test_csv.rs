use connector_agent::data_sources::csv::CSVSource;
use connector_agent::data_sources::dummy::U64CounterSource;

#[test]
fn load_csv() {
    CSVSource::new("./tests/uspop.csv");
}