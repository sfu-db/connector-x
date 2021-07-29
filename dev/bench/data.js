window.BENCHMARK_DATA = {
  "lastUpdate": 1627584819979,
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
          "distinct": false,
          "id": "3a3e3f02940cd4c56b438f9d6eccba0354049e4e",
          "message": "add polars rust test",
          "timestamp": "2021-07-20T22:54:40Z",
          "tree_id": "d97ac659a66392e3f5f589bfa791698cc43a52f5",
          "url": "https://github.com/sfu-db/connector-x/commit/3a3e3f02940cd4c56b438f9d6eccba0354049e4e"
        },
        "date": 1626823340071,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06506041915055688,
            "unit": "iter/sec",
            "range": "stddev: 0.5886551795399168",
            "extra": "mean: 15.370328274182976 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06385055601593312,
            "unit": "iter/sec",
            "range": "stddev: 2.520911337842341",
            "extra": "mean: 15.661570742633193 sec\nrounds: 5"
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
          "id": "165818839d76abcb2fd8d29e6bf359da4678c615",
          "message": "do not clean benchmark results",
          "timestamp": "2021-07-20T23:30:20Z",
          "tree_id": "64f8d5997c25dbfcaa4127ee7604670483285ae5",
          "url": "https://github.com/sfu-db/connector-x/commit/165818839d76abcb2fd8d29e6bf359da4678c615"
        },
        "date": 1626824502906,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0651573307081698,
            "unit": "iter/sec",
            "range": "stddev: 0.233511244816218",
            "extra": "mean: 15.347467263182626 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0665763889429248,
            "unit": "iter/sec",
            "range": "stddev: 1.170653135839821",
            "extra": "mean: 15.020340031618252 sec\nrounds: 5"
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
          "distinct": false,
          "id": "165818839d76abcb2fd8d29e6bf359da4678c615",
          "message": "do not clean benchmark results",
          "timestamp": "2021-07-20T23:30:20Z",
          "tree_id": "64f8d5997c25dbfcaa4127ee7604670483285ae5",
          "url": "https://github.com/sfu-db/connector-x/commit/165818839d76abcb2fd8d29e6bf359da4678c615"
        },
        "date": 1626827442086,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06405913703876019,
            "unit": "iter/sec",
            "range": "stddev: 0.2764713459612062",
            "extra": "mean: 15.61057557479944 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.055133531381832086,
            "unit": "iter/sec",
            "range": "stddev: 3.3525872645470467",
            "extra": "mean: 18.137782488018274 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "xiaoying_wang@sfu.ca",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "xiaoying_wang@sfu.ca",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "7ee5f85c8b79d71b45d651d9e081f3a354a57e32",
          "message": "add benchmark for clickhouse",
          "timestamp": "2021-07-21T23:59:24Z",
          "tree_id": "5ee3ef4d053595dc5bc7a8a3e3a2c071e7c4b181",
          "url": "https://github.com/sfu-db/connector-x/commit/7ee5f85c8b79d71b45d651d9e081f3a354a57e32"
        },
        "date": 1626912774822,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0462878353500854,
            "unit": "iter/sec",
            "range": "stddev: 1.0606101578752007",
            "extra": "mean: 21.603948260634205 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.04708311690084669,
            "unit": "iter/sec",
            "range": "stddev: 1.1096567777567008",
            "extra": "mean: 21.239035684615374 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "quchangbo1990@gmail.com",
            "name": "cbqu",
            "username": "CBQu"
          },
          "committer": {
            "email": "quchangbo1990@gmail.com",
            "name": "cbqu",
            "username": "CBQu"
          },
          "distinct": true,
          "id": "8a3c8deac764311c33bd046a72b80f6fa89e91ed",
          "message": "upload tpch scripts for redshift",
          "timestamp": "2021-07-22T22:43:21Z",
          "tree_id": "c849a58872a9c0d233e096c32a3ae4788ab27147",
          "url": "https://github.com/sfu-db/connector-x/commit/8a3c8deac764311c33bd046a72b80f6fa89e91ed"
        },
        "date": 1626994705571,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.04866432217515626,
            "unit": "iter/sec",
            "range": "stddev: 0.839534895258935",
            "extra": "mean: 20.548935139807874 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.04528594212869319,
            "unit": "iter/sec",
            "range": "stddev: 1.9772716110646915",
            "extra": "mean: 22.08190782822203 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "quchangbo1990@gmail.com",
            "name": "CbQu",
            "username": "CBQu"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "862ad56161bad6bd0fd8724ad3bf9a2ea53c9309",
          "message": "Update benchmark.md for redshift",
          "timestamp": "2021-07-22T16:15:28-07:00",
          "tree_id": "a62f0f8487cb537f0d1b5b609152af63dfd8a4aa",
          "url": "https://github.com/sfu-db/connector-x/commit/862ad56161bad6bd0fd8724ad3bf9a2ea53c9309"
        },
        "date": 1626996418882,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06181545069203178,
            "unit": "iter/sec",
            "range": "stddev: 0.5626791148787815",
            "extra": "mean: 16.177185295987876 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05572187318113634,
            "unit": "iter/sec",
            "range": "stddev: 6.068503845222284",
            "extra": "mean: 17.946273929974996 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "xiaoying_wang@sfu.ca",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "xiaoying_wang@sfu.ca",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "49adeee3e6c88f9e6f4fb04d186169b7b8fb2016",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-agent into main",
          "timestamp": "2021-07-23T01:19:46Z",
          "tree_id": "1073b625faf28f993bb653cf3f645a52dee91f40",
          "url": "https://github.com/sfu-db/connector-x/commit/49adeee3e6c88f9e6f4fb04d186169b7b8fb2016"
        },
        "date": 1627003912613,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06279418106073886,
            "unit": "iter/sec",
            "range": "stddev: 0.6772186770942324",
            "extra": "mean: 15.925042465841397 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.058173096568556536,
            "unit": "iter/sec",
            "range": "stddev: 4.29685683315079",
            "extra": "mean: 17.19007683941163 sec\nrounds: 5"
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
          "id": "e317d9be51fe6d01ddb4a48c0c1e384a4e4554e4",
          "message": "moving rust dependencies behind feature gates",
          "timestamp": "2021-07-25T03:39:47Z",
          "tree_id": "287ad3a4de4b07f54002d3ee3d57f7636131e235",
          "url": "https://github.com/sfu-db/connector-x/commit/e317d9be51fe6d01ddb4a48c0c1e384a4e4554e4"
        },
        "date": 1627184931496,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06682051542334197,
            "unit": "iter/sec",
            "range": "stddev: 0.34804308096074293",
            "extra": "mean: 14.965463730180636 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05073767198445093,
            "unit": "iter/sec",
            "range": "stddev: 3.3702994603162715",
            "extra": "mean: 19.70922119379975 sec\nrounds: 5"
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
          "id": "7e4f6cab3b0085016e8b83efcf5c165fb8e75a8f",
          "message": "now source/destination/transport have their own error",
          "timestamp": "2021-07-25T08:12:01Z",
          "tree_id": "b4d7f71c6aed1422e7e58b5a3def40c28bf30fcd",
          "url": "https://github.com/sfu-db/connector-x/commit/7e4f6cab3b0085016e8b83efcf5c165fb8e75a8f"
        },
        "date": 1627201261967,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.061868867943704185,
            "unit": "iter/sec",
            "range": "stddev: 0.4750916135455115",
            "extra": "mean: 16.163218000205234 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.052641801309695174,
            "unit": "iter/sec",
            "range": "stddev: 3.814288805827136",
            "extra": "mean: 18.99631044380367 sec\nrounds: 5"
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
          "id": "ad47e84e65b1c8f0a98f8e5e5bbccb12e308c370",
          "message": "move errors to each source/destination",
          "timestamp": "2021-07-25T09:45:02Z",
          "tree_id": "9b9b70ef8e24e7a695e4510f096e4357a4f70d0b",
          "url": "https://github.com/sfu-db/connector-x/commit/ad47e84e65b1c8f0a98f8e5e5bbccb12e308c370"
        },
        "date": 1627206825325,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06385256047322888,
            "unit": "iter/sec",
            "range": "stddev: 0.6583740547240273",
            "extra": "mean: 15.661079095164315 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05623531543006889,
            "unit": "iter/sec",
            "range": "stddev: 3.7469952509677564",
            "extra": "mean: 17.78242003894411 sec\nrounds: 5"
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
          "id": "b38256c9930ca6b9e968ab0404985b91cd33d2fc",
          "message": "further remove mandatory dependencies",
          "timestamp": "2021-07-25T10:02:29Z",
          "tree_id": "49bee12fe529dfdb16bcb10dca50e5db0a588475",
          "url": "https://github.com/sfu-db/connector-x/commit/b38256c9930ca6b9e968ab0404985b91cd33d2fc"
        },
        "date": 1627207861261,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0642028585989691,
            "unit": "iter/sec",
            "range": "stddev: 0.39283933935818854",
            "extra": "mean: 15.575630459794775 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06270361113005855,
            "unit": "iter/sec",
            "range": "stddev: 1.9954720175419516",
            "extra": "mean: 15.94804480918683 sec\nrounds: 5"
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
          "id": "dfd74fbacadcb23eb0db3ca3cdc33ea28c5adfbf",
          "message": "make r2d2 optional",
          "timestamp": "2021-07-25T10:07:06Z",
          "tree_id": "8d1c1f063ce2eecdc26920448ec5e2bddd2fd1fb",
          "url": "https://github.com/sfu-db/connector-x/commit/dfd74fbacadcb23eb0db3ca3cdc33ea28c5adfbf"
        },
        "date": 1627208385069,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06300209399473912,
            "unit": "iter/sec",
            "range": "stddev: 0.0917274463482906",
            "extra": "mean: 15.872488303063438 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06384835471147265,
            "unit": "iter/sec",
            "range": "stddev: 1.8317102738261437",
            "extra": "mean: 15.66211070776917 sec\nrounds: 5"
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
          "id": "949a2aaaf55544d02be87fe974779c56aea4dfe8",
          "message": "change author to sfu-db",
          "timestamp": "2021-07-25T10:14:15Z",
          "tree_id": "80607d9286c927694dd19223de9826ad2100bcc3",
          "url": "https://github.com/sfu-db/connector-x/commit/949a2aaaf55544d02be87fe974779c56aea4dfe8"
        },
        "date": 1627208905516,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0586049477583023,
            "unit": "iter/sec",
            "range": "stddev: 0.7721077855156782",
            "extra": "mean: 17.063405706360935 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06230110794171541,
            "unit": "iter/sec",
            "range": "stddev: 1.4119373810882498",
            "extra": "mean: 16.051078913966194 sec\nrounds: 5"
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
          "id": "6dbdf679f2b21cc25ecdb41f17658125c3d8f91e",
          "message": "tidy up exports",
          "timestamp": "2021-07-25T17:52:29Z",
          "tree_id": "7ab6b74940d8c35d2c6654eec90145b125e2fe61",
          "url": "https://github.com/sfu-db/connector-x/commit/6dbdf679f2b21cc25ecdb41f17658125c3d8f91e"
        },
        "date": 1627236096046,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.058570398291856146,
            "unit": "iter/sec",
            "range": "stddev: 0.4128592565013035",
            "extra": "mean: 17.07347105643712 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06038444393437088,
            "unit": "iter/sec",
            "range": "stddev: 0.7952973759790952",
            "extra": "mean: 16.560556574584915 sec\nrounds: 5"
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
          "id": "767e1df00c59281fcefd5daf7dd4d5233c7d9b68",
          "message": "let clippy check the whole project",
          "timestamp": "2021-07-25T18:18:38Z",
          "tree_id": "dca2a8aca23c30ec591d11869a566842367e00c5",
          "url": "https://github.com/sfu-db/connector-x/commit/767e1df00c59281fcefd5daf7dd4d5233c7d9b68"
        },
        "date": 1627237689847,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05995763278217722,
            "unit": "iter/sec",
            "range": "stddev: 0.5782623363336292",
            "extra": "mean: 16.678443654254078 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05671674802553579,
            "unit": "iter/sec",
            "range": "stddev: 2.163079858660456",
            "extra": "mean: 17.631476324237884 sec\nrounds: 5"
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
          "id": "57a39161d3c0862b24415c26a79c5a355b7b1635",
          "message": "do not clippy the python package in the rust ci",
          "timestamp": "2021-07-25T18:37:33Z",
          "tree_id": "d9dd714f0a227b6f891ae63edfaef8de877567da",
          "url": "https://github.com/sfu-db/connector-x/commit/57a39161d3c0862b24415c26a79c5a355b7b1635"
        },
        "date": 1627238765261,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.067709242917445,
            "unit": "iter/sec",
            "range": "stddev: 0.13936913385698163",
            "extra": "mean: 14.769032364152372 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0640969244966657,
            "unit": "iter/sec",
            "range": "stddev: 2.41338662225601",
            "extra": "mean: 15.60137257524766 sec\nrounds: 5"
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
          "id": "91858ffd084984e948e14fc50992afb367efd275",
          "message": "Merge pull request #104 from wseaton/postgres_ssl\n\nAdd SSL/TLS support for Postgres connections",
          "timestamp": "2021-07-26T15:12:49-07:00",
          "tree_id": "5ba63ab80206a439d31785ea8cefd636e12575fd",
          "url": "https://github.com/sfu-db/connector-x/commit/91858ffd084984e948e14fc50992afb367efd275"
        },
        "date": 1627338380953,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0659949238700004,
            "unit": "iter/sec",
            "range": "stddev: 0.5763580434319482",
            "extra": "mean: 15.152680560247973 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05198368864558919,
            "unit": "iter/sec",
            "range": "stddev: 3.1877071617839303",
            "extra": "mean: 19.236803429201245 sec\nrounds: 5"
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
          "id": "faa08e9f37b9f595fc6f0f1c3fe95e0b11ba22b1",
          "message": "0.2.0",
          "timestamp": "2021-07-26T22:34:54Z",
          "tree_id": "3f8ec54834bb4525a07fbb3a268ccb16eefebe58",
          "url": "https://github.com/sfu-db/connector-x/commit/faa08e9f37b9f595fc6f0f1c3fe95e0b11ba22b1"
        },
        "date": 1627340349972,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06737283422006901,
            "unit": "iter/sec",
            "range": "stddev: 0.17082373325055286",
            "extra": "mean: 14.84277768000029 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.04538383160890267,
            "unit": "iter/sec",
            "range": "stddev: 6.105896850196432",
            "extra": "mean: 22.034278829023243 sec\nrounds: 5"
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
          "distinct": false,
          "id": "faa08e9f37b9f595fc6f0f1c3fe95e0b11ba22b1",
          "message": "0.2.0",
          "timestamp": "2021-07-26T22:34:54Z",
          "tree_id": "3f8ec54834bb4525a07fbb3a268ccb16eefebe58",
          "url": "https://github.com/sfu-db/connector-x/commit/faa08e9f37b9f595fc6f0f1c3fe95e0b11ba22b1"
        },
        "date": 1627340901570,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06755466721304117,
            "unit": "iter/sec",
            "range": "stddev: 0.6057067484603056",
            "extra": "mean: 14.802826233254745 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.049972904642675675,
            "unit": "iter/sec",
            "range": "stddev: 8.256965076914295",
            "extra": "mean: 20.01084401938133 sec\nrounds: 5"
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
          "id": "90c6c4d38c9ce88ddc3d9bad8ac34fde985c505c",
          "message": "fix release",
          "timestamp": "2021-07-26T22:58:03Z",
          "tree_id": "9d0be1d6d910a9616b2d9494beab403c3a47d886",
          "url": "https://github.com/sfu-db/connector-x/commit/90c6c4d38c9ce88ddc3d9bad8ac34fde985c505c"
        },
        "date": 1627341551637,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0658847628024793,
            "unit": "iter/sec",
            "range": "stddev: 0.4065752692723045",
            "extra": "mean: 15.178016243269667 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05333989252534053,
            "unit": "iter/sec",
            "range": "stddev: 3.005572502734451",
            "extra": "mean: 18.747694317623974 sec\nrounds: 5"
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
          "id": "1f4649690b4bbda49efb0496f98ab84dd0646b8d",
          "message": "Update README.md",
          "timestamp": "2021-07-27T18:34:03-07:00",
          "tree_id": "211f2318eaee3f800f7e16b2d6d598b575f1e3c7",
          "url": "https://github.com/sfu-db/connector-x/commit/1f4649690b4bbda49efb0496f98ab84dd0646b8d"
        },
        "date": 1627436610935,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06507931879927,
            "unit": "iter/sec",
            "range": "stddev: 0.5426215497068041",
            "extra": "mean: 15.365864585712552 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.04448252857342462,
            "unit": "iter/sec",
            "range": "stddev: 2.9913298420791308",
            "extra": "mean: 22.480736416531727 sec\nrounds: 5"
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
          "id": "2a170896e8e10a18e7c053c8f96d719d417ae81f",
          "message": "Merge pull request #105 from sfu-db/mssql\n\nimplement mssql",
          "timestamp": "2021-07-27T18:58:53-07:00",
          "tree_id": "56a8d249f384329283681eb46c163d78fe05f19f",
          "url": "https://github.com/sfu-db/connector-x/commit/2a170896e8e10a18e7c053c8f96d719d417ae81f"
        },
        "date": 1627438129384,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06616344571105698,
            "unit": "iter/sec",
            "range": "stddev: 0.26260175999132707",
            "extra": "mean: 15.114085871027783 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.042198822407239496,
            "unit": "iter/sec",
            "range": "stddev: 7.418482016026507",
            "extra": "mean: 23.697343739820646 sec\nrounds: 5"
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
          "id": "9653cecd802dca479c861304231f1832eb4c17be",
          "message": "add mssql python tests",
          "timestamp": "2021-07-29T00:08:30Z",
          "tree_id": "29602a6a9ffe62b314f890125fc493b3dd3d6eb3",
          "url": "https://github.com/sfu-db/connector-x/commit/9653cecd802dca479c861304231f1832eb4c17be"
        },
        "date": 1627518003487,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.04519904194112127,
            "unit": "iter/sec",
            "range": "stddev: 1.8129727785863485",
            "extra": "mean: 22.124362753145398 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.041222231965942754,
            "unit": "iter/sec",
            "range": "stddev: 5.218364437246045",
            "extra": "mean: 24.258754373760894 sec\nrounds: 5"
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
          "id": "f28e73bbcdd11654e6249c4e6b612c52f3e81ad2",
          "message": "remove tls tests for mssql",
          "timestamp": "2021-07-29T00:11:27Z",
          "tree_id": "f2f213df441df708a045a1ae2371b6eaf8392651",
          "url": "https://github.com/sfu-db/connector-x/commit/f28e73bbcdd11654e6249c4e6b612c52f3e81ad2"
        },
        "date": 1627518561619,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06857789299418304,
            "unit": "iter/sec",
            "range": "stddev: 0.4470793278717916",
            "extra": "mean: 14.581958650797606 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05090015920367288,
            "unit": "iter/sec",
            "range": "stddev: 3.1102545208516417",
            "extra": "mean: 19.64630397320725 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "xiaoying_wang@sfu.ca",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "xiaoying_wang@sfu.ca",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "ddb69f06a754d6ce1d8197254f45aed0b423338f",
          "message": "add byte slice to pandas bytes, add test of types for mssql",
          "timestamp": "2021-07-29T01:34:55Z",
          "tree_id": "99baede6251cdd4bea42bb799ee496e45383695e",
          "url": "https://github.com/sfu-db/connector-x/commit/ddb69f06a754d6ce1d8197254f45aed0b423338f"
        },
        "date": 1627523049994,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06408320432749633,
            "unit": "iter/sec",
            "range": "stddev: 0.3313265595951601",
            "extra": "mean: 15.604712818190455 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06275610871016228,
            "unit": "iter/sec",
            "range": "stddev: 1.4842434061813536",
            "extra": "mean: 15.934703737264499 sec\nrounds: 5"
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
          "id": "0f07ed9c5bf9467041b99f0d2b29dc6e18e3bf6b",
          "message": "Create Contribute.md",
          "timestamp": "2021-07-28T23:26:48-07:00",
          "tree_id": "af3f2ec14b4f9eabf47a4612d27eca7f0c41a2de",
          "url": "https://github.com/sfu-db/connector-x/commit/0f07ed9c5bf9467041b99f0d2b29dc6e18e3bf6b"
        },
        "date": 1627540583373,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05609990442229976,
            "unit": "iter/sec",
            "range": "stddev: 0.6852250634132097",
            "extra": "mean: 17.82534231203608 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05836310894516763,
            "unit": "iter/sec",
            "range": "stddev: 0.891389725476902",
            "extra": "mean: 17.13411122323014 sec\nrounds: 5"
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
          "id": "575c89555e53bc2882184178bbfdd6c90c98c849",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-agent into main",
          "timestamp": "2021-07-29T06:27:39Z",
          "tree_id": "2e6a891371605924a4439c12f1504f0f843e2bf1",
          "url": "https://github.com/sfu-db/connector-x/commit/575c89555e53bc2882184178bbfdd6c90c98c849"
        },
        "date": 1627541155897,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06834902300642437,
            "unit": "iter/sec",
            "range": "stddev: 0.3614409748981466",
            "extra": "mean: 14.63078704001382 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.049169005935516534,
            "unit": "iter/sec",
            "range": "stddev: 6.278183155294901",
            "extra": "mean: 20.33801540164277 sec\nrounds: 5"
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
          "id": "f64971b18ec30d792c2109cc5887e27fe769eb42",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-agent into main",
          "timestamp": "2021-07-29T06:55:35Z",
          "tree_id": "ef41be6d844463c3c68931b3f39321fdfd29cabe",
          "url": "https://github.com/sfu-db/connector-x/commit/f64971b18ec30d792c2109cc5887e27fe769eb42"
        },
        "date": 1627542301156,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06657295886807896,
            "unit": "iter/sec",
            "range": "stddev: 0.20190615454870026",
            "extra": "mean: 15.021113932784647 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06145718526029602,
            "unit": "iter/sec",
            "range": "stddev: 3.189925740958951",
            "extra": "mean: 16.271490400424227 sec\nrounds: 5"
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
          "id": "5056832fb452468ecedaaf3c295ba54ef2c76a18",
          "message": "fix doctest",
          "timestamp": "2021-07-29T18:42:20Z",
          "tree_id": "558cd47958161b5c1c286826af7128403d4fe281",
          "url": "https://github.com/sfu-db/connector-x/commit/5056832fb452468ecedaaf3c295ba54ef2c76a18"
        },
        "date": 1627584818890,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.044789020150248327,
            "unit": "iter/sec",
            "range": "stddev: 1.188743749508109",
            "extra": "mean: 22.326900580665097 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.04332372986524945,
            "unit": "iter/sec",
            "range": "stddev: 3.8064980018008305",
            "extra": "mean: 23.08203848353587 sec\nrounds: 5"
          }
        ]
      }
    ]
  }
}