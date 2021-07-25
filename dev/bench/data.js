window.BENCHMARK_DATA = {
  "lastUpdate": 1627206826355,
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
      }
    ]
  }
}