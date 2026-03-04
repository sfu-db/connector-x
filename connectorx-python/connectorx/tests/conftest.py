"""Pytest configuration for tests with testcontainers."""
import json
import os
from pathlib import Path
from typing import Generator, Any, Optional
import urllib.error
import urllib.request

import pytest

# Check if Docker is available
try:
    from testcontainers.oracle import OracleDbContainer
    from testcontainers.clickhouse import ClickHouseContainer
    from testcontainers.postgres import PostgresContainer
    from testcontainers.mysql import MySqlContainer
    from testcontainers.trino import TrinoContainer
    TESTCONTAINERS_AVAILABLE = True
except ImportError:
    TESTCONTAINERS_AVAILABLE = False
    OracleDbContainer = None  # Avoid type hint errors
    ClickHouseContainer = None
    PostgresContainer = None
    MySqlContainer = None
    TrinoContainer = None


@pytest.fixture(scope="session")
def postgres_container() -> Generator[Optional[Any], None, None]:
    """
    Fixture that starts a PostgreSQL container for tests.

    This fixture is used at session level to avoid restarting the container
    for each test.
    """
    # If POSTGRES_URL is already defined, use it (for backward compatibility)
    if os.environ.get("POSTGRES_URL"):
        yield None
        return

    # If testcontainers is not available, skip
    if not TESTCONTAINERS_AVAILABLE:
        pytest.skip("testcontainers is not installed. Install with: pip install testcontainers")

    init_script = Path(__file__).parent.parent.parent.parent / "scripts" / "postgres.sql"
    if not init_script.exists():
        raise FileNotFoundError(f"PostgreSQL init script not found: {init_script}")

    # Use pgvector image because scripts/postgres.sql creates the vector extension.
    postgres_container = PostgresContainer(
        image="pgvector/pgvector:pg17",
        username="postgres",
        password="postgres",
        dbname="postgres",
    ).with_volume_mapping(str(init_script), "/docker-entrypoint-initdb.d/postgres.sql", mode="ro")

    with postgres_container as postgres:

        # connectorx expects a URL without SQLAlchemy driver suffix.
        os.environ["POSTGRES_URL"] = postgres.get_connection_url(driver=None)

        try:
            yield postgres
        finally:
            if "POSTGRES_URL" in os.environ:
                del os.environ["POSTGRES_URL"]


@pytest.fixture(scope="module")
def postgres_url(postgres_container: Optional[Any]) -> str:
    """
    Fixture that returns the PostgreSQL connection URL.

    This fixture uses either the testcontainers container or a URL
    defined in the POSTGRES_URL environment variable.
    """
    if os.environ.get("POSTGRES_URL"):
        return os.environ["POSTGRES_URL"]

    if postgres_container is not None:
        return postgres_container.get_connection_url(driver=None)

    pytest.skip("No PostgreSQL database available for tests")


@pytest.fixture(scope="session")
def mysql_container() -> Generator[Optional[Any], None, None]:
    """
    Fixture that starts a MySQL container for tests.

    This fixture is used at session level to avoid restarting the container
    for each test.
    """
    # If MYSQL_URL is already defined, use it (for backward compatibility)
    if os.environ.get("MYSQL_URL"):
        yield None
        return

    # If testcontainers is not available, skip
    if not TESTCONTAINERS_AVAILABLE:
        pytest.skip("testcontainers is not installed. Install with: pip install testcontainers")

    init_script = Path(__file__).parent.parent.parent.parent / "scripts" / "mysql.sql"
    if not init_script.exists():
        raise FileNotFoundError(f"MySQL init script not found: {init_script}")

    mysql_container = MySqlContainer(
        image="ghcr.io/wangxiaoying/mysql:latest",
        username="root",
        password="mysql",
        dbname="mysql",
    ).with_volume_mapping(str(init_script), "/docker-entrypoint-initdb.d/mysql.sql", mode="ro")

    with mysql_container as mysql:
        os.environ["MYSQL_URL"] = mysql.get_connection_url()

        try:
            yield mysql
        finally:
            if "MYSQL_URL" in os.environ:
                del os.environ["MYSQL_URL"]


@pytest.fixture(scope="module")
def mysql_url(mysql_container: Optional[Any]) -> str:
    """
    Fixture that returns the MySQL connection URL.

    This fixture uses either the testcontainers container or a URL
    defined in the MYSQL_URL environment variable.
    """
    if os.environ.get("MYSQL_URL"):
        return os.environ["MYSQL_URL"]

    if mysql_container is not None:
        return mysql_container.get_connection_url()

    pytest.skip("No MySQL database available for tests")


def _run_trino_statement(host: str, port: str, statement: str) -> None:
    if not statement.strip():
        return

    url = f"http://{host}:{port}/v1/statement"
    headers = {
        "X-Trino-User": "test",
        "X-Trino-Catalog": "test",
        "X-Trino-Schema": "test",
    }
    request = urllib.request.Request(
        url,
        data=statement.encode("utf-8"),
        headers=headers,
        method="POST",
    )

    try:
        with urllib.request.urlopen(request) as response:
            payload = json.loads(response.read().decode("utf-8"))
    except urllib.error.HTTPError as e:
        raise RuntimeError(f"Trino initialization query failed: {statement}") from e

    if payload.get("error"):
        raise RuntimeError(f"Trino initialization query error: {payload['error']}")

    next_uri = payload.get("nextUri")
    while next_uri:
        poll_request = urllib.request.Request(next_uri, headers=headers, method="GET")
        with urllib.request.urlopen(poll_request) as poll_response:
            payload = json.loads(poll_response.read().decode("utf-8"))
        if payload.get("error"):
            raise RuntimeError(f"Trino initialization query error: {payload['error']}")
        next_uri = payload.get("nextUri")


def _iter_trino_init_statements(script: str) -> Generator[str, None, None]:
    for raw in script.split(";"):
        statement = raw.strip()
        if not statement:
            continue
        # Trino memory connector does not support row-level DELETE.
        if statement.upper().startswith("DELETE FROM"):
            continue
        yield statement


@pytest.fixture(scope="session")
def trino_container() -> Generator[Optional[Any], None, None]:
    """
    Fixture that starts a Trino container for tests.

    This fixture is used at session level to avoid restarting the container
    for each test.
    """
    # If TRINO_URL is already defined, use it (for backward compatibility)
    if os.environ.get("TRINO_URL"):
        yield None
        return

    # If testcontainers is not available, skip
    if not TESTCONTAINERS_AVAILABLE:
        pytest.skip("testcontainers is not installed. Install with: pip install testcontainers")

    trino_catalog = Path(__file__).with_name("trino-test.properties")
    trino_catalog.write_text("connector.name=memory\n", encoding="utf-8")

    trino_container = TrinoContainer(
        image="trinodb/trino:latest",
        user="test",
    ).with_volume_mapping(str(trino_catalog), "/etc/trino/catalog/test.properties", mode="ro")

    init_script = Path(__file__).parent.parent.parent.parent / "scripts" / "trino.sql"

    with trino_container as trino:
        host = trino.get_container_host_ip()
        port = trino.get_exposed_port(8080)
        os.environ["TRINO_URL"] = f"trino://test@{host}:{port}/test"

        if init_script.exists():
            script = init_script.read_text(encoding="utf-8")
            for statement in _iter_trino_init_statements(script):
                _run_trino_statement(host, port, statement)

        try:
            yield trino
        finally:
            if "TRINO_URL" in os.environ:
                del os.environ["TRINO_URL"]
            trino_catalog.unlink(missing_ok=True)


@pytest.fixture(scope="module")
def trino_url(trino_container: Optional[Any]) -> str:
    """
    Fixture that returns the Trino connection URL.

    This fixture uses either the testcontainers container or a URL
    defined in the TRINO_URL environment variable.
    """
    if os.environ.get("TRINO_URL"):
        return os.environ["TRINO_URL"]

    if trino_container is not None:
        host = trino_container.get_container_host_ip()
        port = trino_container.get_exposed_port(8080)
        return f"trino://test@{host}:{port}/test"

    pytest.skip("No Trino database available for tests")




@pytest.fixture(scope="session")
def oracle_container() -> Generator[Optional[Any], None, None]:
    """
    Fixture that starts an Oracle Database Free container for tests.
    
    This fixture is used at session level to avoid restarting the container
    for each test, which would be very slow.
    """
    # If ORACLE_URL is already defined, use it (for backward compatibility)
    if os.environ.get("ORACLE_URL"):
        yield None
        return
    
    # If testcontainers is not available, skip
    if not TESTCONTAINERS_AVAILABLE:
        pytest.skip("testcontainers[oracle-free] is not installed. Install with: pip install testcontainers[oracle-free]")
    
    # Start Oracle container with context manager (recommended)
    with OracleDbContainer() as oracle:
        # Execute initialization script
        init_script = Path(__file__).parent.parent.parent.parent / "scripts" / "oracle.sql"
        if init_script.exists():
            try:
                import sqlalchemy
                
                # Use the connection URL provided by testcontainers
                engine = sqlalchemy.create_engine(oracle.get_connection_url())
                
                # Read SQL script
                with init_script.open('r') as f:
                    sql_content = f.read()
                
                # Remove DROP TABLE statements (tables don't exist yet in fresh container)
                lines = sql_content.split('\n')
                cleaned_lines = [line for line in lines if not line.strip().upper().startswith('DROP TABLE')]
                cleaned_sql = '\n'.join(cleaned_lines)
                
                # Execute using sqlalchemy - split by semicolon
                with engine.begin() as connection:
                    for statement in cleaned_sql.split(';'):
                        statement = statement.strip()
                        if statement:
                            connection.execute(sqlalchemy.text(statement))
                    # Ensure commit happens
                    connection.commit()
            except Exception as e:
                print(f"Warning: Could not initialize database: {e}")
                import traceback
                traceback.print_exc()
        
        # Convert SQLAlchemy URL to connectorx format
        # From: oracle+oracledb://system:pass@localhost:port/?service_name=FREEPDB1
        # To: oracle://system:pass@localhost:port/FREEPDB1
        sqlalchemy_url = oracle.get_connection_url()
        connectorx_url = sqlalchemy_url.replace("oracle+oracledb://", "oracle://")
        connectorx_url = connectorx_url.replace("/?service_name=", "/")
        
        # Set ORACLE_URL for tests
        os.environ["ORACLE_URL"] = connectorx_url
        
        try:
            yield oracle
        finally:
            # Clean up environment variable
            if "ORACLE_URL" in os.environ:
                del os.environ["ORACLE_URL"]


@pytest.fixture(scope="module")
def oracle_url(oracle_container: Optional[Any]) -> str:
    """
    Fixture that returns the Oracle connection URL.
    
    This fixture uses either the testcontainers container or a URL
    defined in the ORACLE_URL environment variable.
    """
    # If ORACLE_URL is already defined, use it
    if os.environ.get("ORACLE_URL"):
        return os.environ["ORACLE_URL"]
    
    # Otherwise, use testcontainers
    if oracle_container is not None:
        # Convert SQLAlchemy URL to connectorx format
        # From: oracle+oracledb://system:pass@localhost:port/?service_name=FREEPDB1
        # To: oracle://system:pass@localhost:port/FREEPDB1
        sqlalchemy_url = oracle_container.get_connection_url()
        connectorx_url = sqlalchemy_url.replace("oracle+oracledb://", "oracle://")
        connectorx_url = connectorx_url.replace("/?service_name=", "/")
        return connectorx_url
    
    pytest.skip("No Oracle database available for tests")

@pytest.fixture(scope="session")
def clickhouse_container() -> Generator[Optional[Any], None, None]:
    """
    Fixture that starts an Clickhouse Database Free container for tests.
    
    This fixture is used at session level to avoid restarting the container
    for each test, which would be very slow.
    """
    # If CLICKHOUSE_URL is already defined, use it (for backward compatibility)
    if os.environ.get("CLICKHOUSE_URL"):
        yield None
        return
    
    # If testcontainers is not available, skip
    if not TESTCONTAINERS_AVAILABLE:
        pytest.skip("testcontainers is not installed. Install with: pip install testcontainers")
    
    # Start Clickhouse container with context manager (recommended)
    with ClickHouseContainer() as clickhouse:
        # Execute initialization script
        init_script = Path(__file__).parent.parent.parent.parent / "scripts" / "clickhouse.sql"
        if init_script.exists():
            try:
                import sqlalchemy

                # HTTP port
                http_port = clickhouse.get_exposed_port(8123)
                # Use connection url with HTTP port
                clickhouse_url = f"clickhouse://test:test@localhost:{http_port}/test"
                # Create sqlalchemy engine
                engine = sqlalchemy.create_engine(clickhouse_url)
                
                # Read SQL script
                with init_script.open('r') as f:
                    sql_content = f.read()
                
                # Remove DROP TABLE statements (tables don't exist yet in fresh container)
                lines = sql_content.split('\n')
                cleaned_lines = [line for line in lines if not line.strip().upper().startswith('DROP TABLE')]
                cleaned_sql = '\n'.join(cleaned_lines)
                
                # Execute using sqlalchemy - split by semicolon
                with engine.begin() as connection:
                    for statement in cleaned_sql.split(';'):
                        statement = statement.strip()
                        if statement:
                            connection.execute(sqlalchemy.text(statement))
                    # Ensure commit happens
                    connection.commit()
            except Exception as e:
                print(f"Warning: Could not initialize database: {e}")
                import traceback
                traceback.print_exc()
        
        os.environ["CLICKHOUSE_URL"] = clickhouse_url
        try:
            yield clickhouse
        finally:
            # Clean up environment variable
            if "CLICKHOUSE_URL" in os.environ:
                del os.environ["CLICKHOUSE_URL"]


@pytest.fixture(scope="module")
def clickhouse_url(clickhouse_container: Optional[Any]) -> str:
    """
    Fixture that returns the Oracle connection URL.
    
    This fixture uses either the testcontainers container or a URL
    defined in the ORACLE_URL environment variable.
    """
    # If CLICKHOUSE_URL is already defined, use it
    if os.environ.get("CLICKHOUSE_URL"):
        return os.environ["CLICKHOUSE_URL"]
    
    # Otherwise, use testcontainers
    if clickhouse_container is not None:
        # Use connection url with HTTP port
        http_port = clickhouse.get_exposed_port(8123)
        clickhouse_url = f"clickhouse://test:test@localhost:{http_port}/test"
        return clickhouse_url
    
    pytest.skip("No Oracle database available for tests")
