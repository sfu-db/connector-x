window.BENCHMARK_DATA = {
  "lastUpdate": 1626761974140,
  "repoUrl": "https://github.com/sfu-db/connector-x",
  "entries": {
    "ConnectorX TPC-H Scale1 Benchmarks": [
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
          "id": "64b036da09697d5229174ce12029198e3a0f3fef",
          "message": "generate benchmark report when commit and pull request",
          "timestamp": "2021-07-19T21:13:03Z",
          "tree_id": "c8867c4e1b7a0a06ae4d633d9e89094965f7c2df",
          "url": "https://github.com/sfu-db/connector-x/commit/64b036da09697d5229174ce12029198e3a0f3fef"
        },
        "date": 1626729725991,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06545517836696285,
            "unit": "iter/sec",
            "range": "stddev: 0.24043822265221626",
            "extra": "mean: 15.277630050806328 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.060696623342137636,
            "unit": "iter/sec",
            "range": "stddev: 2.597003040953377",
            "extra": "mean: 16.475381082785315 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "64ca37e94fd51fc63bca337b1f74214b88e76f89",
          "message": "Merge pull request #100 from sfu-db/feat/benchmark_bot\n\nFeat/benchmark bot",
          "timestamp": "2021-07-19T19:54:53-07:00",
          "tree_id": "36eeff939bec1bbc5584b3a039dfab46b4e7e48d",
          "url": "https://github.com/sfu-db/connector-x/commit/64ca37e94fd51fc63bca337b1f74214b88e76f89"
        },
        "date": 1626750206747,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0608387087642097,
            "unit": "iter/sec",
            "range": "stddev: 0.6522933649056051",
            "extra": "mean: 16.436903746193273 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05540078267880269,
            "unit": "iter/sec",
            "range": "stddev: 2.776374003088519",
            "extra": "mean: 18.050286505836993 sec\nrounds: 5"
          }
        ]
      }
    ],
    "ConnectorX TPCH Scale1 Benchmarks": [
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a4468ff4df7f5c6eaf85c38a6ed3dbdf36cfc6ac",
          "message": "Create Types.md",
          "timestamp": "2021-07-19T23:10:52-07:00",
          "tree_id": "46d574e8deea439c3c7c360163cdf43156c3f01f",
          "url": "https://github.com/sfu-db/connector-x/commit/a4468ff4df7f5c6eaf85c38a6ed3dbdf36cfc6ac"
        },
        "date": 1626761972982,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06344981981623901,
            "unit": "iter/sec",
            "range": "stddev: 0.47703705990145423",
            "extra": "mean: 15.760486048599706 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06368878499208686,
            "unit": "iter/sec",
            "range": "stddev: 0.6909908674764333",
            "extra": "mean: 15.701351503632031 sec\nrounds: 5"
          }
        ]
      }
    ]
  }
}