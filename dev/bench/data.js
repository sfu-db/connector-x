window.BENCHMARK_DATA = {
  "lastUpdate": 1626821948692,
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
      }
    ]
  }
}