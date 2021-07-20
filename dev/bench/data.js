window.BENCHMARK_DATA = {
  "lastUpdate": 1626820506585,
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
      },
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
      },
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
          "id": "b547346ebd558eacb06eb05a1a135759e67640d7",
          "message": "Update README.md",
          "timestamp": "2021-07-19T23:14:48-07:00",
          "tree_id": "9727f0db75edc36f0146fad4f58f2667a4a0d13e",
          "url": "https://github.com/sfu-db/connector-x/commit/b547346ebd558eacb06eb05a1a135759e67640d7"
        },
        "date": 1626762487556,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06710323724676853,
            "unit": "iter/sec",
            "range": "stddev: 0.6328498656636488",
            "extra": "mean: 14.90241068881005 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.061655291495408576,
            "unit": "iter/sec",
            "range": "stddev: 3.3666615608965413",
            "extra": "mean: 16.219208047608845 sec\nrounds: 5"
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
          "id": "f60d8ddacb3fc0441008a0b748ca4065befc35df",
          "message": "rename benchmark",
          "timestamp": "2021-07-20T06:29:16Z",
          "tree_id": "adaccb428fbd710e8132f5024d0bfcf84ec1dbc5",
          "url": "https://github.com/sfu-db/connector-x/commit/f60d8ddacb3fc0441008a0b748ca4065befc35df"
        },
        "date": 1626763074331,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06565370511098195,
            "unit": "iter/sec",
            "range": "stddev: 0.4203461640479994",
            "extra": "mean: 15.2314328385517 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05871895843376647,
            "unit": "iter/sec",
            "range": "stddev: 2.411082806961569",
            "extra": "mean: 17.03027483241167 sec\nrounds: 5"
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
          "id": "161fd5231de9cbc587c6fb69dcdb1ba28fd9e12a",
          "message": "upgrade dependencies",
          "timestamp": "2021-07-20T19:25:18Z",
          "tree_id": "c1577bb1c8528b2646b91ea55117f0a7bd648f26",
          "url": "https://github.com/sfu-db/connector-x/commit/161fd5231de9cbc587c6fb69dcdb1ba28fd9e12a"
        },
        "date": 1626809641134,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06523995287340702,
            "unit": "iter/sec",
            "range": "stddev: 0.4570273709746459",
            "extra": "mean: 15.328030692180619 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06586714596555739,
            "unit": "iter/sec",
            "range": "stddev: 2.619070599381126",
            "extra": "mean: 15.182075757812708 sec\nrounds: 5"
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
          "id": "40247a6cd2c142d0f9cb173c93b8f7243147a5ae",
          "message": "add modin dask polars",
          "timestamp": "2021-07-20T20:41:30Z",
          "tree_id": "8a4986efdfbb3293ae348b13ea90b931364eb394",
          "url": "https://github.com/sfu-db/connector-x/commit/40247a6cd2c142d0f9cb173c93b8f7243147a5ae"
        },
        "date": 1626814401292,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06332504353446622,
            "unit": "iter/sec",
            "range": "stddev: 0.34590418742702733",
            "extra": "mean: 15.791540663619525 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06721572180429009,
            "unit": "iter/sec",
            "range": "stddev: 0.12474862228978162",
            "extra": "mean: 14.877471715793945 sec\nrounds: 5"
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
          "id": "fe7689b764d18f64617e3d48cf054724bc46ad90",
          "message": "fix test",
          "timestamp": "2021-07-20T22:05:51Z",
          "tree_id": "61de064785b8ccb7379ffd6d81e71d69ce7a75b0",
          "url": "https://github.com/sfu-db/connector-x/commit/fe7689b764d18f64617e3d48cf054724bc46ad90"
        },
        "date": 1626819467680,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06306243438101106,
            "unit": "iter/sec",
            "range": "stddev: 0.44936896091229794",
            "extra": "mean: 15.857300940179266 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06188284627662971,
            "unit": "iter/sec",
            "range": "stddev: 1.67596529168373",
            "extra": "mean: 16.159566990984604 sec\nrounds: 5"
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
          "id": "4a35cf09b2cea9dc51dedb15268ad6f3ec1fad5f",
          "message": "fix clippy",
          "timestamp": "2021-07-20T22:23:37Z",
          "tree_id": "632eeaee995f7a60ffaa137053f01615c0da3c45",
          "url": "https://github.com/sfu-db/connector-x/commit/4a35cf09b2cea9dc51dedb15268ad6f3ec1fad5f"
        },
        "date": 1626820505413,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06392422776976202,
            "unit": "iter/sec",
            "range": "stddev: 0.22562385591758063",
            "extra": "mean: 15.643521007429808 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06627823966735341,
            "unit": "iter/sec",
            "range": "stddev: 1.519363519989321",
            "extra": "mean: 15.08790826399345 sec\nrounds: 5"
          }
        ]
      }
    ]
  }
}