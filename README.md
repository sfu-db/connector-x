# ConnectorX [![status][ci_badge]][ci_page] [![discussions][discussion_badge]][discussion_page] [![Downloads][download_badge]][download_page]

[ci_badge]: https://github.com/sfu-db/connector-x/workflows/ci/badge.svg
[ci_page]: https://github.com/sfu-db/connector-x/actions
[discussion_badge]: https://img.shields.io/badge/Forum-Github%20Discussions-blue
[discussion_page]: https://github.com/sfu-db/connector-x/discussions
[download_badge]: https://pepy.tech/badge/connectorx
[download_page]: https://pepy.tech/project/connectorx

Load data from <img src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/sources.gif" width="6.5%" style="margin-bottom: -2px"/> to <img src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/destinations.gif" width="7%" style="margin-bottom: -2px"/>, the fastest way.

ConnectorX enables you to load data from databases into Python in the fastest and most memory efficient way.

What you need is one line of code:

```python
import connectorx as cx

cx.read_sql("postgresql://username:password@server:port/database", "SELECT * FROM lineitem")
```

Optionally, you can accelerate the data loading using parallelism by specifying a partition column.

```python
import connectorx as cx

cx.read_sql("postgresql://username:password@server:port/database", "SELECT * FROM lineitem", partition_on="l_orderkey", partition_num=10)
```

The function will partition the query by **evenly** splitting the specified column to the amount of partitions.
ConnectorX will assign one thread for each partition to load and write data in parallel.
Currently, we support partitioning on **numerical** columns (**cannot contain NULL**) for **SPJA** queries. 

**Experimental: We are now providing federated query support, you can write a single query to join tables from two or more databases!**
```python
import connectorx as cx
db1 = "postgresql://username1:password1@server1:port1/database1"
db2 = "postgresql://username2:password2@server2:port2/database2"
cx.read_sql({"db1": db1, "db2": db2}, "SELECT * FROM db1.nation n, db2.region r where n.n_regionkey = r.r_regionkey")
```
By default, we pushdown all joins from the same data source. More details for setup and configuration can be found [here](https://github.com/sfu-db/connector-x/blob/main/Federation.md).

Check out more detailed usage and examples [here](https://sfu-db.github.io/connector-x/api.html). A general introduction of the project can be found in this [blog post](https://towardsdatascience.com/connectorx-the-fastest-way-to-load-data-from-databases-a65d4d4062d5).

# Installation

```bash
pip install connectorx
```

_For AArch64 or ARM64 Linux users, `connectorx==0.4.3 & above` is only available for distributions using `glibc 2.35` and above. Specifically, the re-release for this architecture was tested on Ubuntu 22.04. For older distributions, the latest version available is `connectorx==0.2.3` due to dependency limitations._

Check out [here](https://sfu-db.github.io/connector-x/install.html#build-from-source-code) to see how to build python wheel from source.

# Performance

We compared different solutions in Python that provides the `read_sql` function, by loading a 10x TPC-H lineitem table (8.6GB) from Postgres into a DataFrame, with 4 cores parallelism.

## Time chart, lower is better.

<p align="center"><img alt="time chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/pg-time.png"/></p>

## Memory consumption chart, lower is better.

<p align="center"><img alt="memory chart" src="https://raw.githubusercontent.com/sfu-db/connector-x/main/assets/pg-mem.png"/></p>

In conclusion, ConnectorX uses up to **3x** less memory and **21x** less time (**3x** less memory and **13x** less time compared with Pandas.). More on [here](https://github.com/sfu-db/connector-x/blob/main/Benchmark.md#benchmark-result-on-aws-r54xlarge).

## How does ConnectorX achieve a lightning speed while keeping the memory footprint low?

We observe that existing solutions more or less do data copy multiple times when downloading the data.
Additionally, implementing a data intensive application in Python brings additional cost.

ConnectorX is written in Rust and follows "zero-copy" principle.
This allows it to make full use of the CPU by becoming cache and branch predictor friendly. Moreover, the architecture of ConnectorX ensures the data will be copied exactly once, directly from the source to the destination.

## How does ConnectorX download the data?

Upon receiving the query, e.g. `SELECT * FROM lineitem`, ConnectorX will first get the schema of the result set. Depending on the data source, this process may envolve issuing a `LIMIT 1` query `SELECT * FROM lineitem LIMIT 1`.

Then, if `partition_on` is specified, ConnectorX will issue `SELECT MIN($partition_on), MAX($partition_on) FROM (SELECT * FROM lineitem)` to know the range of the partition column.
After that, the original query is split into partitions based on the min/max information, e.g. `SELECT * FROM (SELECT * FROM lineitem) WHERE $partition_on > 0 AND $partition_on < 10000`.
ConnectorX will then run a count query to get the partition size (e.g. `SELECT COUNT(*) FROM (SELECT * FROM lineitem) WHERE $partition_on > 0 AND $partition_on < 10000`). If the partition
is not specified, the count query will be `SELECT COUNT(*) FROM (SELECT * FROM lineitem)`.

Finally, ConnectorX will use the schema info as well as the count info to allocate memory and download data by executing the queries normally.

Once the downloading begins, there will be one thread for each partition so that the data are downloaded in parallel at the partition level. The thread will issue the query of the corresponding
partition to the database and then write the returned data to the destination row-wise or column-wise (depends on the database) in a streaming fashion. 


# Supported Sources & Destinations

Example connection string, supported protocols and data types for each data source can be found [here](https://sfu-db.github.io/connector-x/databases.html).

For more planned data sources, please check out our [discussion](https://github.com/sfu-db/connector-x/discussions/61).

## Sources
- [x] Postgres
- [x] Mysql
- [x] Mariadb (through mysql protocol)
- [x] Sqlite
- [x] Redshift (through postgres protocol)
- [x] Clickhouse (through mysql protocol)
- [x] SQL Server
- [x] Azure SQL Database (through mssql protocol)
- [x] Oracle
- [x] Big Query
- [x] Trino
- [ ] ODBC (WIP)
- [ ] ...

## Destinations
- [x] Pandas
- [x] PyArrow
- [x] Modin (through Pandas)
- [x] Dask (through Pandas)
- [x] Polars (through PyArrow)

# Documentation

Doc: https://sfu-db.github.io/connector-x/intro.html
Rust docs: [stable](https://docs.rs/connectorx) [nightly](https://sfu-db.github.io/connector-x/connectorx/)

# Next Plan

Checkout our [discussion][discussion_page] to participate in deciding our next plan!

# Historical Benchmark Results

https://sfu-db.github.io/connector-x/dev/bench/

# Developer's Guide
Please see [Developer's Guide](https://github.com/sfu-db/connector-x/blob/main/CONTRIBUTING.md) for information about developing ConnectorX.

# Supports

You are always welcomed to:
1. Ask questions & propose new ideas in our github [discussion][discussion_page].
2. Ask questions in stackoverflow. Make sure to have #connectorx attached.

# Organizations and Projects using ConnectorX

[<img src="https://raw.githubusercontent.com/pola-rs/polars-static/master/logos/polars-logo-dark.svg" height="60" style="margin-bottom: -2px"/>](https://github.com/pola-rs/polars)
[<img src="https://raw.githubusercontent.com/sfu-db/dataprep/develop/assets/logo.png" height="60" style="margin-bottom: -2px"/>](https://dataprep.ai/)
[<img src="https://github.com/modin-project/modin/blob/3d6368edf311995ad231ec5342a51cd9e4e3dc20/docs/img/MODIN_ver2_hrz.png?raw=true" height="60" style="margin-bottom: -2px"/>](https://modin.readthedocs.io)

To add your project/organization here, reply our post [here](https://github.com/sfu-db/connector-x/discussions/146)

# Citing ConnectorX

If you use ConnectorX, please consider citing the following paper:

Xiaoying Wang, Weiyuan Wu, Jinze Wu, Yizhou Chen, Nick Zrymiak, Changbo Qu, Lampros Flokas, George Chow, Jiannan Wang, Tianzheng Wang, Eugene Wu, Qingqing Zhou. [ConnectorX: Accelerating Data Loading From Databases to Dataframes.](https://www.vldb.org/pvldb/vol15/p2994-wang.pdf) _VLDB 2022_.

BibTeX entry:

```bibtex
@article{connectorx2022,
  author    = {Xiaoying Wang and Weiyuan Wu and Jinze Wu and Yizhou Chen and Nick Zrymiak and Changbo Qu and Lampros Flokas and George Chow and Jiannan Wang and Tianzheng Wang and Eugene Wu and Qingqing Zhou},
  title     = {ConnectorX: Accelerating Data Loading From Databases to Dataframes},
  journal   = {Proc. {VLDB} Endow.},
  volume    = {15},
  number    = {11},
  pages     = {2994--3003},
  year      = {2022},
  url       = {https://www.vldb.org/pvldb/vol15/p2994-wang.pdf},
}
```

# Contributors

<!-- readme: contributors -start -->
<table>
	<tbody>
		<tr>
            <td align="center">
                <a href="https://github.com/wangxiaoying">
                    <img src="https://avatars.githubusercontent.com/u/5569610?v=4" width="66;" alt="wangxiaoying"/>
                    <br />
                    <sub><b>Xiaoying Wang</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/dovahcrow">
                    <img src="https://avatars.githubusercontent.com/u/998606?v=4" width="66;" alt="dovahcrow"/>
                    <br />
                    <sub><b>Weiyuan Wu</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/Wukkkinz-0725">
                    <img src="https://avatars.githubusercontent.com/u/60677420?v=4" width="66;" alt="Wukkkinz-0725"/>
                    <br />
                    <sub><b>Null</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/EricFecteau">
                    <img src="https://avatars.githubusercontent.com/u/96687807?v=4" width="66;" alt="EricFecteau"/>
                    <br />
                    <sub><b>EricFecteau</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/Yizhou150">
                    <img src="https://avatars.githubusercontent.com/u/62522644?v=4" width="66;" alt="Yizhou150"/>
                    <br />
                    <sub><b>Yizhou</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/pangjunrong">
                    <img src="https://avatars.githubusercontent.com/u/61274749?v=4" width="66;" alt="pangjunrong"/>
                    <br />
                    <sub><b>Pang Jun Rong (Jayden)</b></sub>
                </a>
            </td>
		</tr>
		<tr>
            <td align="center">
                <a href="https://github.com/zen-xu">
                    <img src="https://avatars.githubusercontent.com/u/38552291?v=4" width="66;" alt="zen-xu"/>
                    <br />
                    <sub><b>ZhengYu, Xu</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/domnikl">
                    <img src="https://avatars.githubusercontent.com/u/603116?v=4" width="66;" alt="domnikl"/>
                    <br />
                    <sub><b>Dominik Liebler</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/wseaton">
                    <img src="https://avatars.githubusercontent.com/u/16678729?v=4" width="66;" alt="wseaton"/>
                    <br />
                    <sub><b>Will Eaton</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/AnatolyBuga">
                    <img src="https://avatars.githubusercontent.com/u/60788447?v=4" width="66;" alt="AnatolyBuga"/>
                    <br />
                    <sub><b>Anatoly Bugakov</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/Jordan-M-Young">
                    <img src="https://avatars.githubusercontent.com/u/54070169?v=4" width="66;" alt="Jordan-M-Young"/>
                    <br />
                    <sub><b>Jordan M. Young</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/jsjasonseba">
                    <img src="https://avatars.githubusercontent.com/u/46563896?v=4" width="66;" alt="jsjasonseba"/>
                    <br />
                    <sub><b>Jason</b></sub>
                </a>
            </td>
		</tr>
		<tr>
            <td align="center">
                <a href="https://github.com/auyer">
                    <img src="https://avatars.githubusercontent.com/u/12375421?v=4" width="66;" alt="auyer"/>
                    <br />
                    <sub><b>Rafael Passos</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/jinzew">
                    <img src="https://avatars.githubusercontent.com/u/55274369?v=4" width="66;" alt="jinzew"/>
                    <br />
                    <sub><b>Null</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/gruuya">
                    <img src="https://avatars.githubusercontent.com/u/45558892?v=4" width="66;" alt="gruuya"/>
                    <br />
                    <sub><b>Marko Grujic</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/alswang18">
                    <img src="https://avatars.githubusercontent.com/u/44207558?v=4" width="66;" alt="alswang18"/>
                    <br />
                    <sub><b>Alec Wang</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/lBilali">
                    <img src="https://avatars.githubusercontent.com/u/5528169?v=4" width="66;" alt="lBilali"/>
                    <br />
                    <sub><b>Lulzim Bilali</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/ritchie46">
                    <img src="https://avatars.githubusercontent.com/u/3023000?v=4" width="66;" alt="ritchie46"/>
                    <br />
                    <sub><b>Ritchie Vink</b></sub>
                </a>
            </td>
		</tr>
		<tr>
            <td align="center">
                <a href="https://github.com/houqp">
                    <img src="https://avatars.githubusercontent.com/u/670302?v=4" width="66;" alt="houqp"/>
                    <br />
                    <sub><b>QP Hou</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/wKollendorf">
                    <img src="https://avatars.githubusercontent.com/u/83725977?v=4" width="66;" alt="wKollendorf"/>
                    <br />
                    <sub><b>Null</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/glennpierce">
                    <img src="https://avatars.githubusercontent.com/u/691783?v=4" width="66;" alt="glennpierce"/>
                    <br />
                    <sub><b>Glenn Pierce</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/jorgecarleitao">
                    <img src="https://avatars.githubusercontent.com/u/2772607?v=4" width="66;" alt="jorgecarleitao"/>
                    <br />
                    <sub><b>Jorge Leitao</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/chitralverma">
                    <img src="https://avatars.githubusercontent.com/u/11135032?v=4" width="66;" alt="chitralverma"/>
                    <br />
                    <sub><b>Chitral Verma</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/quambene">
                    <img src="https://avatars.githubusercontent.com/u/33333672?v=4" width="66;" alt="quambene"/>
                    <br />
                    <sub><b>Null</b></sub>
                </a>
            </td>
		</tr>
		<tr>
            <td align="center">
                <a href="https://github.com/CBQu">
                    <img src="https://avatars.githubusercontent.com/u/16992497?v=4" width="66;" alt="CBQu"/>
                    <br />
                    <sub><b>CbQu</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/tschm">
                    <img src="https://avatars.githubusercontent.com/u/2046079?v=4" width="66;" alt="tschm"/>
                    <br />
                    <sub><b>Thomas Schmelzer</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/maxb2">
                    <img src="https://avatars.githubusercontent.com/u/9096667?v=4" width="66;" alt="maxb2"/>
                    <br />
                    <sub><b>Matthew Anderson</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/JakkuSakura">
                    <img src="https://avatars.githubusercontent.com/u/33482468?v=4" width="66;" alt="JakkuSakura"/>
                    <br />
                    <sub><b>Jakku Sakura</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/therealhieu">
                    <img src="https://avatars.githubusercontent.com/u/38937534?v=4" width="66;" alt="therealhieu"/>
                    <br />
                    <sub><b>Hieu Minh Nguyen</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/FerriLuli">
                    <img src="https://avatars.githubusercontent.com/u/110925223?v=4" width="66;" alt="FerriLuli"/>
                    <br />
                    <sub><b>FerriLuli</b></sub>
                </a>
            </td>
		</tr>
		<tr>
            <td align="center">
                <a href="https://github.com/quixoten">
                    <img src="https://avatars.githubusercontent.com/u/63675?v=4" width="66;" alt="quixoten"/>
                    <br />
                    <sub><b>Devin Christensen</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/DeflateAwning">
                    <img src="https://avatars.githubusercontent.com/u/11021263?v=4" width="66;" alt="DeflateAwning"/>
                    <br />
                    <sub><b>DeflateAwning</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/alexander-beedie">
                    <img src="https://avatars.githubusercontent.com/u/2613171?v=4" width="66;" alt="alexander-beedie"/>
                    <br />
                    <sub><b>Alexander Beedie</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/zzzdong">
                    <img src="https://avatars.githubusercontent.com/u/5125482?v=4" width="66;" alt="zzzdong"/>
                    <br />
                    <sub><b>Null</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/zemelLeong">
                    <img src="https://avatars.githubusercontent.com/u/26835087?v=4" width="66;" alt="zemelLeong"/>
                    <br />
                    <sub><b>zemel leong</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/venkashank">
                    <img src="https://avatars.githubusercontent.com/u/27744439?v=4" width="66;" alt="venkashank"/>
                    <br />
                    <sub><b>Null</b></sub>
                </a>
            </td>
		</tr>
		<tr>
            <td align="center">
                <a href="https://github.com/tvandelooij">
                    <img src="https://avatars.githubusercontent.com/u/128473860?v=4" width="66;" alt="tvandelooij"/>
                    <br />
                    <sub><b>tvandelooij</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/surister">
                    <img src="https://avatars.githubusercontent.com/u/37985796?v=4" width="66;" alt="surister"/>
                    <br />
                    <sub><b>Ivan</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/phanindra-ramesh">
                    <img src="https://avatars.githubusercontent.com/u/16794420?v=4" width="66;" alt="phanindra-ramesh"/>
                    <br />
                    <sub><b>Null</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/messense">
                    <img src="https://avatars.githubusercontent.com/u/1556054?v=4" width="66;" alt="messense"/>
                    <br />
                    <sub><b>Messense</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/kotval">
                    <img src="https://avatars.githubusercontent.com/u/11917243?v=4" width="66;" alt="kotval"/>
                    <br />
                    <sub><b>Kotval</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/albcunha">
                    <img src="https://avatars.githubusercontent.com/u/13671325?v=4" width="66;" alt="albcunha"/>
                    <br />
                    <sub><b>Null</b></sub>
                </a>
            </td>
		</tr>
		<tr>
            <td align="center">
                <a href="https://github.com/rursprung">
                    <img src="https://avatars.githubusercontent.com/u/39383228?v=4" width="66;" alt="rursprung"/>
                    <br />
                    <sub><b>Ralph Ursprung</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/MatsMoll">
                    <img src="https://avatars.githubusercontent.com/u/4439131?v=4" width="66;" alt="MatsMoll"/>
                    <br />
                    <sub><b>Mats Eikeland Mollestad</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/marianoguerra">
                    <img src="https://avatars.githubusercontent.com/u/68463?v=4" width="66;" alt="marianoguerra"/>
                    <br />
                    <sub><b>Mariano Guerra</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/kevinheavey">
                    <img src="https://avatars.githubusercontent.com/u/24635973?v=4" width="66;" alt="kevinheavey"/>
                    <br />
                    <sub><b>Kevin Heavey</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/kayhoogland">
                    <img src="https://avatars.githubusercontent.com/u/22837350?v=4" width="66;" alt="kayhoogland"/>
                    <br />
                    <sub><b>Kay Hoogland</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/holicc">
                    <img src="https://avatars.githubusercontent.com/u/19146591?v=4" width="66;" alt="holicc"/>
                    <br />
                    <sub><b>Joe</b></sub>
                </a>
            </td>
		</tr>
		<tr>
            <td align="center">
                <a href="https://github.com/deepsourcebot">
                    <img src="https://avatars.githubusercontent.com/u/60907429?v=4" width="66;" alt="deepsourcebot"/>
                    <br />
                    <sub><b>DeepSource Bot</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/bealdav">
                    <img src="https://avatars.githubusercontent.com/u/1853434?v=4" width="66;" alt="bealdav"/>
                    <br />
                    <sub><b>David Beal</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/AndrewJackson2020">
                    <img src="https://avatars.githubusercontent.com/u/46945903?v=4" width="66;" alt="AndrewJackson2020"/>
                    <br />
                    <sub><b>Andrew Jackson</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/Cabbagec">
                    <img src="https://avatars.githubusercontent.com/u/14164987?v=4" width="66;" alt="Cabbagec"/>
                    <br />
                    <sub><b>Brandon</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/Amar1729">
                    <img src="https://avatars.githubusercontent.com/u/15623522?v=4" width="66;" alt="Amar1729"/>
                    <br />
                    <sub><b>Amar Paul</b></sub>
                </a>
            </td>
            <td align="center">
                <a href="https://github.com/aljazerzen">
                    <img src="https://avatars.githubusercontent.com/u/11072061?v=4" width="66;" alt="aljazerzen"/>
                    <br />
                    <sub><b>Aljaž Mur Eržen</b></sub>
                </a>
            </td>
		</tr>
		<tr>
            <td align="center">
                <a href="https://github.com/aimtsou">
                    <img src="https://avatars.githubusercontent.com/u/2598924?v=4" width="66;" alt="aimtsou"/>
                    <br />
                    <sub><b>Aimilios Tsouvelekakis</b></sub>
                </a>
            </td>
		</tr>
	<tbody>
</table>
<!-- readme: contributors -end -->
