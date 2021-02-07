use connector_agent::data_sources::{
    postgres::{PostgresDataSource, PostgresDataSourceBuilder},
    DataSource, Produce,
};
use connector_agent::writers::dummy::U64Writer;
use connector_agent::{DataType, Dispatcher};
use ndarray::array;
#[test]
fn test_postgres() {
    let schema = vec![DataType::U64; 5];
    let dispatcher = Dispatcher::new(
        PostgresDataSourceBuilder::new("host=localhost user=postgres dbname=test_table_1 port=5432 password=wjz283200"),
        schema,
        vec!["select * from test_table_1".to_string()]);

    let dw = dispatcher
        .run_checked::<U64Writer>()
        .expect("run dispatcher");

    assert_eq!(
        array![
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
            [25, 26, 27, 28, 29],
            [30, 31, 32, 33, 34],
            [35, 36, 37, 38, 39],
            [40, 41, 42, 43, 44],
            [45, 46, 47, 48, 49],
            [50, 51, 52, 53, 54],
        ],
        dw.buffer()
    );
}