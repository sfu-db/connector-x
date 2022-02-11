# Getting Started

Whether you write your book's content in Jupyter Notebooks (`.ipynb`) or
in regular markdown files (`.md`), you'll write in the same flavor of markdown
called **MyST Markdown**.

# Installation

## Pip

### Linux and OSX

To install ConnectorX on Linux and OSX, use the following command:

```bash
pip install connectorx
```
### Windows

To install ConnectorX on Windows, use the following command:

```bash
pip install connectorx
```

## Conda

To install ConnectorX via conda, use the following command:

```bash
conda install -c conda-forge connectorx
```

# Basic usage
ConnectorX enables you to load data from databases into Python in the fastest and most memory efficient way. Here are some basic usages.

## Read a DataFrame from a SQL using a single thread
```python
import connectorx as cx

postgres_url = "postgresql://username:password@server:port/database"
query = "SELECT * FROM lineitem

cx.read_sql(postgres_url, query)
```

## Read a DataFrame from a SQL using 10 threads
```python
import connectorx as cx

postgres_url = "postgresql://username:password@server:port/database"
query = "SELECT * FROM lineitem"

cx.read_sql(postgres_url, query, partition_on="l_orderkey", partition_num=10)
```


