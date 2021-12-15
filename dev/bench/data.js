window.BENCHMARK_DATA = {
  "lastUpdate": 1639533007914,
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
          "id": "8d2f05e955d270b7804a0fbb19520505b87b2d63",
          "message": "fix docstring",
          "timestamp": "2021-07-29T19:10:50Z",
          "tree_id": "193c8018e3d7d6de2a8cb52119336d40c62e51f5",
          "url": "https://github.com/sfu-db/connector-x/commit/8d2f05e955d270b7804a0fbb19520505b87b2d63"
        },
        "date": 1627586405414,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06571159611759547,
            "unit": "iter/sec",
            "range": "stddev: 0.16497381616254758",
            "extra": "mean: 15.21801415705122 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05785183621292887,
            "unit": "iter/sec",
            "range": "stddev: 4.331193944026608",
            "extra": "mean: 17.285536042787136 sec\nrounds: 5"
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
          "id": "83cb903f2f0c9ac727e3fe553d223c7fcd267681",
          "message": "fix docstring",
          "timestamp": "2021-07-29T19:12:03Z",
          "tree_id": "94503a36e1c3571d018e2307ae363da413f8f135",
          "url": "https://github.com/sfu-db/connector-x/commit/83cb903f2f0c9ac727e3fe553d223c7fcd267681"
        },
        "date": 1627586965922,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06703370577096418,
            "unit": "iter/sec",
            "range": "stddev: 0.19490631962964258",
            "extra": "mean: 14.917868384253234 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.056356671405025635,
            "unit": "iter/sec",
            "range": "stddev: 3.6964375235273423",
            "extra": "mean: 17.744128158548847 sec\nrounds: 5"
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
          "id": "5b4f4b17e0a543d277d2af24d6ab6a30941582e7",
          "message": "update docs for main branch push",
          "timestamp": "2021-07-29T19:18:44Z",
          "tree_id": "c32974aa134798f6be7824bd43a0f2bdb537b94a",
          "url": "https://github.com/sfu-db/connector-x/commit/5b4f4b17e0a543d277d2af24d6ab6a30941582e7"
        },
        "date": 1627587517200,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06867047855445016,
            "unit": "iter/sec",
            "range": "stddev: 0.26532911216513316",
            "extra": "mean: 14.562298400280998 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05502443919073956,
            "unit": "iter/sec",
            "range": "stddev: 2.928020574410823",
            "extra": "mean: 18.173742698831482 sec\nrounds: 5"
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
          "id": "ee42f97265fed1d7f32f94d61c1442110ddb5f97",
          "message": "doctest the example",
          "timestamp": "2021-07-29T19:56:57Z",
          "tree_id": "fc07ba71b939ff2d93192e0c8029cb6e23c54a26",
          "url": "https://github.com/sfu-db/connector-x/commit/ee42f97265fed1d7f32f94d61c1442110ddb5f97"
        },
        "date": 1627589194164,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06742161128065813,
            "unit": "iter/sec",
            "range": "stddev: 0.2575050728600105",
            "extra": "mean: 14.832039475254714 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0519015274538519,
            "unit": "iter/sec",
            "range": "stddev: 6.377237782663035",
            "extra": "mean: 19.26725568701513 sec\nrounds: 5"
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
          "id": "4fce8fb74653155d0ca94b6e9481c1d9ae34c45c",
          "message": "fix doc",
          "timestamp": "2021-07-29T20:21:12Z",
          "tree_id": "c2012714af72c27f8eef8e796900cb546b879bdc",
          "url": "https://github.com/sfu-db/connector-x/commit/4fce8fb74653155d0ca94b6e9481c1d9ae34c45c"
        },
        "date": 1627590635429,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07013724614006808,
            "unit": "iter/sec",
            "range": "stddev: 0.47643616053861143",
            "extra": "mean: 14.257759678829462 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05193033235244128,
            "unit": "iter/sec",
            "range": "stddev: 4.4443630350939936",
            "extra": "mean: 19.25656845816411 sec\nrounds: 5"
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
          "id": "6855a7c75849694a6a7a83b0d7acd3fe0c52e87d",
          "message": "do not parse url twice in postgres",
          "timestamp": "2021-08-02T22:10:41Z",
          "tree_id": "be44bd00b7c5424cfa379dbebdd9199412f33cff",
          "url": "https://github.com/sfu-db/connector-x/commit/6855a7c75849694a6a7a83b0d7acd3fe0c52e87d"
        },
        "date": 1627942912168,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.04145431459930852,
            "unit": "iter/sec",
            "range": "stddev: 0.2797939072244671",
            "extra": "mean: 24.122941355220973 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.04206577098958936,
            "unit": "iter/sec",
            "range": "stddev: 0.5609326601000034",
            "extra": "mean: 23.772296964377166 sec\nrounds: 5"
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
          "id": "b5eee718e3414d4781ca0e485efe8a993df223c4",
          "message": "fix rust test",
          "timestamp": "2021-08-03T00:25:20Z",
          "tree_id": "63892cce06a88556f4b78779ff20a510cfedf5df",
          "url": "https://github.com/sfu-db/connector-x/commit/b5eee718e3414d4781ca0e485efe8a993df223c4"
        },
        "date": 1627950869196,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06597910043684278,
            "unit": "iter/sec",
            "range": "stddev: 0.20354962119734257",
            "extra": "mean: 15.156314550805837 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06051717807899604,
            "unit": "iter/sec",
            "range": "stddev: 1.843907276738765",
            "extra": "mean: 16.524233808368443 sec\nrounds: 5"
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
          "id": "e2821ddc1162fb288dc2d4707bcd0a0e9b2f76d2",
          "message": "bugfix: mysql null value",
          "timestamp": "2021-08-16T18:25:38Z",
          "tree_id": "a7c799d95abf3131fb28fbaa765eda425dd8b972",
          "url": "https://github.com/sfu-db/connector-x/commit/e2821ddc1162fb288dc2d4707bcd0a0e9b2f76d2"
        },
        "date": 1629140300397,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.04028925166932923,
            "unit": "iter/sec",
            "range": "stddev: 0.7389784891203668",
            "extra": "mean: 24.820515610650183 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.04046958022498614,
            "unit": "iter/sec",
            "range": "stddev: 1.877179999410771",
            "extra": "mean: 24.70991778122261 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "62522644+Yizhou150@users.noreply.github.com",
            "name": "Yizhou",
            "username": "Yizhou150"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c52c80e0a1b0a182abdeb348c5e83aedd7bf1b0e",
          "message": "Update Types.md",
          "timestamp": "2021-08-17T19:03:04+08:00",
          "tree_id": "a9c6f821082f6a6832ea9f5ce1bdd910ccee0965",
          "url": "https://github.com/sfu-db/connector-x/commit/c52c80e0a1b0a182abdeb348c5e83aedd7bf1b0e"
        },
        "date": 1629198760768,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.051944518307812666,
            "unit": "iter/sec",
            "range": "stddev: 0.5039373156840101",
            "extra": "mean: 19.251309523638337 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05340527814252671,
            "unit": "iter/sec",
            "range": "stddev: 2.7548193038131616",
            "extra": "mean: 18.72474097656086 sec\nrounds: 5"
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
          "id": "f0266f95fd5d4eeb526ccc092c10ab88ae876df1",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-agent into main",
          "timestamp": "2021-08-17T12:10:23Z",
          "tree_id": "2a796c0d900abc1e7dfb8dcc09283a4dda88c5eb",
          "url": "https://github.com/sfu-db/connector-x/commit/f0266f95fd5d4eeb526ccc092c10ab88ae876df1"
        },
        "date": 1629202810999,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06411891359113789,
            "unit": "iter/sec",
            "range": "stddev: 0.1586936988394917",
            "extra": "mean: 15.59602220300585 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.04156046719450485,
            "unit": "iter/sec",
            "range": "stddev: 6.095501670188671",
            "extra": "mean: 24.061327205970883 sec\nrounds: 5"
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
          "id": "21b6fa24ed399803f7c5966b726001d336a77418",
          "message": "resolve conflict",
          "timestamp": "2021-08-24T23:44:07Z",
          "tree_id": "193833605e9207f12ea02932b297b5c3ab89e77c",
          "url": "https://github.com/sfu-db/connector-x/commit/21b6fa24ed399803f7c5966b726001d336a77418"
        },
        "date": 1629850580706,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0489235157338859,
            "unit": "iter/sec",
            "range": "stddev: 1.2854346287825535",
            "extra": "mean: 20.44006823711097 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.005095641968631747,
            "unit": "iter/sec",
            "range": "stddev: 40.1125605580005",
            "extra": "mean: 196.24612681893632 sec\nrounds: 5"
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
          "id": "3671c81b8ec7c0cabe22b656751336b27333380a",
          "message": "update",
          "timestamp": "2021-08-25T05:00:52Z",
          "tree_id": "9df15866839595ee966a218fd6474f316474186c",
          "url": "https://github.com/sfu-db/connector-x/commit/3671c81b8ec7c0cabe22b656751336b27333380a"
        },
        "date": 1629869742048,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05015111056744283,
            "unit": "iter/sec",
            "range": "stddev: 3.3813740523772298",
            "extra": "mean: 19.939737897831947 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.004608538446288348,
            "unit": "iter/sec",
            "range": "stddev: 19.674070453941123",
            "extra": "mean: 216.98853370863944 sec\nrounds: 5"
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
          "id": "8c37965b50190fa1addd52e8bdb71669b83f920b",
          "message": "update postgres url sf=1 test",
          "timestamp": "2021-08-25T05:53:42Z",
          "tree_id": "f20b7476f218bd24dfdb5a00d8f387fc04b0b371",
          "url": "https://github.com/sfu-db/connector-x/commit/8c37965b50190fa1addd52e8bdb71669b83f920b"
        },
        "date": 1629871393559,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07361557132826771,
            "unit": "iter/sec",
            "range": "stddev: 0.592698391401377",
            "extra": "mean: 13.584082578681409 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.047229081096094264,
            "unit": "iter/sec",
            "range": "stddev: 3.1847995663469026",
            "extra": "mean: 21.17339522158727 sec\nrounds: 5"
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
          "id": "3102b3b1931f454bcd50d9015a418427f75c0fd0",
          "message": "Merge pull request #116 from jorgecarleitao/arrow2\n\nAdded support for arrow2",
          "timestamp": "2021-09-08T21:51:36-07:00",
          "tree_id": "8fd7d7e83e3c2fa7a9531114623046ddecc39ac5",
          "url": "https://github.com/sfu-db/connector-x/commit/3102b3b1931f454bcd50d9015a418427f75c0fd0"
        },
        "date": 1631163666858,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06801328870284223,
            "unit": "iter/sec",
            "range": "stddev: 1.3135929479124364",
            "extra": "mean: 14.703009060025215 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.04298495575102168,
            "unit": "iter/sec",
            "range": "stddev: 2.4062828004694548",
            "extra": "mean: 23.263953225687146 sec\nrounds: 5"
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
          "id": "32f689cbb4e747745dc78a92faa4478952bf1d09",
          "message": "Merge pull request #117 from sfu-db/array\n\nSupport Array Type",
          "timestamp": "2021-09-10T16:28:25-07:00",
          "tree_id": "eef1f5c736ea9d717a68baf7e167e829f5410e08",
          "url": "https://github.com/sfu-db/connector-x/commit/32f689cbb4e747745dc78a92faa4478952bf1d09"
        },
        "date": 1631317149543,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05409332906065953,
            "unit": "iter/sec",
            "range": "stddev: 4.325851297094783",
            "extra": "mean: 18.486567888595165 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.03828105207005658,
            "unit": "iter/sec",
            "range": "stddev: 2.3596172712687213",
            "extra": "mean: 26.12258404418826 sec\nrounds: 5"
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
          "id": "ff45772f903bd5ce7727adc9ddd929812e13d10b",
          "message": "update cargo lock file",
          "timestamp": "2021-09-27T17:13:01Z",
          "tree_id": "745461affa43153b58da61b15bbb7b3807fc83a6",
          "url": "https://github.com/sfu-db/connector-x/commit/ff45772f903bd5ce7727adc9ddd929812e13d10b"
        },
        "date": 1632763203045,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0992627524407122,
            "unit": "iter/sec",
            "range": "stddev: 0.19485219275913684",
            "extra": "mean: 10.074272326845676 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07609023396995838,
            "unit": "iter/sec",
            "range": "stddev: 1.9483935782117585",
            "extra": "mean: 13.142291038227267 sec\nrounds: 5"
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
          "distinct": false,
          "id": "ff45772f903bd5ce7727adc9ddd929812e13d10b",
          "message": "update cargo lock file",
          "timestamp": "2021-09-27T17:13:01Z",
          "tree_id": "745461affa43153b58da61b15bbb7b3807fc83a6",
          "url": "https://github.com/sfu-db/connector-x/commit/ff45772f903bd5ce7727adc9ddd929812e13d10b"
        },
        "date": 1632764960320,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.10317434506076657,
            "unit": "iter/sec",
            "range": "stddev: 0.24172957491611186",
            "extra": "mean: 9.692331939795986 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0887316602047727,
            "unit": "iter/sec",
            "range": "stddev: 1.8192007664664425",
            "extra": "mean: 11.26993451595772 sec\nrounds: 5"
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
          "id": "44828ac353a812c10eebb0ea8e8e87efd2ffa864",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-agent into main",
          "timestamp": "2021-09-27T21:26:11Z",
          "tree_id": "739871e6546b6517262c98549aafffaae4f57b3d",
          "url": "https://github.com/sfu-db/connector-x/commit/44828ac353a812c10eebb0ea8e8e87efd2ffa864"
        },
        "date": 1632778406369,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.08803174796325208,
            "unit": "iter/sec",
            "range": "stddev: 0.5977013988664283",
            "extra": "mean: 11.359538156818598 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07982897346347755,
            "unit": "iter/sec",
            "range": "stddev: 3.197907416565188",
            "extra": "mean: 12.526780147780665 sec\nrounds: 5"
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
          "id": "8888684ae2f854b9d9285221882cb9a481f6ef72",
          "message": "update version info",
          "timestamp": "2021-09-27T21:52:45Z",
          "tree_id": "27c2ab714ffcef7f022e27f948ab232940af7ca5",
          "url": "https://github.com/sfu-db/connector-x/commit/8888684ae2f854b9d9285221882cb9a481f6ef72"
        },
        "date": 1632779995952,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.09857357843924608,
            "unit": "iter/sec",
            "range": "stddev: 0.29713166256125484",
            "extra": "mean: 10.144706277619116 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09292982646594229,
            "unit": "iter/sec",
            "range": "stddev: 1.7141724392011282",
            "extra": "mean: 10.760807783994824 sec\nrounds: 5"
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
          "distinct": false,
          "id": "8888684ae2f854b9d9285221882cb9a481f6ef72",
          "message": "update version info",
          "timestamp": "2021-09-27T21:52:45Z",
          "tree_id": "27c2ab714ffcef7f022e27f948ab232940af7ca5",
          "url": "https://github.com/sfu-db/connector-x/commit/8888684ae2f854b9d9285221882cb9a481f6ef72"
        },
        "date": 1632780452324,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.10300323141656036,
            "unit": "iter/sec",
            "range": "stddev: 0.10918953540835265",
            "extra": "mean: 9.708433281630278 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06993379682047338,
            "unit": "iter/sec",
            "range": "stddev: 2.364803475958093",
            "extra": "mean: 14.299237928795629 sec\nrounds: 5"
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
          "id": "85d4e3c31036baf61d61aa65d0bf47e826383e0d",
          "message": "upgrade dependencies, fix arrow2 support",
          "timestamp": "2021-09-28T17:34:36Z",
          "tree_id": "d1ade9288926445a9c4dbf62d47dbdf2fe9617b9",
          "url": "https://github.com/sfu-db/connector-x/commit/85d4e3c31036baf61d61aa65d0bf47e826383e0d"
        },
        "date": 1632850970652,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07026416156284308,
            "unit": "iter/sec",
            "range": "stddev: 0.27864172348572136",
            "extra": "mean: 14.232006441941484 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06592630781285891,
            "unit": "iter/sec",
            "range": "stddev: 1.7790524636920737",
            "extra": "mean: 15.168451460055076 sec\nrounds: 5"
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
          "id": "39dbdfba5610736c47d2860e5832aabadaed8740",
          "message": "Merge branch 'oracle_types' into main",
          "timestamp": "2021-10-01T06:53:33Z",
          "tree_id": "90c245471a0f94816daa782946628b71d8f33e84",
          "url": "https://github.com/sfu-db/connector-x/commit/39dbdfba5610736c47d2860e5832aabadaed8740"
        },
        "date": 1633071728709,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.059053047596850414,
            "unit": "iter/sec",
            "range": "stddev: 0.5331586939120976",
            "extra": "mean: 16.933927048556505 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06897010580067228,
            "unit": "iter/sec",
            "range": "stddev: 3.309147797833932",
            "extra": "mean: 14.499035319592803 sec\nrounds: 5"
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
          "id": "2703df418e0ca7e2358b6cdae463f2eb23b6bac7",
          "message": "update justfile",
          "timestamp": "2021-10-01T06:55:27Z",
          "tree_id": "4296c9ed98d5bd34bd329558e238c619b7ef32d6",
          "url": "https://github.com/sfu-db/connector-x/commit/2703df418e0ca7e2358b6cdae463f2eb23b6bac7"
        },
        "date": 1633072213341,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06873730118021677,
            "unit": "iter/sec",
            "range": "stddev: 0.38713143130940475",
            "extra": "mean: 14.548141734255477 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06782882313807123,
            "unit": "iter/sec",
            "range": "stddev: 2.1953930909219026",
            "extra": "mean: 14.742994994390756 sec\nrounds: 5"
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
          "id": "ff5a7f4b07af8fcb6b2999ba83c0c598045891e7",
          "message": "update test",
          "timestamp": "2021-10-01T18:09:58Z",
          "tree_id": "a2e48478fffdf09911cab0ddc9ddb903ca68fe1e",
          "url": "https://github.com/sfu-db/connector-x/commit/ff5a7f4b07af8fcb6b2999ba83c0c598045891e7"
        },
        "date": 1633112328995,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05646470617357435,
            "unit": "iter/sec",
            "range": "stddev: 0.15732360980290871",
            "extra": "mean: 17.710178052214907 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05927863590209872,
            "unit": "iter/sec",
            "range": "stddev: 1.346415749599379",
            "extra": "mean: 16.86948400181718 sec\nrounds: 5"
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
          "id": "acd8ce0e2a3ba7fa70798a31b60de793d03d6ae6",
          "message": "update alpha version",
          "timestamp": "2021-10-01T22:23:52Z",
          "tree_id": "9d3743151728564c45843cca3694b064bc76b203",
          "url": "https://github.com/sfu-db/connector-x/commit/acd8ce0e2a3ba7fa70798a31b60de793d03d6ae6"
        },
        "date": 1633127549312,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0654713582827892,
            "unit": "iter/sec",
            "range": "stddev: 0.46064924976053134",
            "extra": "mean: 15.273854494979606 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05586209200180664,
            "unit": "iter/sec",
            "range": "stddev: 2.948442441342616",
            "extra": "mean: 17.90122718582861 sec\nrounds: 5"
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
          "distinct": false,
          "id": "acd8ce0e2a3ba7fa70798a31b60de793d03d6ae6",
          "message": "update alpha version",
          "timestamp": "2021-10-01T22:23:52Z",
          "tree_id": "9d3743151728564c45843cca3694b064bc76b203",
          "url": "https://github.com/sfu-db/connector-x/commit/acd8ce0e2a3ba7fa70798a31b60de793d03d6ae6"
        },
        "date": 1633128027142,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06978929906034766,
            "unit": "iter/sec",
            "range": "stddev: 0.17510192507050837",
            "extra": "mean: 14.328844299400226 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06893277377740971,
            "unit": "iter/sec",
            "range": "stddev: 2.432989183655055",
            "extra": "mean: 14.506887583388016 sec\nrounds: 5"
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
          "id": "5e1791809ac119691b5d5d2636117d3d5e98f63a",
          "message": "update justfile flame-tpch",
          "timestamp": "2021-10-03T04:51:17Z",
          "tree_id": "3c6fe360788fbd8cfc74ff859b6e038ef745e9c1",
          "url": "https://github.com/sfu-db/connector-x/commit/5e1791809ac119691b5d5d2636117d3d5e98f63a"
        },
        "date": 1633237201517,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06327880778030245,
            "unit": "iter/sec",
            "range": "stddev: 1.1921118835170368",
            "extra": "mean: 15.803079025633632 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05609658842472649,
            "unit": "iter/sec",
            "range": "stddev: 4.120546981732086",
            "extra": "mean: 17.826396008767187 sec\nrounds: 5"
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
          "id": "4aa320d811d29c4035ca690f19f69b0d51903a99",
          "message": "Merge pull request #128 from wseaton/upgrade-r2d2postgres\n\nupgrade postgres connection pooling lib",
          "timestamp": "2021-10-05T16:00:24-07:00",
          "tree_id": "1bb4169fd525ccf75c7f3a0eb4398d9eb854ed59",
          "url": "https://github.com/sfu-db/connector-x/commit/4aa320d811d29c4035ca690f19f69b0d51903a99"
        },
        "date": 1633475302902,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06926545753133131,
            "unit": "iter/sec",
            "range": "stddev: 0.32009780395767073",
            "extra": "mean: 14.437210633419454 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06846796168677202,
            "unit": "iter/sec",
            "range": "stddev: 2.845097951452993",
            "extra": "mean: 14.605371262179688 sec\nrounds: 5"
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
          "id": "a721637bb746d64552264ef49fd7f3fbdd1b6d88",
          "message": "Merge pull request #122 from sfu-db/test_db\n\nimproved test cases for mysql postgres sqlite mssql",
          "timestamp": "2021-10-05T17:38:12-07:00",
          "tree_id": "72699e0e9593cba13e6ec5123a223295a595c86a",
          "url": "https://github.com/sfu-db/connector-x/commit/a721637bb746d64552264ef49fd7f3fbdd1b6d88"
        },
        "date": 1633481394697,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06577908600347585,
            "unit": "iter/sec",
            "range": "stddev: 0.1958786780926353",
            "extra": "mean: 15.2024003487546 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06147412164371622,
            "unit": "iter/sec",
            "range": "stddev: 1.5228916879219248",
            "extra": "mean: 16.26700753523037 sec\nrounds: 5"
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
          "distinct": false,
          "id": "a721637bb746d64552264ef49fd7f3fbdd1b6d88",
          "message": "Merge pull request #122 from sfu-db/test_db\n\nimproved test cases for mysql postgres sqlite mssql",
          "timestamp": "2021-10-05T17:38:12-07:00",
          "tree_id": "72699e0e9593cba13e6ec5123a223295a595c86a",
          "url": "https://github.com/sfu-db/connector-x/commit/a721637bb746d64552264ef49fd7f3fbdd1b6d88"
        },
        "date": 1633491047417,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06054739431225692,
            "unit": "iter/sec",
            "range": "stddev: 0.8579892080629115",
            "extra": "mean: 16.51598737416789 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0683006842258494,
            "unit": "iter/sec",
            "range": "stddev: 3.3483551606783757",
            "extra": "mean: 14.641141759185121 sec\nrounds: 5"
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
          "id": "be8d31722b2cd4e7a1c1fc19c7343c60d31ec8b4",
          "message": "release 0.2.1",
          "timestamp": "2021-10-06T03:27:50Z",
          "tree_id": "a1e0370b1cd5bf70f238f78f7dd52f7e0c43f6b3",
          "url": "https://github.com/sfu-db/connector-x/commit/be8d31722b2cd4e7a1c1fc19c7343c60d31ec8b4"
        },
        "date": 1633491548364,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0694081995991887,
            "unit": "iter/sec",
            "range": "stddev: 0.4525164502467451",
            "extra": "mean: 14.407519655814394 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.054978156411531545,
            "unit": "iter/sec",
            "range": "stddev: 4.08303095948702",
            "extra": "mean: 18.189042071811855 sec\nrounds: 5"
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
          "distinct": false,
          "id": "be8d31722b2cd4e7a1c1fc19c7343c60d31ec8b4",
          "message": "release 0.2.1",
          "timestamp": "2021-10-06T03:27:50Z",
          "tree_id": "a1e0370b1cd5bf70f238f78f7dd52f7e0c43f6b3",
          "url": "https://github.com/sfu-db/connector-x/commit/be8d31722b2cd4e7a1c1fc19c7343c60d31ec8b4"
        },
        "date": 1633492049518,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06763738233350139,
            "unit": "iter/sec",
            "range": "stddev: 0.32173790578898753",
            "extra": "mean: 14.78472355818376 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05873335735011689,
            "unit": "iter/sec",
            "range": "stddev: 3.0670080311557313",
            "extra": "mean: 17.02609973475337 sec\nrounds: 5"
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
          "id": "a12fde6837d1227038244abe7d4eff4453bc311d",
          "message": "[skil ci] update types",
          "timestamp": "2021-10-05T22:19:13-07:00",
          "tree_id": "b11409600301d5a1cb071d86dd74d9e5d5d7b13f",
          "url": "https://github.com/sfu-db/connector-x/commit/a12fde6837d1227038244abe7d4eff4453bc311d"
        },
        "date": 1633498060113,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0670295272102052,
            "unit": "iter/sec",
            "range": "stddev: 0.3486477647654023",
            "extra": "mean: 14.91879835082218 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05711498091196524,
            "unit": "iter/sec",
            "range": "stddev: 3.5017464823419484",
            "extra": "mean: 17.50854126242921 sec\nrounds: 5"
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
          "id": "c1cd31f3966d6b15199af23ce8a9ee3f39d60a52",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-agent into main",
          "timestamp": "2021-10-06T20:35:42Z",
          "tree_id": "a23f3258b81fb007e278e9cbd77c561eadd2a1d1",
          "url": "https://github.com/sfu-db/connector-x/commit/c1cd31f3966d6b15199af23ce8a9ee3f39d60a52"
        },
        "date": 1633553064661,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05921829646897175,
            "unit": "iter/sec",
            "range": "stddev: 1.2678903501639829",
            "extra": "mean: 16.886672863410787 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06852030208831181,
            "unit": "iter/sec",
            "range": "stddev: 1.0466852195370377",
            "extra": "mean: 14.594214700208976 sec\nrounds: 5"
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
          "distinct": false,
          "id": "c1cd31f3966d6b15199af23ce8a9ee3f39d60a52",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-agent into main",
          "timestamp": "2021-10-06T20:35:42Z",
          "tree_id": "a23f3258b81fb007e278e9cbd77c561eadd2a1d1",
          "url": "https://github.com/sfu-db/connector-x/commit/c1cd31f3966d6b15199af23ce8a9ee3f39d60a52"
        },
        "date": 1633554336421,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06681761311648844,
            "unit": "iter/sec",
            "range": "stddev: 0.35564691663696496",
            "extra": "mean: 14.966113773873076 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0641090735954478,
            "unit": "iter/sec",
            "range": "stddev: 5.742697502677566",
            "extra": "mean: 15.598416010662913 sec\nrounds: 5"
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
          "id": "833ed49325038255235fb39cecc6443dd288bfef",
          "message": "fix memory issue",
          "timestamp": "2021-10-12T08:32:57Z",
          "tree_id": "90b1e43e0ad312edebdcef0cf076ec1b8d7eebb6",
          "url": "https://github.com/sfu-db/connector-x/commit/833ed49325038255235fb39cecc6443dd288bfef"
        },
        "date": 1634028214319,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06948572040374436,
            "unit": "iter/sec",
            "range": "stddev: 0.35135951094315543",
            "extra": "mean: 14.391446101292967 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06592805884200557,
            "unit": "iter/sec",
            "range": "stddev: 5.171239290359361",
            "extra": "mean: 15.168048590607942 sec\nrounds: 5"
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
          "id": "833ed49325038255235fb39cecc6443dd288bfef",
          "message": "fix memory issue",
          "timestamp": "2021-10-12T08:32:57Z",
          "tree_id": "90b1e43e0ad312edebdcef0cf076ec1b8d7eebb6",
          "url": "https://github.com/sfu-db/connector-x/commit/833ed49325038255235fb39cecc6443dd288bfef"
        },
        "date": 1634028703496,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07192462318955262,
            "unit": "iter/sec",
            "range": "stddev: 0.602099513979325",
            "extra": "mean: 13.90344440685585 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05544317164075612,
            "unit": "iter/sec",
            "range": "stddev: 3.166871909152329",
            "extra": "mean: 18.03648619670421 sec\nrounds: 5"
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
          "id": "a3036547fffbf1a186a6d809ac6c75851f738a4e",
          "message": "alpha.5",
          "timestamp": "2021-10-12T08:55:16Z",
          "tree_id": "6fee00d4e7ca342da52dc46cfda6714cb435fb5c",
          "url": "https://github.com/sfu-db/connector-x/commit/a3036547fffbf1a186a6d809ac6c75851f738a4e"
        },
        "date": 1634029418779,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06882758766999339,
            "unit": "iter/sec",
            "range": "stddev: 0.3519066309256407",
            "extra": "mean: 14.529057807382197 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05365533756425374,
            "unit": "iter/sec",
            "range": "stddev: 0.7157004106724745",
            "extra": "mean: 18.637474767584354 sec\nrounds: 5"
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
          "id": "cb3c35031cccab29e0920f152169baae87c33ecb",
          "message": "fix",
          "timestamp": "2021-10-12T09:15:44Z",
          "tree_id": "a0d3b728b0010bcbcd706ff9d4ee8b33623ccbb8",
          "url": "https://github.com/sfu-db/connector-x/commit/cb3c35031cccab29e0920f152169baae87c33ecb"
        },
        "date": 1634030613536,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07038705879250375,
            "unit": "iter/sec",
            "range": "stddev: 0.16224083337556422",
            "extra": "mean: 14.207157070562243 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0773779671440878,
            "unit": "iter/sec",
            "range": "stddev: 1.6160395043401106",
            "extra": "mean: 12.923575494531542 sec\nrounds: 5"
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
          "id": "f5dd4deebc5c7bf2e20dcd6c3d8ce3d8851b6fcf",
          "message": "Merge pull request #142 from sfu-db/mssql_url\n\nadd urldecode",
          "timestamp": "2021-10-21T22:14:16-07:00",
          "tree_id": "a51431a52715d78e1703716feb1a6c50a54192e9",
          "url": "https://github.com/sfu-db/connector-x/commit/f5dd4deebc5c7bf2e20dcd6c3d8ce3d8851b6fcf"
        },
        "date": 1634883310738,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0692512144611283,
            "unit": "iter/sec",
            "range": "stddev: 0.5360051199461757",
            "extra": "mean: 14.440179970581084 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05293983542215201,
            "unit": "iter/sec",
            "range": "stddev: 2.884829543051788",
            "extra": "mean: 18.889367373846472 sec\nrounds: 5"
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
          "id": "9eabc13a4f72de64d6db1e84cd85c1284a043cc9",
          "message": "update alpha version to 0.2.1-alpha.6",
          "timestamp": "2021-10-22T05:15:53Z",
          "tree_id": "8a85a77990efad3adcd4bc77aee1c5446871f75a",
          "url": "https://github.com/sfu-db/connector-x/commit/9eabc13a4f72de64d6db1e84cd85c1284a043cc9"
        },
        "date": 1634883786154,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06780534517399113,
            "unit": "iter/sec",
            "range": "stddev: 0.5118360196103947",
            "extra": "mean: 14.748099835403263 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06317606374796127,
            "unit": "iter/sec",
            "range": "stddev: 3.907731547355093",
            "extra": "mean: 15.828779773134737 sec\nrounds: 5"
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
          "distinct": false,
          "id": "9eabc13a4f72de64d6db1e84cd85c1284a043cc9",
          "message": "update alpha version to 0.2.1-alpha.6",
          "timestamp": "2021-10-22T05:15:53Z",
          "tree_id": "8a85a77990efad3adcd4bc77aee1c5446871f75a",
          "url": "https://github.com/sfu-db/connector-x/commit/9eabc13a4f72de64d6db1e84cd85c1284a043cc9"
        },
        "date": 1634885061150,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07015153355388118,
            "unit": "iter/sec",
            "range": "stddev: 0.32603593868238645",
            "extra": "mean: 14.254855871852488 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06411439826260858,
            "unit": "iter/sec",
            "range": "stddev: 4.600747967385506",
            "extra": "mean: 15.597120570391416 sec\nrounds: 5"
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
          "id": "146629aad000c9a7fa06bdae324e693e345039e4",
          "message": "Merge pull request #138 from sfu-db/read_sql_params\n\nadd index_col for read_sql",
          "timestamp": "2021-10-25T10:37:25-07:00",
          "tree_id": "4f50ab3d874d3cfcaf7313f670d5b804875cd03f",
          "url": "https://github.com/sfu-db/connector-x/commit/146629aad000c9a7fa06bdae324e693e345039e4"
        },
        "date": 1635183929276,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07024337993473233,
            "unit": "iter/sec",
            "range": "stddev: 0.42841920611236156",
            "extra": "mean: 14.236217006202788 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07182692469931379,
            "unit": "iter/sec",
            "range": "stddev: 3.749123069444581",
            "extra": "mean: 13.922355776559561 sec\nrounds: 5"
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
          "id": "9b0409ec26a7639c9254e58233533ca5a8f2784c",
          "message": "add cache",
          "timestamp": "2021-10-26T03:28:20Z",
          "tree_id": "79832c90075fbc7c1aea34d99635510a79ce233d",
          "url": "https://github.com/sfu-db/connector-x/commit/9b0409ec26a7639c9254e58233533ca5a8f2784c"
        },
        "date": 1635219423038,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06956671306962978,
            "unit": "iter/sec",
            "range": "stddev: 0.11840686043455438",
            "extra": "mean: 14.374690938740969 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.047543393760066374,
            "unit": "iter/sec",
            "range": "stddev: 4.057356767926164",
            "extra": "mean: 21.033416441548617 sec\nrounds: 5"
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
          "id": "0124ff9c0165b4c1e13baf01aa1c52bde99f8ed8",
          "message": "also cache pyhton",
          "timestamp": "2021-10-26T16:23:59Z",
          "tree_id": "a3d90873f6ef56ed48e6230aac871739985f6424",
          "url": "https://github.com/sfu-db/connector-x/commit/0124ff9c0165b4c1e13baf01aa1c52bde99f8ed8"
        },
        "date": 1635265893173,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07215947021269381,
            "unit": "iter/sec",
            "range": "stddev: 0.898952447762111",
            "extra": "mean: 13.858194871060551 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09050986635833642,
            "unit": "iter/sec",
            "range": "stddev: 1.49062579653335",
            "extra": "mean: 11.048519241437315 sec\nrounds: 5"
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
          "id": "fe5e88ad77dcb9610cc7c9a90340c5c191bb1cb1",
          "message": "fix ci",
          "timestamp": "2021-10-31T05:31:36Z",
          "tree_id": "b99f0a9f96920b2a284e7625d02f9e3201130f9c",
          "url": "https://github.com/sfu-db/connector-x/commit/fe5e88ad77dcb9610cc7c9a90340c5c191bb1cb1"
        },
        "date": 1635658764726,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07257239374211134,
            "unit": "iter/sec",
            "range": "stddev: 0.3064735979630826",
            "extra": "mean: 13.779344299342483 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06525887987535317,
            "unit": "iter/sec",
            "range": "stddev: 3.6337219299440546",
            "extra": "mean: 15.323585110716522 sec\nrounds: 5"
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
          "id": "e423be547c1f058bb79584677839ba71f4e6f900",
          "message": "fix ci",
          "timestamp": "2021-10-31T05:41:23Z",
          "tree_id": "f4fde6a7dba166c3dfb9408a99880bc16cd522be",
          "url": "https://github.com/sfu-db/connector-x/commit/e423be547c1f058bb79584677839ba71f4e6f900"
        },
        "date": 1635659352585,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07089495859327455,
            "unit": "iter/sec",
            "range": "stddev: 0.25806287708094056",
            "extra": "mean: 14.105375330522657 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07555229109443377,
            "unit": "iter/sec",
            "range": "stddev: 2.3302892753651268",
            "extra": "mean: 13.235865987837315 sec\nrounds: 5"
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
          "id": "0fecf17f4082b7c9ca00c91a87f7f195df5b72ed",
          "message": "fix ci",
          "timestamp": "2021-10-31T06:15:39Z",
          "tree_id": "2be8ae4491039079af3e87cfd137561aa3ccb921",
          "url": "https://github.com/sfu-db/connector-x/commit/0fecf17f4082b7c9ca00c91a87f7f195df5b72ed"
        },
        "date": 1635661713638,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07069718289779314,
            "unit": "iter/sec",
            "range": "stddev: 0.4111852167524321",
            "extra": "mean: 14.14483518311754 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06604438664338849,
            "unit": "iter/sec",
            "range": "stddev: 5.751843134860447",
            "extra": "mean: 15.141332228574901 sec\nrounds: 5"
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
          "id": "ffb41a8eb241fb5e81d780f4b3f17572c8910dfb",
          "message": "fix ci",
          "timestamp": "2021-10-31T06:47:28Z",
          "tree_id": "d948de56e42ed042dd1310f6ed0176f268958c9a",
          "url": "https://github.com/sfu-db/connector-x/commit/ffb41a8eb241fb5e81d780f4b3f17572c8910dfb"
        },
        "date": 1635663319464,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07399122854571917,
            "unit": "iter/sec",
            "range": "stddev: 0.47715554901530377",
            "extra": "mean: 13.515115502942354 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07222309697911482,
            "unit": "iter/sec",
            "range": "stddev: 2.8792438674298424",
            "extra": "mean: 13.845986143313349 sec\nrounds: 5"
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
          "id": "f89a782f64516ffcc139110dcc663259f3c3ef5d",
          "message": "fix ci",
          "timestamp": "2021-10-31T06:47:51Z",
          "tree_id": "75da6d6f85a466d0f5606ac963de7c32728c7d92",
          "url": "https://github.com/sfu-db/connector-x/commit/f89a782f64516ffcc139110dcc663259f3c3ef5d"
        },
        "date": 1635663782259,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07022277126326369,
            "unit": "iter/sec",
            "range": "stddev: 0.6452720932717263",
            "extra": "mean: 14.240394988842308 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07535150374416219,
            "unit": "iter/sec",
            "range": "stddev: 2.2317060982403527",
            "extra": "mean: 13.271135283447801 sec\nrounds: 5"
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
          "id": "a587309758c13e7797139b9be258e8021b8a2e72",
          "message": "remove limit1 query for postgres and mysql",
          "timestamp": "2021-11-02T17:28:15Z",
          "tree_id": "46a698205bd08b13de44c7b330377231f90d0e41",
          "url": "https://github.com/sfu-db/connector-x/commit/a587309758c13e7797139b9be258e8021b8a2e72"
        },
        "date": 1635901718488,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06851175478514603,
            "unit": "iter/sec",
            "range": "stddev: 0.4964543970622145",
            "extra": "mean: 14.596035426855087 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07170945767029119,
            "unit": "iter/sec",
            "range": "stddev: 3.0859276686577672",
            "extra": "mean: 13.945161942206322 sec\nrounds: 5"
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
          "id": "1b35ef4261b7883b50f334f03124be3d5af78110",
          "message": "remove limit1 query for postgres and mysql",
          "timestamp": "2021-11-02T17:44:43Z",
          "tree_id": "c060694fb3069274720f3185c9f501f6f2f839b6",
          "url": "https://github.com/sfu-db/connector-x/commit/1b35ef4261b7883b50f334f03124be3d5af78110"
        },
        "date": 1635902158238,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07161414533994702,
            "unit": "iter/sec",
            "range": "stddev: 0.26810491333696623",
            "extra": "mean: 13.96372176548466 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08677138283960585,
            "unit": "iter/sec",
            "range": "stddev: 2.361982670468718",
            "extra": "mean: 11.524536860827357 sec\nrounds: 5"
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
          "id": "ec0e659c2b60e2a4e4fc9ad1ca4eb83bd502cc42",
          "message": "remove limit1 query for postgres and mysql",
          "timestamp": "2021-11-02T18:08:50Z",
          "tree_id": "c060694fb3069274720f3185c9f501f6f2f839b6",
          "url": "https://github.com/sfu-db/connector-x/commit/ec0e659c2b60e2a4e4fc9ad1ca4eb83bd502cc42"
        },
        "date": 1635902615133,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0700224163110152,
            "unit": "iter/sec",
            "range": "stddev: 0.43670239773958547",
            "extra": "mean: 14.281140992883593 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07582146997802792,
            "unit": "iter/sec",
            "range": "stddev: 2.191579460733505",
            "extra": "mean: 13.1888764526695 sec\nrounds: 5"
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
          "id": "fc45f63104cbac33727eae32975a7a925a6b3597",
          "message": "remove limit1 query for postgres and mysql",
          "timestamp": "2021-11-02T18:17:16Z",
          "tree_id": "343df0eedb4d2f111991e63a1f8c4ff2d4ed4e5e",
          "url": "https://github.com/sfu-db/connector-x/commit/fc45f63104cbac33727eae32975a7a925a6b3597"
        },
        "date": 1635903075707,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07314710474631668,
            "unit": "iter/sec",
            "range": "stddev: 0.39859366983689376",
            "extra": "mean: 13.67108108336106 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06881347453467604,
            "unit": "iter/sec",
            "range": "stddev: 4.880017389318554",
            "extra": "mean: 14.532037609815598 sec\nrounds: 5"
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
          "id": "6b324d3fdb6ecef3090ad3009729910c4b239a11",
          "message": "remove limit1 query for postgres and mysql",
          "timestamp": "2021-11-02T18:28:40Z",
          "tree_id": "343df0eedb4d2f111991e63a1f8c4ff2d4ed4e5e",
          "url": "https://github.com/sfu-db/connector-x/commit/6b324d3fdb6ecef3090ad3009729910c4b239a11"
        },
        "date": 1635903516525,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0704618950599566,
            "unit": "iter/sec",
            "range": "stddev: 0.5277428048001575",
            "extra": "mean: 14.192067913431675 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08881481630248032,
            "unit": "iter/sec",
            "range": "stddev: 2.384325807216489",
            "extra": "mean: 11.259382630418987 sec\nrounds: 5"
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
          "id": "1b291dfbc21a3318b0c1de4faf9544e77c5bd368",
          "message": "remove limit1 query for postgres and mysql",
          "timestamp": "2021-11-02T18:36:04Z",
          "tree_id": "f9593a2f68a1580245967fcbb7dc27c507c5009f",
          "url": "https://github.com/sfu-db/connector-x/commit/1b291dfbc21a3318b0c1de4faf9544e77c5bd368"
        },
        "date": 1635903991114,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07077309601316038,
            "unit": "iter/sec",
            "range": "stddev: 0.3729973593401803",
            "extra": "mean: 14.12966305464506 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06575288565494652,
            "unit": "iter/sec",
            "range": "stddev: 3.3245324711602247",
            "extra": "mean: 15.208458002097904 sec\nrounds: 5"
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
          "id": "91c976db01a9767e5f2c15815c54f9cf6f4011ba",
          "message": "Merge pull request #153 from wseaton/postgres-int-fix\n\nFix for #149",
          "timestamp": "2021-11-02T16:13:56-07:00",
          "tree_id": "944a99a99e2afde72208e89f6a1c1a78a5ee1652",
          "url": "https://github.com/sfu-db/connector-x/commit/91c976db01a9767e5f2c15815c54f9cf6f4011ba"
        },
        "date": 1635905231162,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0718627286239043,
            "unit": "iter/sec",
            "range": "stddev: 0.3161890403486802",
            "extra": "mean: 13.915419288258999 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0644944659070107,
            "unit": "iter/sec",
            "range": "stddev: 4.002856830221902",
            "extra": "mean: 15.505206314008683 sec\nrounds: 5"
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
          "id": "1fba739112ac422e20e18bf544f71e1b09fd3f17",
          "message": "0.2.2-alpha.1",
          "timestamp": "2021-11-02T23:41:07Z",
          "tree_id": "00405a246e5d946f63cf718f03b6b9a477fecb9f",
          "url": "https://github.com/sfu-db/connector-x/commit/1fba739112ac422e20e18bf544f71e1b09fd3f17"
        },
        "date": 1635905659956,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07617640541277984,
            "unit": "iter/sec",
            "range": "stddev: 0.12934068372385524",
            "extra": "mean: 13.127424359042198 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09366231342792693,
            "unit": "iter/sec",
            "range": "stddev: 1.6420243816735183",
            "extra": "mean: 10.676652790233494 sec\nrounds: 5"
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
          "distinct": false,
          "id": "1fba739112ac422e20e18bf544f71e1b09fd3f17",
          "message": "0.2.2-alpha.1",
          "timestamp": "2021-11-02T23:41:07Z",
          "tree_id": "00405a246e5d946f63cf718f03b6b9a477fecb9f",
          "url": "https://github.com/sfu-db/connector-x/commit/1fba739112ac422e20e18bf544f71e1b09fd3f17"
        },
        "date": 1635906574951,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06863727762098168,
            "unit": "iter/sec",
            "range": "stddev: 0.2582225499329881",
            "extra": "mean: 14.569342413637788 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0725750203270856,
            "unit": "iter/sec",
            "range": "stddev: 3.4210545015080522",
            "extra": "mean: 13.778845606837422 sec\nrounds: 5"
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
          "id": "4813a79104c8220f37273a3423d57bb712f8f4cc",
          "message": "Merge pull request #154 from wseaton/upgrade-polars\n\nadd support for polars >= 0.8",
          "timestamp": "2021-11-03T12:23:51-07:00",
          "tree_id": "d09ceb4a3f4c743ae676220b0d268cc53ae8150e",
          "url": "https://github.com/sfu-db/connector-x/commit/4813a79104c8220f37273a3423d57bb712f8f4cc"
        },
        "date": 1635968116790,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06762590281384843,
            "unit": "iter/sec",
            "range": "stddev: 0.06561324523070804",
            "extra": "mean: 14.787233269959689 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06389584960777255,
            "unit": "iter/sec",
            "range": "stddev: 3.530868360322624",
            "extra": "mean: 15.650468788482248 sec\nrounds: 5"
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
          "id": "f5f2d252894791209dbcec944ab30a5a34d9ae3f",
          "message": "add 3.10 to release",
          "timestamp": "2021-11-04T22:51:23Z",
          "tree_id": "c979239ce88dd9e5ae0de54735d796b71ce3e8b1",
          "url": "https://github.com/sfu-db/connector-x/commit/f5f2d252894791209dbcec944ab30a5a34d9ae3f"
        },
        "date": 1636066783262,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06747736304582888,
            "unit": "iter/sec",
            "range": "stddev: 0.1782205436020495",
            "extra": "mean: 14.8197848116979 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07516198759041771,
            "unit": "iter/sec",
            "range": "stddev: 3.9026257693457196",
            "extra": "mean: 13.30459760390222 sec\nrounds: 5"
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
          "id": "55631f411563eea49ba6c8fa0112d12f3e04899b",
          "message": "add 3.10 to import-test",
          "timestamp": "2021-11-04T22:54:38Z",
          "tree_id": "151baad842fb7c53bd661a9afad2a263952be788",
          "url": "https://github.com/sfu-db/connector-x/commit/55631f411563eea49ba6c8fa0112d12f3e04899b"
        },
        "date": 1636067270864,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06737038386856611,
            "unit": "iter/sec",
            "range": "stddev: 0.32683028097031935",
            "extra": "mean: 14.843317531794309 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07584765851908502,
            "unit": "iter/sec",
            "range": "stddev: 1.9681371904300806",
            "extra": "mean: 13.18432262148708 sec\nrounds: 5"
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
          "id": "e5450f152c450e44578828dfb668d7ad7845d1db",
          "message": "0.2.2-alpha.2",
          "timestamp": "2021-11-04T23:05:28Z",
          "tree_id": "ff1527652305731a146eb53fa8755ea7c901fbda",
          "url": "https://github.com/sfu-db/connector-x/commit/e5450f152c450e44578828dfb668d7ad7845d1db"
        },
        "date": 1636067753957,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06623632215900145,
            "unit": "iter/sec",
            "range": "stddev: 0.20274052785180863",
            "extra": "mean: 15.097456612996757 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07669958994498631,
            "unit": "iter/sec",
            "range": "stddev: 3.143796861436696",
            "extra": "mean: 13.037879351340234 sec\nrounds: 5"
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
          "distinct": false,
          "id": "e5450f152c450e44578828dfb668d7ad7845d1db",
          "message": "0.2.2-alpha.2",
          "timestamp": "2021-11-04T23:05:28Z",
          "tree_id": "ff1527652305731a146eb53fa8755ea7c901fbda",
          "url": "https://github.com/sfu-db/connector-x/commit/e5450f152c450e44578828dfb668d7ad7845d1db"
        },
        "date": 1636069091193,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06688493968469295,
            "unit": "iter/sec",
            "range": "stddev: 0.20422037489974784",
            "extra": "mean: 14.951048841699958 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07098081098892782,
            "unit": "iter/sec",
            "range": "stddev: 1.4362687751923486",
            "extra": "mean: 14.088314659520984 sec\nrounds: 5"
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
          "id": "36d0c034dc378403bfc1ebb263a07739d153c140",
          "message": "solve dependency issues for python 3.10",
          "timestamp": "2021-11-05T00:59:28Z",
          "tree_id": "eea4f81922e4e80f82f82ab5c7eced00bb8ab35c",
          "url": "https://github.com/sfu-db/connector-x/commit/36d0c034dc378403bfc1ebb263a07739d153c140"
        },
        "date": 1636074461362,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0706825732653436,
            "unit": "iter/sec",
            "range": "stddev: 0.31369804026931075",
            "extra": "mean: 14.147758829407394 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07419098346451294,
            "unit": "iter/sec",
            "range": "stddev: 2.5499639538622967",
            "extra": "mean: 13.47872683852911 sec\nrounds: 5"
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
          "distinct": false,
          "id": "36d0c034dc378403bfc1ebb263a07739d153c140",
          "message": "solve dependency issues for python 3.10",
          "timestamp": "2021-11-05T00:59:28Z",
          "tree_id": "eea4f81922e4e80f82f82ab5c7eced00bb8ab35c",
          "url": "https://github.com/sfu-db/connector-x/commit/36d0c034dc378403bfc1ebb263a07739d153c140"
        },
        "date": 1636075026218,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0689500685148855,
            "unit": "iter/sec",
            "range": "stddev: 0.31719565993175175",
            "extra": "mean: 14.503248822502792 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07776625434887009,
            "unit": "iter/sec",
            "range": "stddev: 4.332944736232294",
            "extra": "mean: 12.85904803276062 sec\nrounds: 5"
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
          "id": "77438d0ab7f16c8f31e54942bf0c299c9c8a1f4d",
          "message": "update release",
          "timestamp": "2021-11-05T02:10:30Z",
          "tree_id": "14b0770328b5d9cff7c242a61a978e4180394428",
          "url": "https://github.com/sfu-db/connector-x/commit/77438d0ab7f16c8f31e54942bf0c299c9c8a1f4d"
        },
        "date": 1636078724614,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06617148355885637,
            "unit": "iter/sec",
            "range": "stddev: 0.21205794752880006",
            "extra": "mean: 15.11224996354431 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.061825637873067135,
            "unit": "iter/sec",
            "range": "stddev: 2.991797864816048",
            "extra": "mean: 16.174519736506046 sec\nrounds: 5"
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
          "distinct": false,
          "id": "77438d0ab7f16c8f31e54942bf0c299c9c8a1f4d",
          "message": "update release",
          "timestamp": "2021-11-05T02:10:30Z",
          "tree_id": "14b0770328b5d9cff7c242a61a978e4180394428",
          "url": "https://github.com/sfu-db/connector-x/commit/77438d0ab7f16c8f31e54942bf0c299c9c8a1f4d"
        },
        "date": 1636079215703,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06744616873771102,
            "unit": "iter/sec",
            "range": "stddev: 0.2916237262408012",
            "extra": "mean: 14.826639062166214 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07417239903875457,
            "unit": "iter/sec",
            "range": "stddev: 2.9415067043761733",
            "extra": "mean: 13.482104030065239 sec\nrounds: 5"
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
          "id": "a10710d2a268a695082c959652540efbbe37f747",
          "message": "update release",
          "timestamp": "2021-11-05T03:10:32Z",
          "tree_id": "6797318b93f95bc2f5dffef77dd39207d80d0616",
          "url": "https://github.com/sfu-db/connector-x/commit/a10710d2a268a695082c959652540efbbe37f747"
        },
        "date": 1636082336493,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06777122331052658,
            "unit": "iter/sec",
            "range": "stddev: 0.36437254444651457",
            "extra": "mean: 14.755525297485292 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08731676626551373,
            "unit": "iter/sec",
            "range": "stddev: 1.9912104558063393",
            "extra": "mean: 11.452554220333695 sec\nrounds: 5"
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
          "distinct": false,
          "id": "a10710d2a268a695082c959652540efbbe37f747",
          "message": "update release",
          "timestamp": "2021-11-05T03:10:32Z",
          "tree_id": "6797318b93f95bc2f5dffef77dd39207d80d0616",
          "url": "https://github.com/sfu-db/connector-x/commit/a10710d2a268a695082c959652540efbbe37f747"
        },
        "date": 1636083927824,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0654021976683567,
            "unit": "iter/sec",
            "range": "stddev: 0.6670761761618038",
            "extra": "mean: 15.290006080083549 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07456568481245497,
            "unit": "iter/sec",
            "range": "stddev: 2.155062630146363",
            "extra": "mean: 13.410994648747145 sec\nrounds: 5"
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
          "id": "bd665587fa0304aa4b5fe3ab2dcd612997a89d5d",
          "message": "0.2.2-alpha.3: add mssql instance name",
          "timestamp": "2021-11-05T05:38:28Z",
          "tree_id": "a803c9aa4bf8c11615340c14ee7e5a286cb4128b",
          "url": "https://github.com/sfu-db/connector-x/commit/bd665587fa0304aa4b5fe3ab2dcd612997a89d5d"
        },
        "date": 1636091932768,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.059639656855164466,
            "unit": "iter/sec",
            "range": "stddev: 0.6536601398689655",
            "extra": "mean: 16.767366761155426 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0715655439391161,
            "unit": "iter/sec",
            "range": "stddev: 2.8564064649330674",
            "extra": "mean: 13.973204770870506 sec\nrounds: 5"
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
          "distinct": false,
          "id": "bd665587fa0304aa4b5fe3ab2dcd612997a89d5d",
          "message": "0.2.2-alpha.3: add mssql instance name",
          "timestamp": "2021-11-05T05:38:28Z",
          "tree_id": "a803c9aa4bf8c11615340c14ee7e5a286cb4128b",
          "url": "https://github.com/sfu-db/connector-x/commit/bd665587fa0304aa4b5fe3ab2dcd612997a89d5d"
        },
        "date": 1636093309866,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06078909818294486,
            "unit": "iter/sec",
            "range": "stddev: 0.11850730366963984",
            "extra": "mean: 16.450318065099417 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.056167765383764584,
            "unit": "iter/sec",
            "range": "stddev: 3.117850560349933",
            "extra": "mean: 17.803806029446424 sec\nrounds: 5"
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
          "id": "0c647a73b10aa873d3d8348762088f94a81af117",
          "message": "Merge pull request #161 from sfu-db/cte\n\nSupport CTE",
          "timestamp": "2021-11-11T22:54:33-08:00",
          "tree_id": "46b67c14958e74450a4ed5988eb76e08b13101b3",
          "url": "https://github.com/sfu-db/connector-x/commit/0c647a73b10aa873d3d8348762088f94a81af117"
        },
        "date": 1636700524106,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06913777770008858,
            "unit": "iter/sec",
            "range": "stddev: 0.21683420025376698",
            "extra": "mean: 14.463872477039695 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07712047381134357,
            "unit": "iter/sec",
            "range": "stddev: 2.9814076270346948",
            "extra": "mean: 12.966725314036012 sec\nrounds: 5"
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
          "id": "d2a67b35cae0af65917963a6c74e01601bba109d",
          "message": "0.2.2-alpha.4: support cte",
          "timestamp": "2021-11-12T17:41:23Z",
          "tree_id": "67cff6218d00a74aa66fa8951b1e0f3cd6f1f39d",
          "url": "https://github.com/sfu-db/connector-x/commit/d2a67b35cae0af65917963a6c74e01601bba109d"
        },
        "date": 1636739362822,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06852513580378827,
            "unit": "iter/sec",
            "range": "stddev: 0.24249763095058272",
            "extra": "mean: 14.593185234442354 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08114552413819721,
            "unit": "iter/sec",
            "range": "stddev: 2.503312264313342",
            "extra": "mean: 12.323538613133133 sec\nrounds: 5"
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
          "distinct": false,
          "id": "d2a67b35cae0af65917963a6c74e01601bba109d",
          "message": "0.2.2-alpha.4: support cte",
          "timestamp": "2021-11-12T17:41:23Z",
          "tree_id": "67cff6218d00a74aa66fa8951b1e0f3cd6f1f39d",
          "url": "https://github.com/sfu-db/connector-x/commit/d2a67b35cae0af65917963a6c74e01601bba109d"
        },
        "date": 1636741160297,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06747020612107639,
            "unit": "iter/sec",
            "range": "stddev: 0.2169595727900951",
            "extra": "mean: 14.821356825344264 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06135479843100491,
            "unit": "iter/sec",
            "range": "stddev: 2.590395847115318",
            "extra": "mean: 16.298643717728556 sec\nrounds: 5"
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
          "id": "1fd3630e09cec838ef92ed805a121aadbd388e6d",
          "message": "Merge pull request #147 from sfu-db/no_part_count\n\nStream write to destination",
          "timestamp": "2021-11-13T21:46:03-08:00",
          "tree_id": "f22770ed21b29cc126406fcea3cab36c1f8d3e60",
          "url": "https://github.com/sfu-db/connector-x/commit/1fd3630e09cec838ef92ed805a121aadbd388e6d"
        },
        "date": 1636869293507,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05969000293755856,
            "unit": "iter/sec",
            "range": "stddev: 0.6138584721073143",
            "extra": "mean: 16.753224171325563 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.058759493860267284,
            "unit": "iter/sec",
            "range": "stddev: 3.2038084480241187",
            "extra": "mean: 17.018526442348957 sec\nrounds: 5"
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
          "id": "2a953d07948aba346a950176d064e3553b637ed3",
          "message": "update benchmark result",
          "timestamp": "2021-11-14T15:17:07-08:00",
          "tree_id": "7ae3a7766966cee9357afca25cff3dab101e5150",
          "url": "https://github.com/sfu-db/connector-x/commit/2a953d07948aba346a950176d064e3553b637ed3"
        },
        "date": 1636932347087,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06554426334498256,
            "unit": "iter/sec",
            "range": "stddev: 0.6977676440984782",
            "extra": "mean: 15.256865345127881 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06018670858821551,
            "unit": "iter/sec",
            "range": "stddev: 1.33561752074978",
            "extra": "mean: 16.614964058622718 sec\nrounds: 5"
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
          "id": "79203dfd899906a68221527c46f2fc0c982d5838",
          "message": "support partition on decimal on postgres",
          "timestamp": "2021-11-15T00:34:18Z",
          "tree_id": "79e5c33be30b9b4f971e272fe78bda3ef17e2dbf",
          "url": "https://github.com/sfu-db/connector-x/commit/79203dfd899906a68221527c46f2fc0c982d5838"
        },
        "date": 1636936951457,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06628438651751327,
            "unit": "iter/sec",
            "range": "stddev: 0.6547144884551812",
            "extra": "mean: 15.086509094201029 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0697620992008957,
            "unit": "iter/sec",
            "range": "stddev: 1.1954122694106448",
            "extra": "mean: 14.33443103712052 sec\nrounds: 5"
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
          "id": "f5af9184f371d9ac4e95bf7e43a415169eaaaca6",
          "message": "0.2.2-alpha.5: update mssql fetch schema using prepared statement",
          "timestamp": "2021-11-15T21:53:08Z",
          "tree_id": "7b8e75ebd60e2a61d19082bae5164b09a288fe3c",
          "url": "https://github.com/sfu-db/connector-x/commit/f5af9184f371d9ac4e95bf7e43a415169eaaaca6"
        },
        "date": 1637013758145,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06608304948595889,
            "unit": "iter/sec",
            "range": "stddev: 1.428294570103043",
            "extra": "mean: 15.132473573461175 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07797221280301844,
            "unit": "iter/sec",
            "range": "stddev: 2.0351201355282877",
            "extra": "mean: 12.825081706047058 sec\nrounds: 5"
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
          "distinct": false,
          "id": "f5af9184f371d9ac4e95bf7e43a415169eaaaca6",
          "message": "0.2.2-alpha.5: update mssql fetch schema using prepared statement",
          "timestamp": "2021-11-15T21:53:08Z",
          "tree_id": "7b8e75ebd60e2a61d19082bae5164b09a288fe3c",
          "url": "https://github.com/sfu-db/connector-x/commit/f5af9184f371d9ac4e95bf7e43a415169eaaaca6"
        },
        "date": 1637015633345,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06474037141704903,
            "unit": "iter/sec",
            "range": "stddev: 1.5510701474225725",
            "extra": "mean: 15.446312372200191 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08206336133262898,
            "unit": "iter/sec",
            "range": "stddev: 1.9089070171033597",
            "extra": "mean: 12.185706066153944 sec\nrounds: 5"
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
            "email": "xiaoying_wang@sfu.ca",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "8beca50e0f2cf5ed8319c2a43acb3e307fd9ad6f",
          "message": "cherry pick: use Py::from_owned_ptr instead of from_borrowed_ptr",
          "timestamp": "2021-11-15T23:09:33Z",
          "tree_id": "edf297f6567274cb50cc9515cfe6ea5a11bb5a53",
          "url": "https://github.com/sfu-db/connector-x/commit/8beca50e0f2cf5ed8319c2a43acb3e307fd9ad6f"
        },
        "date": 1637018280547,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06660987584908493,
            "unit": "iter/sec",
            "range": "stddev: 0.28453998309325695",
            "extra": "mean: 15.012788828276097 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06815185364453585,
            "unit": "iter/sec",
            "range": "stddev: 1.3573071282640854",
            "extra": "mean: 14.673115205578506 sec\nrounds: 5"
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
            "email": "xiaoying_wang@sfu.ca",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "distinct": false,
          "id": "8beca50e0f2cf5ed8319c2a43acb3e307fd9ad6f",
          "message": "cherry pick: use Py::from_owned_ptr instead of from_borrowed_ptr",
          "timestamp": "2021-11-15T23:09:33Z",
          "tree_id": "edf297f6567274cb50cc9515cfe6ea5a11bb5a53",
          "url": "https://github.com/sfu-db/connector-x/commit/8beca50e0f2cf5ed8319c2a43acb3e307fd9ad6f"
        },
        "date": 1637019282322,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06690450505488688,
            "unit": "iter/sec",
            "range": "stddev: 1.208060478470984",
            "extra": "mean: 14.946676597930491 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08552109682419792,
            "unit": "iter/sec",
            "range": "stddev: 1.497715630435578",
            "extra": "mean: 11.693021220900118 sec\nrounds: 5"
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
          "id": "f569a5dd520bf51571c2c4845a54a9c771dad287",
          "message": "move constants",
          "timestamp": "2021-11-16T01:10:39Z",
          "tree_id": "9088d50b2b99dfae2da8d557bc6b80cc0a751252",
          "url": "https://github.com/sfu-db/connector-x/commit/f569a5dd520bf51571c2c4845a54a9c771dad287"
        },
        "date": 1637025512769,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07248620329195443,
            "unit": "iter/sec",
            "range": "stddev: 0.8196626513391446",
            "extra": "mean: 13.795728767476977 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07200221038532494,
            "unit": "iter/sec",
            "range": "stddev: 2.411656304720907",
            "extra": "mean: 13.8884625159204 sec\nrounds: 5"
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
          "id": "55d80e658580685946f9b733a763391bdbdbee9b",
          "message": "mysql unsigned integer support, not null support",
          "timestamp": "2021-11-16T20:07:46Z",
          "tree_id": "459b1fc86af7d48c7af93e771a417318559d62e2",
          "url": "https://github.com/sfu-db/connector-x/commit/55d80e658580685946f9b733a763391bdbdbee9b"
        },
        "date": 1637093811339,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06017830016579328,
            "unit": "iter/sec",
            "range": "stddev: 2.0616167731474695",
            "extra": "mean: 16.617285587079824 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08966623526204245,
            "unit": "iter/sec",
            "range": "stddev: 0.6125792190500562",
            "extra": "mean: 11.152470013685525 sec\nrounds: 5"
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
          "id": "7bb06cbdd206f8269cdc09b0bacb140299b7523c",
          "message": "fix: mysql min max parse",
          "timestamp": "2021-11-16T22:49:58Z",
          "tree_id": "3cddc4a31746f699eb512b3100b2079eae0f6fcc",
          "url": "https://github.com/sfu-db/connector-x/commit/7bb06cbdd206f8269cdc09b0bacb140299b7523c"
        },
        "date": 1637103501294,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06537412585014406,
            "unit": "iter/sec",
            "range": "stddev: 2.010071431148786",
            "extra": "mean: 15.29657164812088 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08383008309071775,
            "unit": "iter/sec",
            "range": "stddev: 2.4825803883404403",
            "extra": "mean: 11.928891910053789 sec\nrounds: 5"
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
          "id": "9ec2cbfa16a596e09ed09a31b814feeb0459cdae",
          "message": "support mssql money type",
          "timestamp": "2021-11-16T23:24:21Z",
          "tree_id": "67843604641c69ca3350ebfc858b573c481b34eb",
          "url": "https://github.com/sfu-db/connector-x/commit/9ec2cbfa16a596e09ed09a31b814feeb0459cdae"
        },
        "date": 1637105519614,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06706513754708558,
            "unit": "iter/sec",
            "range": "stddev: 1.1740166579348468",
            "extra": "mean: 14.910876747220755 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09518679380848347,
            "unit": "iter/sec",
            "range": "stddev: 0.8101557775259912",
            "extra": "mean: 10.505659030936659 sec\nrounds: 5"
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
          "id": "10415b61e20bdc46923b0ba19e5fccf26d0ff503",
          "message": "add test feature gate to ci",
          "timestamp": "2021-11-16T23:59:33Z",
          "tree_id": "b228cee5323c66dce0bdc172b8dac22dd984aef1",
          "url": "https://github.com/sfu-db/connector-x/commit/10415b61e20bdc46923b0ba19e5fccf26d0ff503"
        },
        "date": 1637107653569,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06819506970934554,
            "unit": "iter/sec",
            "range": "stddev: 0.4364884711877906",
            "extra": "mean: 14.663816669769584 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07043234358863822,
            "unit": "iter/sec",
            "range": "stddev: 1.7274786927113441",
            "extra": "mean: 14.198022514209152 sec\nrounds: 5"
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
          "id": "e788ffd4bd235be90e78b9fd3ed533dc7cd87e79",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-x into main",
          "timestamp": "2021-11-17T00:21:10Z",
          "tree_id": "a3dfd0a08e35fb710fb851b41e40f146b882e655",
          "url": "https://github.com/sfu-db/connector-x/commit/e788ffd4bd235be90e78b9fd3ed533dc7cd87e79"
        },
        "date": 1637108916167,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07252270659851474,
            "unit": "iter/sec",
            "range": "stddev: 0.1593145092511813",
            "extra": "mean: 13.788784877210855 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09182965834491746,
            "unit": "iter/sec",
            "range": "stddev: 0.8450916648552572",
            "extra": "mean: 10.88972798138857 sec\nrounds: 5"
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
          "id": "00554c3fbfff596e5dfae5499590f3d994a22fde",
          "message": "fix limit > count issue",
          "timestamp": "2021-11-17T19:02:52Z",
          "tree_id": "898168e232ba6af93b8509baa8d14f573dad0f44",
          "url": "https://github.com/sfu-db/connector-x/commit/00554c3fbfff596e5dfae5499590f3d994a22fde"
        },
        "date": 1637176281800,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07474853236262123,
            "unit": "iter/sec",
            "range": "stddev: 0.6234523670618128",
            "extra": "mean: 13.37818908803165 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05579567311845749,
            "unit": "iter/sec",
            "range": "stddev: 2.1460163147331177",
            "extra": "mean: 17.922536714933813 sec\nrounds: 5"
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
          "id": "2cb8bb2002a0b47f553c5af14a4eece437d6eaab",
          "message": "Merge pull request #173 from sfu-db/fix-postgres-bytea\n\nFix postgres to arrow type: bytea",
          "timestamp": "2021-11-20T22:02:44-08:00",
          "tree_id": "385381ee35f67fd72aa2c9e346db67ea06557309",
          "url": "https://github.com/sfu-db/connector-x/commit/2cb8bb2002a0b47f553c5af14a4eece437d6eaab"
        },
        "date": 1637475243849,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.061855615768619455,
            "unit": "iter/sec",
            "range": "stddev: 0.31686096954862664",
            "extra": "mean: 16.16668086759746 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.060677997885182605,
            "unit": "iter/sec",
            "range": "stddev: 3.110599947322996",
            "extra": "mean: 16.480438294820487 sec\nrounds: 5"
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
          "id": "0bc0b928e43c53b87c6ab4049f85a8042ac62bed",
          "message": "fix code format",
          "timestamp": "2021-11-21T06:17:54Z",
          "tree_id": "dd609877d5ec079f7ea5721a5ec6463f8d0d8b8f",
          "url": "https://github.com/sfu-db/connector-x/commit/0bc0b928e43c53b87c6ab4049f85a8042ac62bed"
        },
        "date": 1637476111963,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05993170234855769,
            "unit": "iter/sec",
            "range": "stddev: 0.21444743948727876",
            "extra": "mean: 16.68565985634923 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07091493069100333,
            "unit": "iter/sec",
            "range": "stddev: 0.7037612872381254",
            "extra": "mean: 14.101402768865228 sec\nrounds: 5"
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
          "id": "35988af729489603d18bdb9998bf97eb3ae39614",
          "message": "0.2.2",
          "timestamp": "2021-11-23T17:22:25Z",
          "tree_id": "e6b6bee601e38c5292c4534816d1b96f0c1916d8",
          "url": "https://github.com/sfu-db/connector-x/commit/35988af729489603d18bdb9998bf97eb3ae39614"
        },
        "date": 1637688954208,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.043629528235938735,
            "unit": "iter/sec",
            "range": "stddev: 5.7852119414232055",
            "extra": "mean: 22.920257000997662 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05975466888588335,
            "unit": "iter/sec",
            "range": "stddev: 3.233248370045981",
            "extra": "mean: 16.73509398754686 sec\nrounds: 5"
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
          "distinct": false,
          "id": "35988af729489603d18bdb9998bf97eb3ae39614",
          "message": "0.2.2",
          "timestamp": "2021-11-23T17:22:25Z",
          "tree_id": "e6b6bee601e38c5292c4534816d1b96f0c1916d8",
          "url": "https://github.com/sfu-db/connector-x/commit/35988af729489603d18bdb9998bf97eb3ae39614"
        },
        "date": 1637690377353,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06397625884680108,
            "unit": "iter/sec",
            "range": "stddev: 0.5411596524476069",
            "extra": "mean: 15.630798330903053 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06908595959253208,
            "unit": "iter/sec",
            "range": "stddev: 2.2396343930048768",
            "extra": "mean: 14.47472114302218 sec\nrounds: 5"
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
          "id": "1556e59d8fc54c4e955549b99703cb2efa92fa76",
          "message": "Merge pull request #184 from sfu-db/revert-183-mssql-auth\n\nRevert \"Mssql trusted connection support\"",
          "timestamp": "2021-12-02T22:09:00-08:00",
          "tree_id": "e6b6bee601e38c5292c4534816d1b96f0c1916d8",
          "url": "https://github.com/sfu-db/connector-x/commit/1556e59d8fc54c4e955549b99703cb2efa92fa76"
        },
        "date": 1638518971657,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06211865575883408,
            "unit": "iter/sec",
            "range": "stddev: 0.43651100154492367",
            "extra": "mean: 16.098223436810077 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07223867473300986,
            "unit": "iter/sec",
            "range": "stddev: 2.3263843325413895",
            "extra": "mean: 13.843000355362893 sec\nrounds: 5"
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
          "id": "01736efacf98d67f72a1f9b465fae399e26adb03",
          "message": "Merge pull request #185 from sfu-db/mssql-tc\n\nsupport windows trusted connection",
          "timestamp": "2021-12-02T22:23:13-08:00",
          "tree_id": "f115d17db525d793d8b501d07fcb5410761362bd",
          "url": "https://github.com/sfu-db/connector-x/commit/01736efacf98d67f72a1f9b465fae399e26adb03"
        },
        "date": 1638519485616,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06359091719376382,
            "unit": "iter/sec",
            "range": "stddev: 0.3852710188756765",
            "extra": "mean: 15.725516223534942 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07074365945800475,
            "unit": "iter/sec",
            "range": "stddev: 2.8513708084580776",
            "extra": "mean: 14.135542431101204 sec\nrounds: 5"
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
          "id": "5adc0a7096e6e290c46737594b94ed535d1d2009",
          "message": "0.2.3-alpha.1: support windows trusted connection",
          "timestamp": "2021-12-03T18:35:23Z",
          "tree_id": "d6143867f3646b44b9d5a632c80553cdda2c635d",
          "url": "https://github.com/sfu-db/connector-x/commit/5adc0a7096e6e290c46737594b94ed535d1d2009"
        },
        "date": 1638557075715,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.068452907107084,
            "unit": "iter/sec",
            "range": "stddev: 0.45599181042311093",
            "extra": "mean: 14.608583364263177 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07562252933052553,
            "unit": "iter/sec",
            "range": "stddev: 1.4393286491745254",
            "extra": "mean: 13.223572510108351 sec\nrounds: 5"
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
          "distinct": false,
          "id": "5adc0a7096e6e290c46737594b94ed535d1d2009",
          "message": "0.2.3-alpha.1: support windows trusted connection",
          "timestamp": "2021-12-03T18:35:23Z",
          "tree_id": "d6143867f3646b44b9d5a632c80553cdda2c635d",
          "url": "https://github.com/sfu-db/connector-x/commit/5adc0a7096e6e290c46737594b94ed535d1d2009"
        },
        "date": 1638562654409,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06272494372487164,
            "unit": "iter/sec",
            "range": "stddev: 0.5613044679484034",
            "extra": "mean: 15.942620919458568 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06970871160169113,
            "unit": "iter/sec",
            "range": "stddev: 3.291714010854349",
            "extra": "mean: 14.345409304276108 sec\nrounds: 5"
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
          "id": "8c95ceda76a5461274717acfaf6fbb19e4435e11",
          "message": "Merge pull request #187 from alexander-beedie/main\n\nAutomatically select 'cursor' protocol for redshift-prefixed connection strings",
          "timestamp": "2021-12-06T09:45:58-08:00",
          "tree_id": "e0c02ebc3c0a432edc489c1d8da3ea687aa9654a",
          "url": "https://github.com/sfu-db/connector-x/commit/8c95ceda76a5461274717acfaf6fbb19e4435e11"
        },
        "date": 1638813286244,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06061001406263713,
            "unit": "iter/sec",
            "range": "stddev: 0.9310199109541016",
            "extra": "mean: 16.498923741653563 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08226175366652039,
            "unit": "iter/sec",
            "range": "stddev: 1.5710056452711936",
            "extra": "mean: 12.156317552551627 sec\nrounds: 5"
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
          "id": "7d193711affb004ebed6966d05f09ff9f722371c",
          "message": "0.2.3-alpha.2: support redshift scheme in connection uri",
          "timestamp": "2021-12-06T17:54:12Z",
          "tree_id": "baaa30aafcbe14f83b6a07e82d6b99e2333d0ed6",
          "url": "https://github.com/sfu-db/connector-x/commit/7d193711affb004ebed6966d05f09ff9f722371c"
        },
        "date": 1638813772162,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0710058162487694,
            "unit": "iter/sec",
            "range": "stddev: 0.35193543762717894",
            "extra": "mean: 14.083353348076344 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08243809283884178,
            "unit": "iter/sec",
            "range": "stddev: 1.3657940625782536",
            "extra": "mean: 12.13031458593905 sec\nrounds: 5"
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
          "distinct": false,
          "id": "7d193711affb004ebed6966d05f09ff9f722371c",
          "message": "0.2.3-alpha.2: support redshift scheme in connection uri",
          "timestamp": "2021-12-06T17:54:12Z",
          "tree_id": "baaa30aafcbe14f83b6a07e82d6b99e2333d0ed6",
          "url": "https://github.com/sfu-db/connector-x/commit/7d193711affb004ebed6966d05f09ff9f722371c"
        },
        "date": 1638815736930,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06502682709879548,
            "unit": "iter/sec",
            "range": "stddev: 0.4854378482596785",
            "extra": "mean: 15.378268394991755 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08719049355682726,
            "unit": "iter/sec",
            "range": "stddev: 0.7482806712252041",
            "extra": "mean: 11.46914026066661 sec\nrounds: 5"
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
          "id": "ce5e955dbb12e2f2dd571fe1199dd3cd0eefb3e6",
          "message": "Merge pull request #191 from alexander-beedie/main\n\nAutomatically select 'text' protocol for clickhouse-prefixed connection strings",
          "timestamp": "2021-12-09T08:46:51-08:00",
          "tree_id": "dd423da2af5d9e5b25975c374e709449e440fa34",
          "url": "https://github.com/sfu-db/connector-x/commit/ce5e955dbb12e2f2dd571fe1199dd3cd0eefb3e6"
        },
        "date": 1639068993866,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06479166565646463,
            "unit": "iter/sec",
            "range": "stddev: 0.6409150683664855",
            "extra": "mean: 15.43408384192735 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05600656283001814,
            "unit": "iter/sec",
            "range": "stddev: 3.6043649919483625",
            "extra": "mean: 17.855050363205372 sec\nrounds: 5"
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
          "id": "fa5fb0c9f2a3e2ff208c14da9fdf98f49b11f1c6",
          "message": "add mediumtext type test for mysql",
          "timestamp": "2021-12-11T01:33:34Z",
          "tree_id": "97218f79c5c2c5bd8a489f78aa55459313096da7",
          "url": "https://github.com/sfu-db/connector-x/commit/fa5fb0c9f2a3e2ff208c14da9fdf98f49b11f1c6"
        },
        "date": 1639187091314,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06657255880818372,
            "unit": "iter/sec",
            "range": "stddev: 0.953626415781205",
            "extra": "mean: 15.021204200387 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0637141821307167,
            "unit": "iter/sec",
            "range": "stddev: 0.8028905542308876",
            "extra": "mean: 15.69509278088808 sec\nrounds: 5"
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
          "id": "21188b007eff83f6a118307c4ae318d5df7251d8",
          "message": "0.2.23-alpha.3: fix arrow binary field mapping",
          "timestamp": "2021-12-12T05:33:32Z",
          "tree_id": "80689c25572e78994e9b008549c2fa21cd2194a2",
          "url": "https://github.com/sfu-db/connector-x/commit/21188b007eff83f6a118307c4ae318d5df7251d8"
        },
        "date": 1639287793637,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05436889806757964,
            "unit": "iter/sec",
            "range": "stddev: 2.548062339342976",
            "extra": "mean: 18.392868635244668 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07583959170996982,
            "unit": "iter/sec",
            "range": "stddev: 2.5290849134639033",
            "extra": "mean: 13.18572499472648 sec\nrounds: 5"
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
          "distinct": false,
          "id": "ee14fd3ad3a47e5b5a6fb4e3dd70dc78e0b6928d",
          "message": "update test for mariadb",
          "timestamp": "2021-12-12T05:58:08Z",
          "tree_id": "323f7f90b088676bf6be217822fdb576b117fd10",
          "url": "https://github.com/sfu-db/connector-x/commit/ee14fd3ad3a47e5b5a6fb4e3dd70dc78e0b6928d"
        },
        "date": 1639289408720,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07323996059916722,
            "unit": "iter/sec",
            "range": "stddev: 1.1029546839808853",
            "extra": "mean: 13.65374847035855 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09430056415609334,
            "unit": "iter/sec",
            "range": "stddev: 0.74089618836441",
            "extra": "mean: 10.604390429146587 sec\nrounds: 5"
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
          "id": "b94e37dc4144ed5c3d8ef0893e72a20606baf42a",
          "message": "Update README.md",
          "timestamp": "2021-12-12T18:33:37-08:00",
          "tree_id": "59c12929994a5c21894a248d64dfd44f68cb908f",
          "url": "https://github.com/sfu-db/connector-x/commit/b94e37dc4144ed5c3d8ef0893e72a20606baf42a"
        },
        "date": 1639363388358,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06162463671910648,
            "unit": "iter/sec",
            "range": "stddev: 1.0809969934022532",
            "extra": "mean: 16.227276187576354 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0814824341316219,
            "unit": "iter/sec",
            "range": "stddev: 1.1294848561779638",
            "extra": "mean: 12.272583786398172 sec\nrounds: 5"
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
          "id": "621ed1019903fdcdc2c7d4cc58e97685022aaca0",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-x into main",
          "timestamp": "2021-12-15T01:39:53Z",
          "tree_id": "a99947296c527a777d13980398debbb9f11c4c98",
          "url": "https://github.com/sfu-db/connector-x/commit/621ed1019903fdcdc2c7d4cc58e97685022aaca0"
        },
        "date": 1639533006405,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.048753686292303124,
            "unit": "iter/sec",
            "range": "stddev: 8.163125051388358",
            "extra": "mean: 20.511269527487457 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0875304432998124,
            "unit": "iter/sec",
            "range": "stddev: 0.9096624215590114",
            "extra": "mean: 11.42459654379636 sec\nrounds: 5"
          }
        ]
      }
    ]
  }
}