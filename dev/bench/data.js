window.BENCHMARK_DATA = {
  "lastUpdate": 1626822634624,
  "repoUrl": "https://github.com/sfu-db/connector-x",
  "entries": {
    "ConnectorX TPC-H Scale@1 Benchmarks": [
      {
        "commit": {
          "author": {
            "email": "youngw@sfu.ca",
            "name": "Weiyuan Wu",
            "username": "dovahcrow"
          },
          "committer": {
            "email": "youngw@sfu.ca",
            "name": "Weiyuan Wu",
            "username": "dovahcrow"
          },
          "distinct": false,
          "id": "19af6329cb253d19bbdf4cadc84ef1ef12c3b4a4",
          "message": "0.2.0-alpha.3",
          "timestamp": "2021-07-20T22:35:46Z",
          "tree_id": "9b46f1b09cbaff4548363e45c67ee8623fae4987",
          "url": "https://github.com/sfu-db/connector-x/commit/19af6329cb253d19bbdf4cadc84ef1ef12c3b4a4"
        },
        "date": 1626821947314,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06654699023020173,
            "unit": "iter/sec",
            "range": "stddev: 0.27978951961334764",
            "extra": "mean: 15.026975623401814 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05570817624901951,
            "unit": "iter/sec",
            "range": "stddev: 4.0734554363587625",
            "extra": "mean: 17.950686368369496 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "youngw@sfu.ca",
            "name": "Weiyuan Wu",
            "username": "dovahcrow"
          },
          "committer": {
            "email": "youngw@sfu.ca",
            "name": "Weiyuan Wu",
            "username": "dovahcrow"
          },
          "distinct": true,
          "id": "3a3e3f02940cd4c56b438f9d6eccba0354049e4e",
          "message": "add polars rust test",
          "timestamp": "2021-07-20T22:54:40Z",
          "tree_id": "d97ac659a66392e3f5f589bfa791698cc43a52f5",
          "url": "https://github.com/sfu-db/connector-x/commit/3a3e3f02940cd4c56b438f9d6eccba0354049e4e"
        },
        "date": 1626822633559,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0649936848928417,
            "unit": "iter/sec",
            "range": "stddev: 0.34118691951634206",
            "extra": "mean: 15.386110229766928 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06440907427849822,
            "unit": "iter/sec",
            "range": "stddev: 2.775012180857785",
            "extra": "mean: 15.525762653816491 sec\nrounds: 5"
          }
        ]
      }
    ]
  }
}