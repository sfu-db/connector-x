window.BENCHMARK_DATA = {
  "lastUpdate": 1652156749425,
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
          "id": "f56008ef0a10ab16ebd2cc4bd2792b7f5ab55a76",
          "message": "upload tar.gz to pypi",
          "timestamp": "2021-12-17T02:30:32Z",
          "tree_id": "d593ff11573bb6c6078c206c2b075c2ab23314c1",
          "url": "https://github.com/sfu-db/connector-x/commit/f56008ef0a10ab16ebd2cc4bd2792b7f5ab55a76"
        },
        "date": 1639708858707,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05190105482154384,
            "unit": "iter/sec",
            "range": "stddev: 3.8545168051291956",
            "extra": "mean: 19.267431142553686 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07801829138828252,
            "unit": "iter/sec",
            "range": "stddev: 1.5869877751371406",
            "extra": "mean: 12.817507051303982 sec\nrounds: 5"
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
          "id": "f56008ef0a10ab16ebd2cc4bd2792b7f5ab55a76",
          "message": "upload tar.gz to pypi",
          "timestamp": "2021-12-17T02:30:32Z",
          "tree_id": "d593ff11573bb6c6078c206c2b075c2ab23314c1",
          "url": "https://github.com/sfu-db/connector-x/commit/f56008ef0a10ab16ebd2cc4bd2792b7f5ab55a76"
        },
        "date": 1639709383003,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06155836443341798,
            "unit": "iter/sec",
            "range": "stddev: 0.3765757191448877",
            "extra": "mean: 16.24474609103054 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06666735040932133,
            "unit": "iter/sec",
            "range": "stddev: 1.781959064461869",
            "extra": "mean: 14.999846159480512 sec\nrounds: 5"
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
          "id": "28447e2f273a32484e5e987b541a826d83c20dae",
          "message": "add license to poetry build",
          "timestamp": "2021-12-17T17:36:35Z",
          "tree_id": "98e4f7f0a5c240a519b43be87901ad331070eb91",
          "url": "https://github.com/sfu-db/connector-x/commit/28447e2f273a32484e5e987b541a826d83c20dae"
        },
        "date": 1639763168587,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06357740640902541,
            "unit": "iter/sec",
            "range": "stddev: 0.8312985716337568",
            "extra": "mean: 15.728858040645719 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0813154147372502,
            "unit": "iter/sec",
            "range": "stddev: 1.8554888929935307",
            "extra": "mean: 12.297791301086544 sec\nrounds: 5"
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
          "id": "28447e2f273a32484e5e987b541a826d83c20dae",
          "message": "add license to poetry build",
          "timestamp": "2021-12-17T17:36:35Z",
          "tree_id": "98e4f7f0a5c240a519b43be87901ad331070eb91",
          "url": "https://github.com/sfu-db/connector-x/commit/28447e2f273a32484e5e987b541a826d83c20dae"
        },
        "date": 1639763664263,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06558094566975621,
            "unit": "iter/sec",
            "range": "stddev: 0.3106953403369247",
            "extra": "mean: 15.24833150524646 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07481626977195947,
            "unit": "iter/sec",
            "range": "stddev: 2.579654824249856",
            "extra": "mean: 13.366076697595417 sec\nrounds: 5"
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
          "id": "f9940ed95e9ec77c5b3e3bcde43bb9522f68000c",
          "message": "0.2.3-alpha.5 include LICENSE to tar.gz",
          "timestamp": "2021-12-17T19:27:37Z",
          "tree_id": "ec03e41db5b68185ecd5dcd1a5745eb78f1ee55b",
          "url": "https://github.com/sfu-db/connector-x/commit/f9940ed95e9ec77c5b3e3bcde43bb9522f68000c"
        },
        "date": 1639769774978,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07195894495686925,
            "unit": "iter/sec",
            "range": "stddev: 0.425153720696806",
            "extra": "mean: 13.896812975779175 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0658106604628705,
            "unit": "iter/sec",
            "range": "stddev: 5.674869657858707",
            "extra": "mean: 15.195106582529842 sec\nrounds: 5"
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
          "id": "f9940ed95e9ec77c5b3e3bcde43bb9522f68000c",
          "message": "0.2.3-alpha.5 include LICENSE to tar.gz",
          "timestamp": "2021-12-17T19:27:37Z",
          "tree_id": "ec03e41db5b68185ecd5dcd1a5745eb78f1ee55b",
          "url": "https://github.com/sfu-db/connector-x/commit/f9940ed95e9ec77c5b3e3bcde43bb9522f68000c"
        },
        "date": 1639770277514,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0725577292768429,
            "unit": "iter/sec",
            "range": "stddev: 0.7904561233386084",
            "extra": "mean: 13.782129208929836 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07109598620486482,
            "unit": "iter/sec",
            "range": "stddev: 3.0639781765855005",
            "extra": "mean: 14.065491645596921 sec\nrounds: 5"
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
          "id": "48959125eab9ac49bedcf9d0ab740e1eb0aa7709",
          "message": "Merge pull request #198 from sfu-db/google_bigquery\n\nGoogle Bigquery Code",
          "timestamp": "2021-12-19T23:09:56-08:00",
          "tree_id": "3b591b54a66ab58d948bcb342c23a8e591f277e4",
          "url": "https://github.com/sfu-db/connector-x/commit/48959125eab9ac49bedcf9d0ab740e1eb0aa7709"
        },
        "date": 1639984748814,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06697147230668017,
            "unit": "iter/sec",
            "range": "stddev: 0.9187726283633174",
            "extra": "mean: 14.931730863265694 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06870453397782353,
            "unit": "iter/sec",
            "range": "stddev: 3.724581671036423",
            "extra": "mean: 14.555080168694257 sec\nrounds: 5"
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
          "id": "9931c4492c9323f7277c15fb519178e3be24acc2",
          "message": "skip tests for big query",
          "timestamp": "2021-12-20T19:57:01Z",
          "tree_id": "7584ecea703ead503b67e0340a76d5d4752fdf68",
          "url": "https://github.com/sfu-db/connector-x/commit/9931c4492c9323f7277c15fb519178e3be24acc2"
        },
        "date": 1640030844339,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0651444745794199,
            "unit": "iter/sec",
            "range": "stddev: 0.6072619516250749",
            "extra": "mean: 15.350496054440736 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0754989624320954,
            "unit": "iter/sec",
            "range": "stddev: 2.304307064911964",
            "extra": "mean: 13.245215136557817 sec\nrounds: 5"
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
          "id": "f45755d7d2401a59ff8cb6aebfc3e39bf8f547b9",
          "message": "0.2.3",
          "timestamp": "2021-12-21T18:10:36Z",
          "tree_id": "9bf816a8e1dc5fcdfd738dd65f088f41b94c9d87",
          "url": "https://github.com/sfu-db/connector-x/commit/f45755d7d2401a59ff8cb6aebfc3e39bf8f547b9"
        },
        "date": 1640110881714,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.059907739803491525,
            "unit": "iter/sec",
            "range": "stddev: 1.2410526560730675",
            "extra": "mean: 16.692333966866137 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06701449295125791,
            "unit": "iter/sec",
            "range": "stddev: 3.093105443202446",
            "extra": "mean: 14.922145284712315 sec\nrounds: 5"
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
          "id": "f45755d7d2401a59ff8cb6aebfc3e39bf8f547b9",
          "message": "0.2.3",
          "timestamp": "2021-12-21T18:10:36Z",
          "tree_id": "9bf816a8e1dc5fcdfd738dd65f088f41b94c9d87",
          "url": "https://github.com/sfu-db/connector-x/commit/f45755d7d2401a59ff8cb6aebfc3e39bf8f547b9"
        },
        "date": 1640113377180,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06447866115132503,
            "unit": "iter/sec",
            "range": "stddev: 1.6261716980114405",
            "extra": "mean: 15.509006889164448 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06511302505876332,
            "unit": "iter/sec",
            "range": "stddev: 3.4300804893820227",
            "extra": "mean: 15.357910327427089 sec\nrounds: 5"
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
          "id": "f115478027b606aef2f7230d6a6feef8056b533a",
          "message": "Merge pull request #203 from wseaton/feature/orcale_system_auth\n\ndraft: enable system auth for oracle based connections - merge for test",
          "timestamp": "2021-12-21T12:57:35-08:00",
          "tree_id": "e3283ad8e738ac128a87b93bc381bd19b40aecc5",
          "url": "https://github.com/sfu-db/connector-x/commit/f115478027b606aef2f7230d6a6feef8056b533a"
        },
        "date": 1640120866920,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06532279616701098,
            "unit": "iter/sec",
            "range": "stddev: 0.5225099249487745",
            "extra": "mean: 15.308591466955841 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0587378471811647,
            "unit": "iter/sec",
            "range": "stddev: 1.6957420682840336",
            "extra": "mean: 17.024798285774885 sec\nrounds: 5"
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
          "id": "f115478027b606aef2f7230d6a6feef8056b533a",
          "message": "Merge pull request #203 from wseaton/feature/orcale_system_auth\n\ndraft: enable system auth for oracle based connections - merge for test",
          "timestamp": "2021-12-21T12:57:35-08:00",
          "tree_id": "e3283ad8e738ac128a87b93bc381bd19b40aecc5",
          "url": "https://github.com/sfu-db/connector-x/commit/f115478027b606aef2f7230d6a6feef8056b533a"
        },
        "date": 1640123930359,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0641318875512691,
            "unit": "iter/sec",
            "range": "stddev: 0.6866939903381881",
            "extra": "mean: 15.592867108434438 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07960008043597964,
            "unit": "iter/sec",
            "range": "stddev: 1.5130661568786024",
            "extra": "mean: 12.562801375612617 sec\nrounds: 5"
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
          "id": "50572ca861734d08808ea976e925a2f046e6709b",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-x into main",
          "timestamp": "2021-12-24T05:08:55Z",
          "tree_id": "44782a6d56dbb3f9e0bed8726717700a507a8ce9",
          "url": "https://github.com/sfu-db/connector-x/commit/50572ca861734d08808ea976e925a2f046e6709b"
        },
        "date": 1640323134535,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06600964394570377,
            "unit": "iter/sec",
            "range": "stddev: 0.9954380097111186",
            "extra": "mean: 15.149301529675721 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07632697652923465,
            "unit": "iter/sec",
            "range": "stddev: 1.5864075951067989",
            "extra": "mean: 13.101527735963463 sec\nrounds: 5"
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
          "id": "50572ca861734d08808ea976e925a2f046e6709b",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-x into main",
          "timestamp": "2021-12-24T05:08:55Z",
          "tree_id": "44782a6d56dbb3f9e0bed8726717700a507a8ce9",
          "url": "https://github.com/sfu-db/connector-x/commit/50572ca861734d08808ea976e925a2f046e6709b"
        },
        "date": 1640323684927,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06483384943750045,
            "unit": "iter/sec",
            "range": "stddev: 0.32910888723051773",
            "extra": "mean: 15.424041741713882 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07905654514051617,
            "unit": "iter/sec",
            "range": "stddev: 1.9588883096342038",
            "extra": "mean: 12.649174059182405 sec\nrounds: 5"
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
          "id": "663d192cb5b43e03f3a8d0543d587c0bedbaebeb",
          "message": "update black",
          "timestamp": "2021-12-24T06:15:15Z",
          "tree_id": "9fff6b72c153dbaaee2d02ca33ae8d54f92c7cb1",
          "url": "https://github.com/sfu-db/connector-x/commit/663d192cb5b43e03f3a8d0543d587c0bedbaebeb"
        },
        "date": 1640327116648,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0658191812375649,
            "unit": "iter/sec",
            "range": "stddev: 0.5757654967982389",
            "extra": "mean: 15.193139464780689 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07795992717150858,
            "unit": "iter/sec",
            "range": "stddev: 1.092968503321349",
            "extra": "mean: 12.827102798596025 sec\nrounds: 5"
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
          "id": "24ee57dd17ef75f22a4de3eb6351244a738a65b9",
          "message": "temp: use 3.10.0 for release",
          "timestamp": "2021-12-24T06:25:06Z",
          "tree_id": "c44be0f8e287c11d6ddbecf7acb4fd799684234d",
          "url": "https://github.com/sfu-db/connector-x/commit/24ee57dd17ef75f22a4de3eb6351244a738a65b9"
        },
        "date": 1640327757315,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06302925603061775,
            "unit": "iter/sec",
            "range": "stddev: 0.25375460364049623",
            "extra": "mean: 15.865648160502314 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0767955572130794,
            "unit": "iter/sec",
            "range": "stddev: 2.4778751046101144",
            "extra": "mean: 13.021586616337299 sec\nrounds: 5"
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
          "id": "b48cc84a30d8e92c4be4d9c66794fd049b14b96f",
          "message": "change python 3.10.0 back to 3.10 in release",
          "timestamp": "2021-12-24T06:51:40Z",
          "tree_id": "32a86ea53015dbd5319da46c51c217072a6f58ea",
          "url": "https://github.com/sfu-db/connector-x/commit/b48cc84a30d8e92c4be4d9c66794fd049b14b96f"
        },
        "date": 1640329235872,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0640431281630969,
            "unit": "iter/sec",
            "range": "stddev: 1.1116450519468806",
            "extra": "mean: 15.61447775401175 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08164579850583586,
            "unit": "iter/sec",
            "range": "stddev: 1.4452141439484367",
            "extra": "mean: 12.24802767932415 sec\nrounds: 5"
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
          "id": "0567600c95715719941635e78296521a99d8f034",
          "message": "update mysqlclient (support python 3.10)",
          "timestamp": "2021-12-24T07:01:10Z",
          "tree_id": "75272fb98ddbee7e7acb654c9d2a592d544aa5a3",
          "url": "https://github.com/sfu-db/connector-x/commit/0567600c95715719941635e78296521a99d8f034"
        },
        "date": 1640329857140,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06542092530154718,
            "unit": "iter/sec",
            "range": "stddev: 0.3495910494786387",
            "extra": "mean: 15.285629107058048 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08799722639061813,
            "unit": "iter/sec",
            "range": "stddev: 0.853103857676554",
            "extra": "mean: 11.36399453729391 sec\nrounds: 5"
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
          "id": "be3817fa0b140aa5cfef825dced15b943fa2ddf0",
          "message": "Update CONTRIBUTING.md",
          "timestamp": "2021-12-24T21:28:05-08:00",
          "tree_id": "5875b6f98ed0d58a25eb19e07f9a6bdf7f2ed20b",
          "url": "https://github.com/sfu-db/connector-x/commit/be3817fa0b140aa5cfef825dced15b943fa2ddf0"
        },
        "date": 1640410684049,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06501668165530808,
            "unit": "iter/sec",
            "range": "stddev: 0.4269803655857476",
            "extra": "mean: 15.380668076872826 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08747511653622825,
            "unit": "iter/sec",
            "range": "stddev: 0.8819093301508746",
            "extra": "mean: 11.431822438165545 sec\nrounds: 5"
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
          "id": "a0911ca50ea3356729b64f6498c0bfdbc6d86875",
          "message": "0.2.4-alpha.3: expose arrow2 and shrink array before return",
          "timestamp": "2022-01-05T05:00:57Z",
          "tree_id": "b1544737166df163089893375ad92643fdcbe7bc",
          "url": "https://github.com/sfu-db/connector-x/commit/a0911ca50ea3356729b64f6498c0bfdbc6d86875"
        },
        "date": 1641359571865,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06452349613607557,
            "unit": "iter/sec",
            "range": "stddev: 0.5133057859867081",
            "extra": "mean: 15.498230255395175 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08790817231829402,
            "unit": "iter/sec",
            "range": "stddev: 0.7855455579457499",
            "extra": "mean: 11.375506663694978 sec\nrounds: 5"
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
          "id": "a0911ca50ea3356729b64f6498c0bfdbc6d86875",
          "message": "0.2.4-alpha.3: expose arrow2 and shrink array before return",
          "timestamp": "2022-01-05T05:00:57Z",
          "tree_id": "b1544737166df163089893375ad92643fdcbe7bc",
          "url": "https://github.com/sfu-db/connector-x/commit/a0911ca50ea3356729b64f6498c0bfdbc6d86875"
        },
        "date": 1641361761485,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06105836567910806,
            "unit": "iter/sec",
            "range": "stddev: 0.4636575874684499",
            "extra": "mean: 16.377772134542464 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07933088803818696,
            "unit": "iter/sec",
            "range": "stddev: 1.9159832680599118",
            "extra": "mean: 12.60543055459857 sec\nrounds: 5"
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
          "id": "85d3cb4651b70063ac73bb115c402651131dcd9d",
          "message": "fix numpy version for python 3.10.1",
          "timestamp": "2022-01-05T07:03:44Z",
          "tree_id": "2d131cbe1a34e81d1167bcdd91b82d85ee6b9207",
          "url": "https://github.com/sfu-db/connector-x/commit/85d3cb4651b70063ac73bb115c402651131dcd9d"
        },
        "date": 1641366979916,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06407299322613337,
            "unit": "iter/sec",
            "range": "stddev: 0.6005008500449175",
            "extra": "mean: 15.607199689745903 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07166242421528976,
            "unit": "iter/sec",
            "range": "stddev: 2.9252787022201026",
            "extra": "mean: 13.954314425587654 sec\nrounds: 5"
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
          "id": "275a5baa298ba60e7d7b86beea8cdf8f92409495",
          "message": "update pymssql version for python 3.10",
          "timestamp": "2022-01-05T07:23:54Z",
          "tree_id": "b98c56ccf940e017b36b12ce4d7adcd1811d1de3",
          "url": "https://github.com/sfu-db/connector-x/commit/275a5baa298ba60e7d7b86beea8cdf8f92409495"
        },
        "date": 1641368155732,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06383719821992799,
            "unit": "iter/sec",
            "range": "stddev: 0.21883443227947946",
            "extra": "mean: 15.664847892522811 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07025158362924967,
            "unit": "iter/sec",
            "range": "stddev: 3.4261810125180747",
            "extra": "mean: 14.234554558619857 sec\nrounds: 5"
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
          "id": "275a5baa298ba60e7d7b86beea8cdf8f92409495",
          "message": "update pymssql version for python 3.10",
          "timestamp": "2022-01-05T07:23:54Z",
          "tree_id": "b98c56ccf940e017b36b12ce4d7adcd1811d1de3",
          "url": "https://github.com/sfu-db/connector-x/commit/275a5baa298ba60e7d7b86beea8cdf8f92409495"
        },
        "date": 1641368851724,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06565054891455102,
            "unit": "iter/sec",
            "range": "stddev: 0.9812213251706774",
            "extra": "mean: 15.232165100425481 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07027151750929145,
            "unit": "iter/sec",
            "range": "stddev: 1.9112720223571078",
            "extra": "mean: 14.230516650900245 sec\nrounds: 5"
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
          "id": "4daa1b3f896256fff0cdb054df47aadc205ea1bf",
          "message": "remove debug message, update psycopg2-binary for cp310",
          "timestamp": "2022-01-06T05:33:35Z",
          "tree_id": "2222300be2f024644a598d41f14b6028550a7700",
          "url": "https://github.com/sfu-db/connector-x/commit/4daa1b3f896256fff0cdb054df47aadc205ea1bf"
        },
        "date": 1641447978529,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0656165342562776,
            "unit": "iter/sec",
            "range": "stddev: 0.6334774830523306",
            "extra": "mean: 15.24006123356521 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07484942513939677,
            "unit": "iter/sec",
            "range": "stddev: 2.2029067364192674",
            "extra": "mean: 13.360156048461794 sec\nrounds: 5"
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
          "id": "076062f783537b62f904a4b91c311ded10dcca40",
          "message": "0.2.4-alpha.4: fix deallocating None issue",
          "timestamp": "2022-01-09T04:57:52Z",
          "tree_id": "fe47a29c7a2887d02536d5788996d6b649e75245",
          "url": "https://github.com/sfu-db/connector-x/commit/076062f783537b62f904a4b91c311ded10dcca40"
        },
        "date": 1641705037291,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06528750890631647,
            "unit": "iter/sec",
            "range": "stddev: 0.40291681889261405",
            "extra": "mean: 15.316865611076356 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0765330059004782,
            "unit": "iter/sec",
            "range": "stddev: 1.3787367509138893",
            "extra": "mean: 13.06625799201429 sec\nrounds: 5"
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
          "id": "076062f783537b62f904a4b91c311ded10dcca40",
          "message": "0.2.4-alpha.4: fix deallocating None issue",
          "timestamp": "2022-01-09T04:57:52Z",
          "tree_id": "fe47a29c7a2887d02536d5788996d6b649e75245",
          "url": "https://github.com/sfu-db/connector-x/commit/076062f783537b62f904a4b91c311ded10dcca40"
        },
        "date": 1641706711313,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0611369710250678,
            "unit": "iter/sec",
            "range": "stddev: 0.5836143814404241",
            "extra": "mean: 16.35671481974423 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07598160335214388,
            "unit": "iter/sec",
            "range": "stddev: 1.6306492851253285",
            "extra": "mean: 13.161080523207783 sec\nrounds: 5"
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
          "id": "39c654da876ed48a5b28f8af82a6a280f937b795",
          "message": "0.2.4-alpha.5: add license back to source file",
          "timestamp": "2022-01-11T04:43:08Z",
          "tree_id": "c8bae53c909f15162c82c07040b60cb10795052b",
          "url": "https://github.com/sfu-db/connector-x/commit/39c654da876ed48a5b28f8af82a6a280f937b795"
        },
        "date": 1641876972090,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06058637516963788,
            "unit": "iter/sec",
            "range": "stddev: 1.6594295791488012",
            "extra": "mean: 16.50536110140383 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07826469317252019,
            "unit": "iter/sec",
            "range": "stddev: 2.7704397578092337",
            "extra": "mean: 12.7771535217762 sec\nrounds: 5"
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
          "id": "39c654da876ed48a5b28f8af82a6a280f937b795",
          "message": "0.2.4-alpha.5: add license back to source file",
          "timestamp": "2022-01-11T04:43:08Z",
          "tree_id": "c8bae53c909f15162c82c07040b60cb10795052b",
          "url": "https://github.com/sfu-db/connector-x/commit/39c654da876ed48a5b28f8af82a6a280f937b795"
        },
        "date": 1641878485688,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.041651575035001526,
            "unit": "iter/sec",
            "range": "stddev: 14.539400863193883",
            "extra": "mean: 24.00869592949748 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06128130799657212,
            "unit": "iter/sec",
            "range": "stddev: 10.108518986148503",
            "extra": "mean: 16.318189553916454 sec\nrounds: 5"
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
            "email": "wangxiaoying0369@gmail.com",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "93018d2c0a21f985e212bf9811e443ccd77aea5f",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-x into main",
          "timestamp": "2022-01-13T03:04:46Z",
          "tree_id": "ce80a575a81e57d5180d88e2dbf46467d967370c",
          "url": "https://github.com/sfu-db/connector-x/commit/93018d2c0a21f985e212bf9811e443ccd77aea5f"
        },
        "date": 1642043894091,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06123232037228458,
            "unit": "iter/sec",
            "range": "stddev: 1.55899436916383",
            "extra": "mean: 16.331244576722383 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.078593744965035,
            "unit": "iter/sec",
            "range": "stddev: 1.199991009523649",
            "extra": "mean: 12.723658866807819 sec\nrounds: 5"
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
            "email": "wangxiaoying0369@gmail.com",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "f96e8b53fae0cec2beccad2884d7001d75425f34",
          "message": "add benchmark scripts",
          "timestamp": "2022-01-13T03:27:54Z",
          "tree_id": "de59d9dc772c2ddc328b3e512be7f7d1c22d3d68",
          "url": "https://github.com/sfu-db/connector-x/commit/f96e8b53fae0cec2beccad2884d7001d75425f34"
        },
        "date": 1642045180485,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0671632330165992,
            "unit": "iter/sec",
            "range": "stddev: 0.5536959051367845",
            "extra": "mean: 14.889098619669676 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0717454928253946,
            "unit": "iter/sec",
            "range": "stddev: 2.3050675419957445",
            "extra": "mean: 13.938157793879508 sec\nrounds: 5"
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
          "id": "928498d60d212e85efa0ad22e0b88c197fcf0bee",
          "message": "0.2.4-alpha.5: update arrow version",
          "timestamp": "2022-01-18T01:24:52Z",
          "tree_id": "0b20069fff2e882bc80726acedab72935828c7cd",
          "url": "https://github.com/sfu-db/connector-x/commit/928498d60d212e85efa0ad22e0b88c197fcf0bee"
        },
        "date": 1642470439320,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.03409077408926122,
            "unit": "iter/sec",
            "range": "stddev: 0.529287507766419",
            "extra": "mean: 29.333449495211244 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.04433061869587945,
            "unit": "iter/sec",
            "range": "stddev: 0.31902442499527883",
            "extra": "mean: 22.557772244513036 sec\nrounds: 5"
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
          "id": "acf9824bccd4fab9a8ba6acb268ee66fd9edb6d3",
          "message": "upgrade ipython dependency",
          "timestamp": "2022-01-21T21:00:25Z",
          "tree_id": "91fb7ab41bc54c22c55f73b54a12fbf8db6f865b",
          "url": "https://github.com/sfu-db/connector-x/commit/acf9824bccd4fab9a8ba6acb268ee66fd9edb6d3"
        },
        "date": 1642799634703,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06363524984116384,
            "unit": "iter/sec",
            "range": "stddev: 0.3748524542074045",
            "extra": "mean: 15.714560758322477 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07505977315539016,
            "unit": "iter/sec",
            "range": "stddev: 1.9550342354303072",
            "extra": "mean: 13.322715456783772 sec\nrounds: 5"
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
          "id": "e5af66503442808cca7c3e4c1b792490659e00e0",
          "message": "[skil ci] add technical report",
          "timestamp": "2022-01-23T22:39:07-08:00",
          "tree_id": "258683b3f1b2b391ef200311edc22fb7eb7171ce",
          "url": "https://github.com/sfu-db/connector-x/commit/e5af66503442808cca7c3e4c1b792490659e00e0"
        },
        "date": 1643007336882,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.045660569425373575,
            "unit": "iter/sec",
            "range": "stddev: 0.7754797467976594",
            "extra": "mean: 21.90073432251811 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.051706597380005256,
            "unit": "iter/sec",
            "range": "stddev: 1.6848829319839704",
            "extra": "mean: 19.33989182561636 sec\nrounds: 5"
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
          "id": "2270a63129c1f3664bcf82b8651de9c8a561e24a",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-x into main",
          "timestamp": "2022-01-25T23:59:48Z",
          "tree_id": "4a3810e0fedf9eaabbd54aa5e7c4e5050303b27d",
          "url": "https://github.com/sfu-db/connector-x/commit/2270a63129c1f3664bcf82b8651de9c8a561e24a"
        },
        "date": 1643156012704,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05932122998430276,
            "unit": "iter/sec",
            "range": "stddev: 2.341366758278201",
            "extra": "mean: 16.85737130306661 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08457928758299169,
            "unit": "iter/sec",
            "range": "stddev: 1.086793378621915",
            "extra": "mean: 11.823225621506571 sec\nrounds: 5"
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
          "id": "2270a63129c1f3664bcf82b8651de9c8a561e24a",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-x into main",
          "timestamp": "2022-01-25T23:59:48Z",
          "tree_id": "4a3810e0fedf9eaabbd54aa5e7c4e5050303b27d",
          "url": "https://github.com/sfu-db/connector-x/commit/2270a63129c1f3664bcf82b8651de9c8a561e24a"
        },
        "date": 1643222685686,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06384734821138231,
            "unit": "iter/sec",
            "range": "stddev: 1.6175402968988921",
            "extra": "mean: 15.662357607856393 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08102516290775945,
            "unit": "iter/sec",
            "range": "stddev: 1.6382438841201068",
            "extra": "mean: 12.341844978928567 sec\nrounds: 5"
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
            "email": "wangxiaoying0369@gmail.com",
            "name": "Xiaoying Wang",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "1aa3ae0dde1a2f9dc09d91423b86ae069bf11471",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-x into main",
          "timestamp": "2022-01-31T03:12:58Z",
          "tree_id": "dd577ab5e1ae84d8cf820b36f19a343e2fa6a4a0",
          "url": "https://github.com/sfu-db/connector-x/commit/1aa3ae0dde1a2f9dc09d91423b86ae069bf11471"
        },
        "date": 1643599725457,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05572675710693206,
            "unit": "iter/sec",
            "range": "stddev: 1.2860447389592176",
            "extra": "mean: 17.94470110796392 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0579759268913341,
            "unit": "iter/sec",
            "range": "stddev: 3.5644933824386036",
            "extra": "mean: 17.248538378253578 sec\nrounds: 5"
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
          "id": "77a9756cee0e21465a022850d34221571dcb15ee",
          "message": "0.2.4-alpha.6: arrow2 add conversion rule for TimestampTz arrow2",
          "timestamp": "2022-02-01T18:25:57Z",
          "tree_id": "34ae4aca0ac570c63a6f0c69c59b99d835bf85d7",
          "url": "https://github.com/sfu-db/connector-x/commit/77a9756cee0e21465a022850d34221571dcb15ee"
        },
        "date": 1643741201089,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05940079065671635,
            "unit": "iter/sec",
            "range": "stddev: 0.39964933960735427",
            "extra": "mean: 16.834792751818895 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06648909067035289,
            "unit": "iter/sec",
            "range": "stddev: 3.754076364758727",
            "extra": "mean: 15.040061308071017 sec\nrounds: 5"
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
          "id": "3cca3a341f71661eb8187b7f11b12709fd196e78",
          "message": "0.2.4-alpha.7:mysql schema support both prepared statement and limit1 for clickhouse",
          "timestamp": "2022-02-06T03:17:51Z",
          "tree_id": "bda0160ae69818a5948bf4b7b7386898906dad6e",
          "url": "https://github.com/sfu-db/connector-x/commit/3cca3a341f71661eb8187b7f11b12709fd196e78"
        },
        "date": 1644118578566,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.059815731573919693,
            "unit": "iter/sec",
            "range": "stddev: 0.5188622276705526",
            "extra": "mean: 16.718010023236275 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07491853303522988,
            "unit": "iter/sec",
            "range": "stddev: 1.2378584682642602",
            "extra": "mean: 13.34783209823072 sec\nrounds: 5"
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
          "id": "87829032d788c6fbd04f8c26775cd26d4f328719",
          "message": "0.2.4-alpha.8: hard code sql generation for oracle",
          "timestamp": "2022-02-06T04:49:09Z",
          "tree_id": "1be1c214752fe7cfc42f7fea78c517580f0c9144",
          "url": "https://github.com/sfu-db/connector-x/commit/87829032d788c6fbd04f8c26775cd26d4f328719"
        },
        "date": 1644123652558,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06545512662266664,
            "unit": "iter/sec",
            "range": "stddev: 0.39759655422282253",
            "extra": "mean: 15.277642128244043 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0777434407653826,
            "unit": "iter/sec",
            "range": "stddev: 1.9793447991738284",
            "extra": "mean: 12.862821482494473 sec\nrounds: 5"
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
          "id": "4ea9b5117293a841676ffcfa34113805f6012e84",
          "message": "0.2.4-alpha.8: hard code sql generation for oracle",
          "timestamp": "2022-02-06T04:48:26Z",
          "tree_id": "1be1c214752fe7cfc42f7fea78c517580f0c9144",
          "url": "https://github.com/sfu-db/connector-x/commit/4ea9b5117293a841676ffcfa34113805f6012e84"
        },
        "date": 1644124355426,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06303718773794736,
            "unit": "iter/sec",
            "range": "stddev: 0.43252083179081624",
            "extra": "mean: 15.863651851937174 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07903540581649361,
            "unit": "iter/sec",
            "range": "stddev: 1.2303352161203587",
            "extra": "mean: 12.652557289600372 sec\nrounds: 5"
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
          "id": "258b5c91b37c96e65a13a404f7763635c7e7adce",
          "message": "update oracle test",
          "timestamp": "2022-02-06T05:26:12Z",
          "tree_id": "e0ff66bfa36c227a945a3a5fdebab5c8f80db4b3",
          "url": "https://github.com/sfu-db/connector-x/commit/258b5c91b37c96e65a13a404f7763635c7e7adce"
        },
        "date": 1644125910280,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06178543238454548,
            "unit": "iter/sec",
            "range": "stddev: 0.6984397364525945",
            "extra": "mean: 16.18504494354129 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0665882234128036,
            "unit": "iter/sec",
            "range": "stddev: 4.271177304204963",
            "extra": "mean: 15.01767052412033 sec\nrounds: 5"
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
          "id": "e31a6ce11c710e93645b2db3057095bd0b4e5acb",
          "message": "cleanup python dependencies",
          "timestamp": "2022-02-06T05:40:41Z",
          "tree_id": "a89dba65b2f5a61518bb5ab45d2bab3096317f85",
          "url": "https://github.com/sfu-db/connector-x/commit/e31a6ce11c710e93645b2db3057095bd0b4e5acb"
        },
        "date": 1644126770679,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0621274000552011,
            "unit": "iter/sec",
            "range": "stddev: 0.3517014948402808",
            "extra": "mean: 16.095957646891474 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06209297211793022,
            "unit": "iter/sec",
            "range": "stddev: 2.3578884851643176",
            "extra": "mean: 16.104882177338006 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ab7e9cbc7a0be9bb5fce9b8f9422b5b71c23ee9a",
          "message": "Merge pull request #230 from sfu-db/oracle_stmt\n\nOracle stmt array size 1k",
          "timestamp": "2022-02-08T12:31:47-08:00",
          "tree_id": "e904dc592745ceb3a495787b5594063dde4bcc2f",
          "url": "https://github.com/sfu-db/connector-x/commit/ab7e9cbc7a0be9bb5fce9b8f9422b5b71c23ee9a"
        },
        "date": 1644353792341,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06892002298273349,
            "unit": "iter/sec",
            "range": "stddev: 0.7026562236399555",
            "extra": "mean: 14.509571481868624 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09410524916391821,
            "unit": "iter/sec",
            "range": "stddev: 0.13411471412449846",
            "extra": "mean: 10.626399790495634 sec\nrounds: 5"
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
          "id": "85dee388f117eedcbf178a021220e675305e8151",
          "message": "add owning_ref to oracle dep",
          "timestamp": "2022-02-08T21:36:47Z",
          "tree_id": "0397fd1813c6cbf3f36623200634fa7b488ece24",
          "url": "https://github.com/sfu-db/connector-x/commit/85dee388f117eedcbf178a021220e675305e8151"
        },
        "date": 1644356960051,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06192028031704394,
            "unit": "iter/sec",
            "range": "stddev: 1.885651515179848",
            "extra": "mean: 16.149797689542176 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08251957750940804,
            "unit": "iter/sec",
            "range": "stddev: 1.4811230208765018",
            "extra": "mean: 12.118336401879787 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "53464500bfcc760d833cfa2e51d1d738f5d2b2ff",
          "message": "Merge pull request #228 from sfu-db/bigquery_more\n\nbigquery add tests and docs",
          "timestamp": "2022-02-09T10:19:29-08:00",
          "tree_id": "f83879b115fb0880594db17278913de14176649f",
          "url": "https://github.com/sfu-db/connector-x/commit/53464500bfcc760d833cfa2e51d1d738f5d2b2ff"
        },
        "date": 1644431649846,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.061165624398727435,
            "unit": "iter/sec",
            "range": "stddev: 0.5986088044330616",
            "extra": "mean: 16.349052426591516 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.053016395752348185,
            "unit": "iter/sec",
            "range": "stddev: 4.165416679804586",
            "extra": "mean: 18.86208946891129 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e79490d25da1d2802cac1a218f9a3958c652435a",
          "message": "Merge pull request #234 from glennpierce/main\n\nUpdate to use polars 0.19.1 and arrow2 0.9",
          "timestamp": "2022-02-15T15:52:03-08:00",
          "tree_id": "038c5cf9fb80c2eab3f9430567a8fb3908f619a1",
          "url": "https://github.com/sfu-db/connector-x/commit/e79490d25da1d2802cac1a218f9a3958c652435a"
        },
        "date": 1644969921918,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05232794441994736,
            "unit": "iter/sec",
            "range": "stddev: 3.2258842015531553",
            "extra": "mean: 19.110248091816903 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07115280412422666,
            "unit": "iter/sec",
            "range": "stddev: 3.477015195854756",
            "extra": "mean: 14.054259875044227 sec\nrounds: 5"
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
          "id": "8fb2f23db6cd465ea01ad629c7d41e2981518071",
          "message": "0.2.4-alpha.9: upgrade arrow2 to 0.9",
          "timestamp": "2022-02-16T00:35:41Z",
          "tree_id": "cb37f1e0b0757e7a7d42a569aa422c7ddddc81c0",
          "url": "https://github.com/sfu-db/connector-x/commit/8fb2f23db6cd465ea01ad629c7d41e2981518071"
        },
        "date": 1644972470710,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.060512697668455045,
            "unit": "iter/sec",
            "range": "stddev: 1.3435193903100864",
            "extra": "mean: 16.525457276403905 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08075587393425993,
            "unit": "iter/sec",
            "range": "stddev: 0.8668937423779827",
            "extra": "mean: 12.383000161871314 sec\nrounds: 5"
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
          "id": "edcdf3b7e6ddf69b8618be3fbf4ecb717f2d397e",
          "message": "merge with main",
          "timestamp": "2022-02-16T00:41:43Z",
          "tree_id": "7664cda747f4ee83789c6272b402f5afda1ef672",
          "url": "https://github.com/sfu-db/connector-x/commit/edcdf3b7e6ddf69b8618be3fbf4ecb717f2d397e"
        },
        "date": 1644973249566,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05957462198858228,
            "unit": "iter/sec",
            "range": "stddev: 1.0045926163701362",
            "extra": "mean: 16.78567092195153 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0651015845029898,
            "unit": "iter/sec",
            "range": "stddev: 4.625882990848235",
            "extra": "mean: 15.360609233006835 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "e33e0255156e7228c4a3632b59d4aa03cab837be",
          "message": "maturin version update",
          "timestamp": "2022-02-23T06:12:32Z",
          "tree_id": "8ce6e0f8e7b808c8730e39dbb0b0314a1acf1998",
          "url": "https://github.com/sfu-db/connector-x/commit/e33e0255156e7228c4a3632b59d4aa03cab837be"
        },
        "date": 1645597536321,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06545357447346434,
            "unit": "iter/sec",
            "range": "stddev: 0.6015799614675442",
            "extra": "mean: 15.278004418313504 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09382121258795643,
            "unit": "iter/sec",
            "range": "stddev: 0.8261236420754562",
            "extra": "mean: 10.658570406585932 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "d6b2299542c0275a04acc75e97e3f422a7a80f9d",
          "message": "0.2.4-alpha.10: switch to maturin for wheel building",
          "timestamp": "2022-02-23T06:53:16Z",
          "tree_id": "8a6ca6df628c256626749a954ca8a8ef91d4f284",
          "url": "https://github.com/sfu-db/connector-x/commit/d6b2299542c0275a04acc75e97e3f422a7a80f9d"
        },
        "date": 1645599829794,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06857529800976005,
            "unit": "iter/sec",
            "range": "stddev: 0.91938798066769",
            "extra": "mean: 14.582510452345014 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07283743612244797,
            "unit": "iter/sec",
            "range": "stddev: 2.381005933999852",
            "extra": "mean: 13.729203734174371 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "e23f790e99e5f542c396c8618aa96f461c408401",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-02-24T04:59:12Z",
          "tree_id": "2e5a6f8be36779a547e43f08eb3a62b65031ecec",
          "url": "https://github.com/sfu-db/connector-x/commit/e23f790e99e5f542c396c8618aa96f461c408401"
        },
        "date": 1645679410311,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0698295790132649,
            "unit": "iter/sec",
            "range": "stddev: 0.22412866408955998",
            "extra": "mean: 14.320578959956766 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07896833528578572,
            "unit": "iter/sec",
            "range": "stddev: 2.6462769519652505",
            "extra": "mean: 12.663303542882204 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "5f6cdeed952780a45cc56bfdda990034c89dfa2c",
          "message": "update",
          "timestamp": "2022-02-25T07:05:15Z",
          "tree_id": "515c93229736f003d16abeff4c91debd1c81ef7c",
          "url": "https://github.com/sfu-db/connector-x/commit/5f6cdeed952780a45cc56bfdda990034c89dfa2c"
        },
        "date": 1645773332765,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.060746506529492976,
            "unit": "iter/sec",
            "range": "stddev: 0.7790799289628753",
            "extra": "mean: 16.461851999908685 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08220059607954885,
            "unit": "iter/sec",
            "range": "stddev: 2.1436016661151003",
            "extra": "mean: 12.165361903607845 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "e9098fa082b35a3db8dabc9232b986933e4bcc74",
          "message": "update",
          "timestamp": "2022-02-25T07:33:23Z",
          "tree_id": "eb80f5d8fed262af16535837c751ddc6afbdbf71",
          "url": "https://github.com/sfu-db/connector-x/commit/e9098fa082b35a3db8dabc9232b986933e4bcc74"
        },
        "date": 1645775939712,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.013659926377797005,
            "unit": "iter/sec",
            "range": "stddev: 2.7605752639219334",
            "extra": "mean: 73.20683672390878 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.01063519906296745,
            "unit": "iter/sec",
            "range": "stddev: 0.5965993329712105",
            "extra": "mean: 94.02738905772567 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "22cf20616738b11009606bff4961c2feca721de2",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-02-25T07:33:33Z",
          "tree_id": "d1dd0c0c42636c28d6a192cf8737fc723ffb6c83",
          "url": "https://github.com/sfu-db/connector-x/commit/22cf20616738b11009606bff4961c2feca721de2"
        },
        "date": 1645777456693,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.014062134017365646,
            "unit": "iter/sec",
            "range": "stddev: 0.5165182419494998",
            "extra": "mean: 71.11296185664833 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.010444380285840906,
            "unit": "iter/sec",
            "range": "stddev: 0.6724690180547902",
            "extra": "mean: 95.74526899941266 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "a719a535a26cdfd6abe5f4b6d72ef315d1e2800d",
          "message": "update",
          "timestamp": "2022-02-25T18:23:16Z",
          "tree_id": "9065eba3071d2130338d0042ba6cede1de0ea6fd",
          "url": "https://github.com/sfu-db/connector-x/commit/a719a535a26cdfd6abe5f4b6d72ef315d1e2800d"
        },
        "date": 1645814834532,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.015177315274073467,
            "unit": "iter/sec",
            "range": "stddev: 3.6029326293467223",
            "extra": "mean: 65.887805711478 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.01182824449032273,
            "unit": "iter/sec",
            "range": "stddev: 4.58315491736793",
            "extra": "mean: 84.54339955672621 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "fd7a61d910110310d521df66e90f0465942ccd2e",
          "message": "update: manylinux:2014 for build arm wheel on linux",
          "timestamp": "2022-02-26T02:01:09Z",
          "tree_id": "fd122b33ce52e750afa3390644b76f01e80ed4c3",
          "url": "https://github.com/sfu-db/connector-x/commit/fd7a61d910110310d521df66e90f0465942ccd2e"
        },
        "date": 1645842342804,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.015456462865382128,
            "unit": "iter/sec",
            "range": "stddev: 3.315480536473377",
            "extra": "mean: 64.69785543493927 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.010782103394930189,
            "unit": "iter/sec",
            "range": "stddev: 4.4735908715486765",
            "extra": "mean: 92.74628181271255 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "ee42fe2ac7e851a9b340f5a55414e647abfb26b2",
          "message": "remove arm for linux for now",
          "timestamp": "2022-02-26T05:15:14Z",
          "tree_id": "501b4341ec2b3ef6bfca3ed80bfac126176986d6",
          "url": "https://github.com/sfu-db/connector-x/commit/ee42fe2ac7e851a9b340f5a55414e647abfb26b2"
        },
        "date": 1645853885688,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.015712871606593314,
            "unit": "iter/sec",
            "range": "stddev: 1.1263457700213262",
            "extra": "mean: 63.642090703547 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.011771756895091445,
            "unit": "iter/sec",
            "range": "stddev: 0.7224582660028104",
            "extra": "mean: 84.94908694699407 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "ebeb74eadc9af1d4b78c101d34e74d350f4e41d2",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-02-26T05:15:23Z",
          "tree_id": "08019b67de1ccffa452c4790174c287db3f70df4",
          "url": "https://github.com/sfu-db/connector-x/commit/ebeb74eadc9af1d4b78c101d34e74d350f4e41d2"
        },
        "date": 1645855208982,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.016167673139017496,
            "unit": "iter/sec",
            "range": "stddev: 0.926546213248967",
            "extra": "mean: 61.851819454878566 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.01227491016077129,
            "unit": "iter/sec",
            "range": "stddev: 1.4814886586372298",
            "extra": "mean: 81.46699135899544 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "b8fd620bac67055e12ca47a5e64ec9d4aa0ac96e",
          "message": "update",
          "timestamp": "2022-02-27T07:56:34Z",
          "tree_id": "c5372a1f7a660f04186f301babd15468bc066b34",
          "url": "https://github.com/sfu-db/connector-x/commit/b8fd620bac67055e12ca47a5e64ec9d4aa0ac96e"
        },
        "date": 1646170471889,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.01855649382385782,
            "unit": "iter/sec",
            "range": "stddev: 0.6431106372675102",
            "extra": "mean: 53.88949062749744 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.01385355790479345,
            "unit": "iter/sec",
            "range": "stddev: 0.9367761464755265",
            "extra": "mean: 72.18362292721868 sec\nrounds: 5"
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
          "id": "7593517b6f005bfd132c5986fb2461dc80d89b05",
          "message": "0.2.4",
          "timestamp": "2022-03-03T05:23:30Z",
          "tree_id": "7f852f755939336b8b476a9e28a95b27ddf42291",
          "url": "https://github.com/sfu-db/connector-x/commit/7593517b6f005bfd132c5986fb2461dc80d89b05"
        },
        "date": 1646285866543,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.058856529563389645,
            "unit": "iter/sec",
            "range": "stddev: 2.4333936570866017",
            "extra": "mean: 16.990468303486704 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06684467069305818,
            "unit": "iter/sec",
            "range": "stddev: 4.248668421533248",
            "extra": "mean: 14.960055747628212 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1ae74ff44fb94e923c63a8949f1f7fb342e9a05b",
          "message": "Merge pull request #247 from Wukkkinz-0725/documentation\n\nAdd documentation",
          "timestamp": "2022-03-03T12:21:28-08:00",
          "tree_id": "1e8b9218f8828de6d3b292e1e663dffc4fb3ff7f",
          "url": "https://github.com/sfu-db/connector-x/commit/1ae74ff44fb94e923c63a8949f1f7fb342e9a05b"
        },
        "date": 1646339859379,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.059594584772639216,
            "unit": "iter/sec",
            "range": "stddev: 0.9663998582967307",
            "extra": "mean: 16.780048117041588 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0657185836353084,
            "unit": "iter/sec",
            "range": "stddev: 1.6967019517203643",
            "extra": "mean: 15.216396104171872 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "60677420+Wukkkinz-0725@users.noreply.github.com",
            "name": "Wukkkinz-0725",
            "username": "Wukkkinz-0725"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "576ac4e5bb743b97dfda6d0777662e33b2ef334c",
          "message": "Update doc.yml",
          "timestamp": "2022-03-03T13:18:35-08:00",
          "tree_id": "a9c767de68d29bebb50ae814d21742a3f935b6db",
          "url": "https://github.com/sfu-db/connector-x/commit/576ac4e5bb743b97dfda6d0777662e33b2ef334c"
        },
        "date": 1646343122205,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05776945080974188,
            "unit": "iter/sec",
            "range": "stddev: 0.551627389131672",
            "extra": "mean: 17.310187062248588 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05325128295800787,
            "unit": "iter/sec",
            "range": "stddev: 2.088487565958039",
            "extra": "mean: 18.778890281170607 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f01631cebc84918071f61f997f533b89b9135050",
          "message": "Merge pull request #249 from Wukkkinz-0725/update_doc\n\nUpdate doc",
          "timestamp": "2022-03-03T15:14:26-08:00",
          "tree_id": "c0fd5c29f461aba35f1ae3ac18f46c6e78beec4d",
          "url": "https://github.com/sfu-db/connector-x/commit/f01631cebc84918071f61f997f533b89b9135050"
        },
        "date": 1646351170663,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0583438942059501,
            "unit": "iter/sec",
            "range": "stddev: 0.47830217196397184",
            "extra": "mean: 17.139754101261495 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05167424267338567,
            "unit": "iter/sec",
            "range": "stddev: 4.154119649323607",
            "extra": "mean: 19.352001079544426 sec\nrounds: 5"
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
          "id": "c6b11b76cfb1756f35143c8cbffbb5890ed23a70",
          "message": "add urlencoding to sqlite dep",
          "timestamp": "2022-03-05T01:39:03Z",
          "tree_id": "4c7703dde9265043be5924a87311c43b57fc5f40",
          "url": "https://github.com/sfu-db/connector-x/commit/c6b11b76cfb1756f35143c8cbffbb5890ed23a70"
        },
        "date": 1646445221756,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05421017416507087,
            "unit": "iter/sec",
            "range": "stddev: 1.9290322182112192",
            "extra": "mean: 18.446721771359442 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05278578875526487,
            "unit": "iter/sec",
            "range": "stddev: 3.814795010067674",
            "extra": "mean: 18.944492894411088 sec\nrounds: 5"
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
          "id": "b6a609c09965aa1bc75d285b30915bca39a93414",
          "message": "add urlencoding to sqlite dep\n\nupdate",
          "timestamp": "2022-03-05T02:32:28Z",
          "tree_id": "020a0df1f2532c42c9201bb7eea8477b54dd3bac",
          "url": "https://github.com/sfu-db/connector-x/commit/b6a609c09965aa1bc75d285b30915bca39a93414"
        },
        "date": 1646448501031,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.04099548702997999,
            "unit": "iter/sec",
            "range": "stddev: 5.076170334481313",
            "extra": "mean: 24.39292889162898 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05867268281693137,
            "unit": "iter/sec",
            "range": "stddev: 2.8328106213267015",
            "extra": "mean: 17.043706747144462 sec\nrounds: 5"
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
          "id": "1986156ca0c79e052fb44d6ded986dcd8cac0bca",
          "message": "update doc",
          "timestamp": "2022-03-07T05:12:13Z",
          "tree_id": "53a813c9131500a09e08e1972995fd884267628a",
          "url": "https://github.com/sfu-db/connector-x/commit/1986156ca0c79e052fb44d6ded986dcd8cac0bca"
        },
        "date": 1646630941100,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.045511226130691924,
            "unit": "iter/sec",
            "range": "stddev: 5.398061451081462",
            "extra": "mean: 21.972600718960166 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.04832796752621058,
            "unit": "iter/sec",
            "range": "stddev: 5.180538419884825",
            "extra": "mean: 20.691952324658633 sec\nrounds: 5"
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
          "id": "233e11de5753f9ca5be8f6ef1749cd04e77b905b",
          "message": "update doc",
          "timestamp": "2022-03-09T06:04:01Z",
          "tree_id": "fbe48f38241d6ca8b5facffef2f6b3120242de3b",
          "url": "https://github.com/sfu-db/connector-x/commit/233e11de5753f9ca5be8f6ef1749cd04e77b905b"
        },
        "date": 1646806514254,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06473379365928668,
            "unit": "iter/sec",
            "range": "stddev: 0.29842278284178575",
            "extra": "mean: 15.447881909459829 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07665796100434302,
            "unit": "iter/sec",
            "range": "stddev: 1.9041997872098242",
            "extra": "mean: 13.044959543645382 sec\nrounds: 5"
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
          "id": "e3581b6c0a8f3351563fb598b4d74beb05ce6abf",
          "message": "add partition_sql test",
          "timestamp": "2022-03-09T08:18:40Z",
          "tree_id": "d4da6a3b1befb73f2dc0985339204b885486f546",
          "url": "https://github.com/sfu-db/connector-x/commit/e3581b6c0a8f3351563fb598b4d74beb05ce6abf"
        },
        "date": 1646814565043,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06893341811127593,
            "unit": "iter/sec",
            "range": "stddev: 0.5633868294139193",
            "extra": "mean: 14.506751984730363 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0797926517615675,
            "unit": "iter/sec",
            "range": "stddev: 1.2073624646740395",
            "extra": "mean: 12.532482351735235 sec\nrounds: 5"
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
          "id": "4dcf8c0823a0e287918baf2c1bb38a5607205f4c",
          "message": "skip auditwheel",
          "timestamp": "2022-03-10T22:07:48Z",
          "tree_id": "28641200363b4a6a34d8d6334ed3e308be4efc0b",
          "url": "https://github.com/sfu-db/connector-x/commit/4dcf8c0823a0e287918baf2c1bb38a5607205f4c"
        },
        "date": 1646951432771,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.017973862068958166,
            "unit": "iter/sec",
            "range": "stddev: 1.522016523039028",
            "extra": "mean: 55.636345497891305 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.013588927753610723,
            "unit": "iter/sec",
            "range": "stddev: 0.8950616536807655",
            "extra": "mean: 73.58932346478105 sec\nrounds: 5"
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
          "id": "3c72184ce0f09746b11fc6f896db4e86e905d746",
          "message": "vendor openssl",
          "timestamp": "2022-03-11T00:54:47Z",
          "tree_id": "1a296ba3ebfadf2b0b943539995288bfd50a8bf6",
          "url": "https://github.com/sfu-db/connector-x/commit/3c72184ce0f09746b11fc6f896db4e86e905d746"
        },
        "date": 1646961347890,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.01706587632121576,
            "unit": "iter/sec",
            "range": "stddev: 8.769335363214157",
            "extra": "mean: 58.596463561430575 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.013068569674510875,
            "unit": "iter/sec",
            "range": "stddev: 2.002927124333289",
            "extra": "mean: 76.51946807540953 sec\nrounds: 5"
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
          "id": "1ed838da2c471194e96f9172b8f2dfb30bb5ae1a",
          "message": "auditwheel",
          "timestamp": "2022-03-11T01:00:06Z",
          "tree_id": "5551ce23a03dc1315213651fa09077cb131b5442",
          "url": "https://github.com/sfu-db/connector-x/commit/1ed838da2c471194e96f9172b8f2dfb30bb5ae1a"
        },
        "date": 1646962554108,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.018471999910726844,
            "unit": "iter/sec",
            "range": "stddev: 0.8054426683879586",
            "extra": "mean: 54.135989867523314 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.013398132020267977,
            "unit": "iter/sec",
            "range": "stddev: 1.1530928077770903",
            "extra": "mean: 74.63727021701634 sec\nrounds: 5"
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
          "id": "c8284631966b92584c286e468875ab18c224a476",
          "message": "add linux arm and apple_arm",
          "timestamp": "2022-03-11T01:39:23Z",
          "tree_id": "9ad40b42a0b9d6b60eb7433fdbaf31873ac7a881",
          "url": "https://github.com/sfu-db/connector-x/commit/c8284631966b92584c286e468875ab18c224a476"
        },
        "date": 1646963966875,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.018453236349973205,
            "unit": "iter/sec",
            "range": "stddev: 0.7229941355456787",
            "extra": "mean: 54.19103625155985 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0134619416668967,
            "unit": "iter/sec",
            "range": "stddev: 0.6310094735825149",
            "extra": "mean: 74.28348931707441 sec\nrounds: 5"
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
          "id": "122674e5890dcf8c8fe02c73ac8601fa34ac578c",
          "message": "fix ci",
          "timestamp": "2022-03-11T02:31:08Z",
          "tree_id": "897999620bf75d5b4b80a92fc7f74450c396307f",
          "url": "https://github.com/sfu-db/connector-x/commit/122674e5890dcf8c8fe02c73ac8601fa34ac578c"
        },
        "date": 1646967063309,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.018354326689875567,
            "unit": "iter/sec",
            "range": "stddev: 0.3816592705329894",
            "extra": "mean: 54.483066412433985 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.013776442030800529,
            "unit": "iter/sec",
            "range": "stddev: 1.001404356418756",
            "extra": "mean: 72.58768249191344 sec\nrounds: 5"
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
          "id": "4ea37283bac3509310310600640b20a9df8aebdd",
          "message": "test build",
          "timestamp": "2022-03-11T02:38:48Z",
          "tree_id": "b3bc1ce6518b7107ad948c45135af13b63aec5ef",
          "url": "https://github.com/sfu-db/connector-x/commit/4ea37283bac3509310310600640b20a9df8aebdd"
        },
        "date": 1646968263877,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.018293765693298828,
            "unit": "iter/sec",
            "range": "stddev: 0.33199100828047845",
            "extra": "mean: 54.66343107074499 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.013532550125618823,
            "unit": "iter/sec",
            "range": "stddev: 0.6849512702819773",
            "extra": "mean: 73.89590215571225 sec\nrounds: 5"
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
          "id": "f6ca5d9d02aec6e72fb6fb6b414bb1d542f932ff",
          "message": "update readme and doc",
          "timestamp": "2022-03-11T05:36:40Z",
          "tree_id": "cff9c6ef429e1b32fc6b1b604bc58afdfd70df27",
          "url": "https://github.com/sfu-db/connector-x/commit/f6ca5d9d02aec6e72fb6fb6b414bb1d542f932ff"
        },
        "date": 1646977650709,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06878646154430447,
            "unit": "iter/sec",
            "range": "stddev: 0.4851744606443408",
            "extra": "mean: 14.537744456529618 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07222948847761418,
            "unit": "iter/sec",
            "range": "stddev: 2.9441844110157813",
            "extra": "mean: 13.84476092904806 sec\nrounds: 5"
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
          "id": "aac26b2a964c1a4de6a8e44648597aa11b5e57ac",
          "message": "do not build arm linux",
          "timestamp": "2022-03-11T21:39:46Z",
          "tree_id": "e02acb63992867a44eff89f075607132859fabee",
          "url": "https://github.com/sfu-db/connector-x/commit/aac26b2a964c1a4de6a8e44648597aa11b5e57ac"
        },
        "date": 1647036051191,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.018392291483186773,
            "unit": "iter/sec",
            "range": "stddev: 1.1543868046815093",
            "extra": "mean: 54.37060416936875 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.013432390772991673,
            "unit": "iter/sec",
            "range": "stddev: 1.39795611125093",
            "extra": "mean: 74.44691097065807 sec\nrounds: 5"
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
          "id": "68ab0f5e83ec696d23ac816fdedbcaed01b31992",
          "message": "fix verify",
          "timestamp": "2022-03-12T02:54:58Z",
          "tree_id": "6f6c375d63fe1a59925d518854ff40af5101131d",
          "url": "https://github.com/sfu-db/connector-x/commit/68ab0f5e83ec696d23ac816fdedbcaed01b31992"
        },
        "date": 1647054915370,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.01864662995301571,
            "unit": "iter/sec",
            "range": "stddev: 0.6833093412600418",
            "extra": "mean: 53.62899368517101 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.013739992513696604,
            "unit": "iter/sec",
            "range": "stddev: 1.1494352119830147",
            "extra": "mean: 72.78024343922735 sec\nrounds: 5"
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
          "id": "2f0388b01c110a721f4586a710ddd62798db65e1",
          "message": "fix ci",
          "timestamp": "2022-03-12T04:37:32Z",
          "tree_id": "f7399a1d044ecbaad4af87cc7cadaa5816eeefa8",
          "url": "https://github.com/sfu-db/connector-x/commit/2f0388b01c110a721f4586a710ddd62798db65e1"
        },
        "date": 1647061042977,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.018605099864870356,
            "unit": "iter/sec",
            "range": "stddev: 0.8387328845226597",
            "extra": "mean: 53.74870370291173 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.013853781398916316,
            "unit": "iter/sec",
            "range": "stddev: 0.28772317373542705",
            "extra": "mean: 72.18245843537152 sec\nrounds: 5"
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
          "id": "b855ef3daa78126271532f8ef9a62c3d1954d693",
          "message": "fix ci",
          "timestamp": "2022-03-12T06:26:03Z",
          "tree_id": "48dd91adcb8f2146d031e443af971be3fa063b21",
          "url": "https://github.com/sfu-db/connector-x/commit/b855ef3daa78126271532f8ef9a62c3d1954d693"
        },
        "date": 1647067577101,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.018501451358548075,
            "unit": "iter/sec",
            "range": "stddev: 0.23026823529350599",
            "extra": "mean: 54.04981374815107 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.01356174132442572,
            "unit": "iter/sec",
            "range": "stddev: 0.20519386213567625",
            "extra": "mean: 73.73684367500246 sec\nrounds: 5"
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
          "id": "75bf4dbf448f25c66efb67c9c7000d7fbbcbd99e",
          "message": "Merge branch 'main' into release",
          "timestamp": "2022-03-05T03:10:34Z",
          "tree_id": "020a0df1f2532c42c9201bb7eea8477b54dd3bac",
          "url": "https://github.com/sfu-db/connector-x/commit/75bf4dbf448f25c66efb67c9c7000d7fbbcbd99e"
        },
        "date": 1647100631324,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06469711049718,
            "unit": "iter/sec",
            "range": "stddev: 0.2018756789958004",
            "extra": "mean: 15.456640834733843 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07952919927416244,
            "unit": "iter/sec",
            "range": "stddev: 2.7459468897383816",
            "extra": "mean: 12.573998092859984 sec\nrounds: 5"
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
          "id": "75bf4dbf448f25c66efb67c9c7000d7fbbcbd99e",
          "message": "Merge branch 'main' into release",
          "timestamp": "2022-03-05T03:10:34Z",
          "tree_id": "020a0df1f2532c42c9201bb7eea8477b54dd3bac",
          "url": "https://github.com/sfu-db/connector-x/commit/75bf4dbf448f25c66efb67c9c7000d7fbbcbd99e"
        },
        "date": 1647111100264,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06976452136379302,
            "unit": "iter/sec",
            "range": "stddev: 0.3207472329047319",
            "extra": "mean: 14.33393335826695 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08053622736718791,
            "unit": "iter/sec",
            "range": "stddev: 2.7141783591226707",
            "extra": "mean: 12.416772335767746 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "64413dfe833ac606cfe98cf1d9114eddeb3d5367",
          "message": "0.2.5-alpha.2",
          "timestamp": "2022-03-12T19:42:14Z",
          "tree_id": "1c43f34e1599b8704af35f8a3f7f5b527e616e53",
          "url": "https://github.com/sfu-db/connector-x/commit/64413dfe833ac606cfe98cf1d9114eddeb3d5367"
        },
        "date": 1647115357033,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.017677405244501468,
            "unit": "iter/sec",
            "range": "stddev: 1.0627845501683442",
            "extra": "mean: 56.569388220086694 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.013226130300990999,
            "unit": "iter/sec",
            "range": "stddev: 0.8944125708695515",
            "extra": "mean: 75.60790474936366 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "6c8558cfd159542d06f691fb45fbf4c50e317e8c",
          "message": "update doc",
          "timestamp": "2022-03-12T20:48:49Z",
          "tree_id": "ee3ed3bc8223cbffcb4f1af0bcf7b68fcaf7b2ff",
          "url": "https://github.com/sfu-db/connector-x/commit/6c8558cfd159542d06f691fb45fbf4c50e317e8c"
        },
        "date": 1647118643543,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07016669701053893,
            "unit": "iter/sec",
            "range": "stddev: 0.2908064450093811",
            "extra": "mean: 14.251775309443474 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08975057224071571,
            "unit": "iter/sec",
            "range": "stddev: 3.8289149306859995",
            "extra": "mean: 11.14199024066329 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "5ca2e750fba568c164a59d63cec2aa6ea1d56eeb",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-03-12T21:59:18Z",
          "tree_id": "ee3ed3bc8223cbffcb4f1af0bcf7b68fcaf7b2ff",
          "url": "https://github.com/sfu-db/connector-x/commit/5ca2e750fba568c164a59d63cec2aa6ea1d56eeb"
        },
        "date": 1647122899091,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06464953265403676,
            "unit": "iter/sec",
            "range": "stddev: 0.43705215246082896",
            "extra": "mean: 15.46801591515541 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.11687859947627138,
            "unit": "iter/sec",
            "range": "stddev: 0.46733148399034474",
            "extra": "mean: 8.55588623136282 sec\nrounds: 5"
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
          "id": "f9df768457b90eeed36a3915fa2b2080f9e10c36",
          "message": "fix upload",
          "timestamp": "2022-03-13T03:09:17Z",
          "tree_id": "beb9db9c570a3a9ead36ed5058f96c348b76e997",
          "url": "https://github.com/sfu-db/connector-x/commit/f9df768457b90eeed36a3915fa2b2080f9e10c36"
        },
        "date": 1647141532870,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06611409646038947,
            "unit": "iter/sec",
            "range": "stddev: 0.8360941107982015",
            "extra": "mean: 15.125367410853505 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06345685207499115,
            "unit": "iter/sec",
            "range": "stddev: 4.4355278646996865",
            "extra": "mean: 15.758739478886127 sec\nrounds: 5"
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
          "id": "d770a89eea2494a311106493befbfcc7fed9927c",
          "message": "fix python package name",
          "timestamp": "2022-03-13T03:13:03Z",
          "tree_id": "9db15933336a5b10c898e9dc78e77fe961470645",
          "url": "https://github.com/sfu-db/connector-x/commit/d770a89eea2494a311106493befbfcc7fed9927c"
        },
        "date": 1647142042497,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0699346322408989,
            "unit": "iter/sec",
            "range": "stddev: 0.35721748255873315",
            "extra": "mean: 14.299067113921046 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0851774379225154,
            "unit": "iter/sec",
            "range": "stddev: 3.629220077620966",
            "extra": "mean: 11.74019816033542 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f6854d16d831a1113dbde90e9a3fd6c9317f752b",
          "message": "Merge pull request #254 from sfu-db/postgres_auth\n\nPostgres auth",
          "timestamp": "2022-03-19T09:27:16-07:00",
          "tree_id": "6e4a0d03f211d7f22e751c22d65fc7d7231d07e2",
          "url": "https://github.com/sfu-db/connector-x/commit/f6854d16d831a1113dbde90e9a3fd6c9317f752b"
        },
        "date": 1647707813654,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06478602181146967,
            "unit": "iter/sec",
            "range": "stddev: 0.9172257681331225",
            "extra": "mean: 15.435428384691477 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06485026653539049,
            "unit": "iter/sec",
            "range": "stddev: 2.7068955377432546",
            "extra": "mean: 15.42013708539307 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "7f07c02a8e4a0dba03dca77ffb57d92a02893fb1",
          "message": "expose get_meta interface",
          "timestamp": "2022-03-24T05:40:12Z",
          "tree_id": "3f235910d30e7a85ccacf3ab9b73be56dc0f1e1c",
          "url": "https://github.com/sfu-db/connector-x/commit/7f07c02a8e4a0dba03dca77ffb57d92a02893fb1"
        },
        "date": 1648101509229,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.047810512247923515,
            "unit": "iter/sec",
            "range": "stddev: 3.7449228281013296",
            "extra": "mean: 20.915902235358953 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0509467925142577,
            "unit": "iter/sec",
            "range": "stddev: 3.1580833172956067",
            "extra": "mean: 19.628321051225065 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9ca85e94b1d319eb374f556ae95acf7336582f05",
          "message": "Merge pull request #255 from sfu-db/oracle_invalid_digit_issue\n\nfix oracle round function issue",
          "timestamp": "2022-03-24T16:45:18-07:00",
          "tree_id": "4c85e504c0d5e04632435f9b1c20d9e019ccb98d",
          "url": "https://github.com/sfu-db/connector-x/commit/9ca85e94b1d319eb374f556ae95acf7336582f05"
        },
        "date": 1648166311205,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05814848584463255,
            "unit": "iter/sec",
            "range": "stddev: 0.21523568225611867",
            "extra": "mean: 17.197352355346084 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06051874145165014,
            "unit": "iter/sec",
            "range": "stddev: 3.455719856371344",
            "extra": "mean: 16.52380694001913 sec\nrounds: 5"
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
          "id": "2e1ed6afa359065ecd5d8fbecab1682beb9665e8",
          "message": "0.2.5-alpha.3: oracle bug fix + postgres client auth",
          "timestamp": "2022-03-25T00:03:25Z",
          "tree_id": "e4079a1788874d1eb7686374539fe37a88fd6271",
          "url": "https://github.com/sfu-db/connector-x/commit/2e1ed6afa359065ecd5d8fbecab1682beb9665e8"
        },
        "date": 1648167375763,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05645740401195163,
            "unit": "iter/sec",
            "range": "stddev: 0.16877093992308173",
            "extra": "mean: 17.71246867440641 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07798928577478563,
            "unit": "iter/sec",
            "range": "stddev: 0.37825892871521377",
            "extra": "mean: 12.822274111956357 sec\nrounds: 5"
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
          "id": "e987e305d9d06ba435a60c8f6dc5f03b59080880",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-03-25T00:13:41Z",
          "tree_id": "8c7c4382af1b5076ef22547146d5834360a820a5",
          "url": "https://github.com/sfu-db/connector-x/commit/e987e305d9d06ba435a60c8f6dc5f03b59080880"
        },
        "date": 1648168145469,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.054967119530410294,
            "unit": "iter/sec",
            "range": "stddev: 0.8050835884373935",
            "extra": "mean: 18.192694260552525 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07125551492636063,
            "unit": "iter/sec",
            "range": "stddev: 1.9105168891134072",
            "extra": "mean: 14.034001452848315 sec\nrounds: 5"
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
          "id": "0012839f281e196482130a3ea0daf9143b556089",
          "message": "fix release script",
          "timestamp": "2022-03-25T01:12:05Z",
          "tree_id": "d01ef85db13a7f13a5ff0da6c080bf6e4f95d031",
          "url": "https://github.com/sfu-db/connector-x/commit/0012839f281e196482130a3ea0daf9143b556089"
        },
        "date": 1648171713453,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.017675717882072096,
            "unit": "iter/sec",
            "range": "stddev: 56.693072488446894",
            "extra": "mean: 56.5747884567827 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07055099159065996,
            "unit": "iter/sec",
            "range": "stddev: 0.921312970467408",
            "extra": "mean: 14.174145216867327 sec\nrounds: 5"
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
          "id": "8d88eb1f732efc22173cc6aefe1eb057784942e2",
          "message": "fix release script",
          "timestamp": "2022-03-25T01:13:08Z",
          "tree_id": "33d7c2264fc3287db39f3ce6d83b628149d1a607",
          "url": "https://github.com/sfu-db/connector-x/commit/8d88eb1f732efc22173cc6aefe1eb057784942e2"
        },
        "date": 1648172476422,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05686353610227907,
            "unit": "iter/sec",
            "range": "stddev: 0.1605504084716676",
            "extra": "mean: 17.585962262377144 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.061817520754268815,
            "unit": "iter/sec",
            "range": "stddev: 3.3154436165991537",
            "extra": "mean: 16.176643576100467 sec\nrounds: 5"
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
          "id": "93974cd84d599e2258c59fa62d30a3be47b66c73",
          "message": "update arrow to 9.0.0",
          "timestamp": "2022-03-28T04:16:16Z",
          "tree_id": "ba021e92ea967ba1adaa933d42a5f0e15111704a",
          "url": "https://github.com/sfu-db/connector-x/commit/93974cd84d599e2258c59fa62d30a3be47b66c73"
        },
        "date": 1648442010415,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.04478565970523373,
            "unit": "iter/sec",
            "range": "stddev: 1.6140157344065968",
            "extra": "mean: 22.328575856238604 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.051982026214149324,
            "unit": "iter/sec",
            "range": "stddev: 4.23770848126805",
            "extra": "mean: 19.237418639287352 sec\nrounds: 5"
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
          "id": "2f690f78e52348d1d86b02b7ecc05dbdbe32a0c0",
          "message": "update conn parse",
          "timestamp": "2022-03-28T04:38:35Z",
          "tree_id": "cd448a47b7f17ecc3f57a391c8260d3a499a9a48",
          "url": "https://github.com/sfu-db/connector-x/commit/2f690f78e52348d1d86b02b7ecc05dbdbe32a0c0"
        },
        "date": 1648443261190,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.03192442911309163,
            "unit": "iter/sec",
            "range": "stddev: 8.971490180021613",
            "extra": "mean: 31.323974391445518 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06605504751687062,
            "unit": "iter/sec",
            "range": "stddev: 0.3341823295580913",
            "extra": "mean: 15.138888511806726 sec\nrounds: 5"
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
          "id": "1a31cc72e4f759e93c1bdf2ff28c5443394b44c3",
          "message": "0.2.5",
          "timestamp": "2022-03-29T03:31:44Z",
          "tree_id": "0ad72e6ab4f71c32e5c325e90439b3ce441a9220",
          "url": "https://github.com/sfu-db/connector-x/commit/1a31cc72e4f759e93c1bdf2ff28c5443394b44c3"
        },
        "date": 1648525574641,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05221459471305324,
            "unit": "iter/sec",
            "range": "stddev: 0.2790688059080033",
            "extra": "mean: 19.151733447238804 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05953562072444894,
            "unit": "iter/sec",
            "range": "stddev: 2.639113473738744",
            "extra": "mean: 16.796667068079113 sec\nrounds: 5"
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
          "id": "aaf415065326871d59857beebab143ef5e5c414a",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-03-29T03:49:56Z",
          "tree_id": "3e2194804140349b89eeaa790ca1f583e53e59ef",
          "url": "https://github.com/sfu-db/connector-x/commit/aaf415065326871d59857beebab143ef5e5c414a"
        },
        "date": 1648526609461,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05136970851525869,
            "unit": "iter/sec",
            "range": "stddev: 0.33344377840249767",
            "extra": "mean: 19.46672521419823 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06284999482979334,
            "unit": "iter/sec",
            "range": "stddev: 2.391214722673938",
            "extra": "mean: 15.910900274664163 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d87f37f434cc286d3e7c22cd0b5714d13452fbb0",
          "message": "Merge pull request #259 from zemelLeong/main\n\nUpgrade `arrow` and export `arrow_schema`",
          "timestamp": "2022-03-31T12:55:55-07:00",
          "tree_id": "ad6d0fe4f7e6902e434b3fd770e0e297a313b843",
          "url": "https://github.com/sfu-db/connector-x/commit/d87f37f434cc286d3e7c22cd0b5714d13452fbb0"
        },
        "date": 1648757054400,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06420049636552345,
            "unit": "iter/sec",
            "range": "stddev: 0.25989658678709976",
            "extra": "mean: 15.576203559339046 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09647749902588243,
            "unit": "iter/sec",
            "range": "stddev: 1.404128993854811",
            "extra": "mean: 10.36511114090681 sec\nrounds: 5"
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
          "id": "f43ada9cf13d6cdd4bd1075b1d2e237ab8e563fa",
          "message": "Merge branch 'main' of github.com:sfu-db/connector-x into main",
          "timestamp": "2022-04-04T02:29:00Z",
          "tree_id": "7e37734b1e8b9b027796ad885237421f4e4fce4d",
          "url": "https://github.com/sfu-db/connector-x/commit/f43ada9cf13d6cdd4bd1075b1d2e237ab8e563fa"
        },
        "date": 1649040233193,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0514805122848318,
            "unit": "iter/sec",
            "range": "stddev: 0.23002437311815827",
            "extra": "mean: 19.42482612580061 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06912477445777539,
            "unit": "iter/sec",
            "range": "stddev: 0.6382047498785258",
            "extra": "mean: 14.466593313962221 sec\nrounds: 5"
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
          "id": "59a705c9a315b9d4de93acf3e14977090530c3d4",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-04-04T02:29:36Z",
          "tree_id": "14e73c97d90ef345b5ac5ed1e0be06d6d13d82d1",
          "url": "https://github.com/sfu-db/connector-x/commit/59a705c9a315b9d4de93acf3e14977090530c3d4"
        },
        "date": 1649041051203,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05231058493971376,
            "unit": "iter/sec",
            "range": "stddev: 0.4024005882657717",
            "extra": "mean: 19.116589905321597 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06182126068180155,
            "unit": "iter/sec",
            "range": "stddev: 2.004784124277917",
            "extra": "mean: 16.175664956867696 sec\nrounds: 5"
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
          "id": "94032b3c3ebb1a0717fb9e841f0c7724c2e5916b",
          "message": "0.2.6-alpha.2: mssql smalldatetime not null bug fix",
          "timestamp": "2022-04-05T03:42:50Z",
          "tree_id": "c2a8862c5aca28936fe5e3849480fb9018323319",
          "url": "https://github.com/sfu-db/connector-x/commit/94032b3c3ebb1a0717fb9e841f0c7724c2e5916b"
        },
        "date": 1649131062170,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06547830326234805,
            "unit": "iter/sec",
            "range": "stddev: 0.2163855315945231",
            "extra": "mean: 15.27223446816206 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09651349090159794,
            "unit": "iter/sec",
            "range": "stddev: 1.5762395526729167",
            "extra": "mean: 10.361245776712895 sec\nrounds: 5"
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
          "id": "e7ac33367843cb7af8c8f2318f556993b75e2da9",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-04-05T03:43:11Z",
          "tree_id": "30582b81058fe11e25118df69899cdda78f7d1ec",
          "url": "https://github.com/sfu-db/connector-x/commit/e7ac33367843cb7af8c8f2318f556993b75e2da9"
        },
        "date": 1649131557002,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06420676657245791,
            "unit": "iter/sec",
            "range": "stddev: 0.2921061881932284",
            "extra": "mean: 15.574682442098856 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0963531574331285,
            "unit": "iter/sec",
            "range": "stddev: 1.729449470616921",
            "extra": "mean: 10.378487084805965 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5fbd84cc5e32819b9e73c082cd8780d4f203a558",
          "message": "Merge pull request #265 from sfu-db/mssql-auth\n\nFix mssql encryption issue and update doc",
          "timestamp": "2022-04-07T13:57:08-07:00",
          "tree_id": "cea891097950544c5bbd1d8b7737bf0a53e49391",
          "url": "https://github.com/sfu-db/connector-x/commit/5fbd84cc5e32819b9e73c082cd8780d4f203a558"
        },
        "date": 1649365550212,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06285683563124704,
            "unit": "iter/sec",
            "range": "stddev: 2.437390275405887",
            "extra": "mean: 15.909168668091297 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.10191161035471802,
            "unit": "iter/sec",
            "range": "stddev: 1.6869845791788711",
            "extra": "mean: 9.812424673885108 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "15766ae3f5f997bd123bdf992b6ca628267e5a05",
          "message": "0.2.6-alpha.3: add encryption option for mssql for azure",
          "timestamp": "2022-04-07T21:28:05Z",
          "tree_id": "c735420c215729e1911116d0d04c693bd106316f",
          "url": "https://github.com/sfu-db/connector-x/commit/15766ae3f5f997bd123bdf992b6ca628267e5a05"
        },
        "date": 1649367395316,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06629474294128014,
            "unit": "iter/sec",
            "range": "stddev: 0.5495981039794798",
            "extra": "mean: 15.084152311831712 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.08475515313189953,
            "unit": "iter/sec",
            "range": "stddev: 2.956388082728527",
            "extra": "mean: 11.7986926227808 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "184a97e043fca143457aa2806a9c8f5f0a4cffa1",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-04-07T22:32:49Z",
          "tree_id": "83175c5cf32c23fab7073415f47ef569cb26fcce",
          "url": "https://github.com/sfu-db/connector-x/commit/184a97e043fca143457aa2806a9c8f5f0a4cffa1"
        },
        "date": 1649371314453,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06652284013351603,
            "unit": "iter/sec",
            "range": "stddev: 0.3199337549151627",
            "extra": "mean: 15.032430936396121 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.06058656015414233,
            "unit": "iter/sec",
            "range": "stddev: 3.5517834262227876",
            "extra": "mean: 16.505310706794262 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "62894396f457eb508b233f181a564bd88a1cd462",
          "message": "add issue templates",
          "timestamp": "2022-04-12T03:58:02Z",
          "tree_id": "1b61bea2896c5c831d9037103d8e960aa0e4c488",
          "url": "https://github.com/sfu-db/connector-x/commit/62894396f457eb508b233f181a564bd88a1cd462"
        },
        "date": 1649736458138,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.053200531732532155,
            "unit": "iter/sec",
            "range": "stddev: 2.2348848474575647",
            "extra": "mean: 18.796804607659578 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09429918551060897,
            "unit": "iter/sec",
            "range": "stddev: 3.183603343095641",
            "extra": "mean: 10.604545464366675 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "9f32e4d4668d0f9446a413a2e82ab48691c3d7af",
          "message": "0.2.6-alpha.4: bug fix for mssql count query construction (do not remove order by)",
          "timestamp": "2022-04-13T05:40:46Z",
          "tree_id": "e1497eedcca94a70cc00f7546a0cc11869d7452b",
          "url": "https://github.com/sfu-db/connector-x/commit/9f32e4d4668d0f9446a413a2e82ab48691c3d7af"
        },
        "date": 1649829250227,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.03894686705912402,
            "unit": "iter/sec",
            "range": "stddev: 7.469529996047076",
            "extra": "mean: 25.676006197929382 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07930176870479656,
            "unit": "iter/sec",
            "range": "stddev: 0.33911034308882754",
            "extra": "mean: 12.610059224814176 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "76ed066e7b5d96ea1a67ed8e4d55117a181a8023",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-04-13T05:41:00Z",
          "tree_id": "3b87b623a089d79738bff208e3e4dcf513574c86",
          "url": "https://github.com/sfu-db/connector-x/commit/76ed066e7b5d96ea1a67ed8e4d55117a181a8023"
        },
        "date": 1649829968724,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.05740725411286705,
            "unit": "iter/sec",
            "range": "stddev: 0.362107481202171",
            "extra": "mean: 17.419401353597642 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07430735904555083,
            "unit": "iter/sec",
            "range": "stddev: 1.530602740436785",
            "extra": "mean: 13.457617291808129 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "d6dbb96b5aed4618fac925f08c15e8967412838b",
          "message": "0.2.6-alpha.4: bug fix for mssql count query construction (do not remove order by when offset exists)",
          "timestamp": "2022-04-13T06:24:17Z",
          "tree_id": "e562f2a628ffe6857db3fed4afc6270a655caf03",
          "url": "https://github.com/sfu-db/connector-x/commit/d6dbb96b5aed4618fac925f08c15e8967412838b"
        },
        "date": 1649832146404,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.03408688007859425,
            "unit": "iter/sec",
            "range": "stddev: 2.9188395456941056",
            "extra": "mean: 29.336800484359266 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.05530100689178405,
            "unit": "iter/sec",
            "range": "stddev: 5.00677531522501",
            "extra": "mean: 18.08285339102149 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "committer": {
            "email": "wangxiaoying0369@gmail.com",
            "name": "wangxiaoying",
            "username": "wangxiaoying"
          },
          "distinct": true,
          "id": "2653765e1557d60744582618286622fff0d07ded",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-04-13T16:16:05Z",
          "tree_id": "4c86ffe4d3d4324ea4ac818b4d12795468348cbd",
          "url": "https://github.com/sfu-db/connector-x/commit/2653765e1557d60744582618286622fff0d07ded"
        },
        "date": 1649867144313,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06272797903816496,
            "unit": "iter/sec",
            "range": "stddev: 0.2264196790002047",
            "extra": "mean: 15.941849479824304 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09795179317331035,
            "unit": "iter/sec",
            "range": "stddev: 1.464902777953516",
            "extra": "mean: 10.209103555977345 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0ecdd480562bc5a013817e4445f7318ae84b6919",
          "message": "Merge pull request #266 from sfu-db/bigquery_tpch\n\nFixed BigQuery multiple pages loading bug",
          "timestamp": "2022-04-20T23:24:44-07:00",
          "tree_id": "c424a8f78df5737e11a032c9075e761b335b625c",
          "url": "https://github.com/sfu-db/connector-x/commit/0ecdd480562bc5a013817e4445f7318ae84b6919"
        },
        "date": 1650522775812,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07164486662925007,
            "unit": "iter/sec",
            "range": "stddev: 0.43761145483302794",
            "extra": "mean: 13.9577341273427 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.11203513992181438,
            "unit": "iter/sec",
            "range": "stddev: 1.6911665362054333",
            "extra": "mean: 8.92577097415924 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8ddf5d566376c7ae75deef37bd69a7053c8f4bff",
          "message": "Merge pull request #271 from kayhoogland/fix/typos-type-hints-readme\n\nFIX - Typo in type hints in Readme",
          "timestamp": "2022-04-21T09:24:03-07:00",
          "tree_id": "fa6d06bed84580b32da7c268150f6acf34b036b5",
          "url": "https://github.com/sfu-db/connector-x/commit/8ddf5d566376c7ae75deef37bd69a7053c8f4bff"
        },
        "date": 1650559009496,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06623559559028373,
            "unit": "iter/sec",
            "range": "stddev: 0.47111204846072996",
            "extra": "mean: 15.097622223943471 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.1009075154490918,
            "unit": "iter/sec",
            "range": "stddev: 1.729504779782704",
            "extra": "mean: 9.910064632445573 sec\nrounds: 5"
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
          "id": "283529f9ca07ec49dd22a28d10df4e4029d5c8c5",
          "message": "0.2.6-alpha.5: add binaryfloat binarydouble to oracle for arrow2",
          "timestamp": "2022-04-29T18:02:09Z",
          "tree_id": "7b665479de0210e9baf5ba0bdc8bf8c0d24b0fcc",
          "url": "https://github.com/sfu-db/connector-x/commit/283529f9ca07ec49dd22a28d10df4e4029d5c8c5"
        },
        "date": 1651255892456,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06981676857038158,
            "unit": "iter/sec",
            "range": "stddev: 0.8068253121826648",
            "extra": "mean: 14.323206594586372 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.11664487779161443,
            "unit": "iter/sec",
            "range": "stddev: 0.984827592891421",
            "extra": "mean: 8.573029685765505 sec\nrounds: 5"
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
          "id": "9471900eaa3415873203f54d8ce0770eef08757a",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-04-29T18:02:59Z",
          "tree_id": "b5b4a5a73be6785cd1cbfb4ce0daf7cbac629521",
          "url": "https://github.com/sfu-db/connector-x/commit/9471900eaa3415873203f54d8ce0770eef08757a"
        },
        "date": 1651256380271,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0664810372018278,
            "unit": "iter/sec",
            "range": "stddev: 0.4364334588836316",
            "extra": "mean: 15.041883251070976 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.10058592671086233,
            "unit": "iter/sec",
            "range": "stddev: 1.8381967376061736",
            "extra": "mean: 9.941748639196158 sec\nrounds: 5"
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
          "id": "7cc4302fb832bac8177745204b58f2ee18285f55",
          "message": "remove libicu66 from ci",
          "timestamp": "2022-04-29T18:32:02Z",
          "tree_id": "3b47d040f1d99bc08bfc921e9f646c920ef555cd",
          "url": "https://github.com/sfu-db/connector-x/commit/7cc4302fb832bac8177745204b58f2ee18285f55"
        },
        "date": 1651257614353,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07095933453894783,
            "unit": "iter/sec",
            "range": "stddev: 0.3241606286373033",
            "extra": "mean: 14.09257860854268 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09362101075967948,
            "unit": "iter/sec",
            "range": "stddev: 2.2636162500188393",
            "extra": "mean: 10.68136299625039 sec\nrounds: 5"
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
          "id": "8f143a93d5877be5566db508f645462da778692c",
          "message": "ci: ubuntu-18.04",
          "timestamp": "2022-04-29T19:04:43Z",
          "tree_id": "fb599e3c390c0196636016ce57f3779b6aa502fa",
          "url": "https://github.com/sfu-db/connector-x/commit/8f143a93d5877be5566db508f645462da778692c"
        },
        "date": 1651259582438,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06813179797958754,
            "unit": "iter/sec",
            "range": "stddev: 0.21993591483104924",
            "extra": "mean: 14.67743446752429 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07519807705300499,
            "unit": "iter/sec",
            "range": "stddev: 3.8299759083322935",
            "extra": "mean: 13.298212390393019 sec\nrounds: 5"
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
          "id": "278f51e45874054249d08a6bb6a76cd96cd2a03c",
          "message": "revert",
          "timestamp": "2022-04-29T19:17:27Z",
          "tree_id": "7b665479de0210e9baf5ba0bdc8bf8c0d24b0fcc",
          "url": "https://github.com/sfu-db/connector-x/commit/278f51e45874054249d08a6bb6a76cd96cd2a03c"
        },
        "date": 1651260310161,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.07040602430083998,
            "unit": "iter/sec",
            "range": "stddev: 0.37563390915197964",
            "extra": "mean: 14.203330040723085 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.11539501683892674,
            "unit": "iter/sec",
            "range": "stddev: 1.4735207749369097",
            "extra": "mean: 8.665885472297669 sec\nrounds: 5"
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
          "id": "c76d5934d5c4eda3d2958621e97e6a32f6665a77",
          "message": "update ci ubuntu-latest to ubuntu-20.04",
          "timestamp": "2022-05-02T16:55:17Z",
          "tree_id": "97bd29401009737057fa658bca03cbb161b561be",
          "url": "https://github.com/sfu-db/connector-x/commit/c76d5934d5c4eda3d2958621e97e6a32f6665a77"
        },
        "date": 1651511085825,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0691034886177441,
            "unit": "iter/sec",
            "range": "stddev: 0.7185162013475638",
            "extra": "mean: 14.471049436181783 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.0809513350500448,
            "unit": "iter/sec",
            "range": "stddev: 2.9753573122407384",
            "extra": "mean: 12.35310077816248 sec\nrounds: 5"
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "edc0c39f13385725ade71d03a4c9703153e9486a",
          "message": "Merge pull request #280 from sfu-db/fedquery\n\nSupport federated query",
          "timestamp": "2022-05-09T18:31:49-07:00",
          "tree_id": "a38137b5ccb4331caeee476279e3152add7e6bdf",
          "url": "https://github.com/sfu-db/connector-x/commit/edc0c39f13385725ade71d03a4c9703153e9486a"
        },
        "date": 1652146777450,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0773729003985072,
            "unit": "iter/sec",
            "range": "stddev: 0.5071952224069842",
            "extra": "mean: 12.92442179173231 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.10976332449088373,
            "unit": "iter/sec",
            "range": "stddev: 1.3121833469731892",
            "extra": "mean: 9.110511226207018 sec\nrounds: 5"
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
          "id": "f712dd2e188ae219e8ec64bef598e7f3a6905337",
          "message": "0.2.6-alpha.6: read_sql supports federated query",
          "timestamp": "2022-05-09T18:52:44-07:00",
          "tree_id": "634cd0b91c4409fa832f112942a3d7fb8e65fe10",
          "url": "https://github.com/sfu-db/connector-x/commit/f712dd2e188ae219e8ec64bef598e7f3a6905337"
        },
        "date": 1652148053463,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.06682750571607347,
            "unit": "iter/sec",
            "range": "stddev: 0.7405878006518469",
            "extra": "mean: 14.963898312300444 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.09832909554888525,
            "unit": "iter/sec",
            "range": "stddev: 2.3882340322862134",
            "extra": "mean: 10.169929809868336 sec\nrounds: 5"
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
          "id": "b6fdc2971e5f157438b96dd4012298b479e74cef",
          "message": "Merge branch 'main' into prerelease",
          "timestamp": "2022-05-09T18:53:00-07:00",
          "tree_id": "897b3483f6ed8079283adb3e038aa5565130d28b",
          "url": "https://github.com/sfu-db/connector-x/commit/b6fdc2971e5f157438b96dd4012298b479e74cef"
        },
        "date": 1652148518365,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.0863370813764481,
            "unit": "iter/sec",
            "range": "stddev: 0.7375144035187285",
            "extra": "mean: 11.582508744299412 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.10529369530916546,
            "unit": "iter/sec",
            "range": "stddev: 2.2009142541536417",
            "extra": "mean: 9.497244797647 sec\nrounds: 5"
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
          "id": "8d5c8fc63935cb0e691a8376373e665c1506a874",
          "message": "fix release",
          "timestamp": "2022-05-09T21:17:32-07:00",
          "tree_id": "a920b009b344317bc64911bbbb868461cd3cf753",
          "url": "https://github.com/sfu-db/connector-x/commit/8d5c8fc63935cb0e691a8376373e665c1506a874"
        },
        "date": 1652156747960,
        "tool": "pytest",
        "benches": [
          {
            "name": "connectorx/tests/benchmarks.py::bench_mysql",
            "value": 0.08439515452062411,
            "unit": "iter/sec",
            "range": "stddev: 0.4549547669848178",
            "extra": "mean: 11.849021495133638 sec\nrounds: 5"
          },
          {
            "name": "connectorx/tests/benchmarks.py::bench_postgres",
            "value": 0.07673412682828575,
            "unit": "iter/sec",
            "range": "stddev: 6.099921756546731",
            "extra": "mean: 13.032011196762323 sec\nrounds: 5"
          }
        ]
      }
    ]
  }
}