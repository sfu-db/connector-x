"""Pytest configuration for tests with testcontainers."""
from contextlib import contextmanager
import json
import os
from pathlib import Path
import platform
import sqlite3
import socket
import time
from typing import Generator, Any, Optional
import urllib.error
import urllib.request

import pytest

# Check if Docker is available
try:
    from testcontainers.core.container import DockerContainer
    from testcontainers.oracle import OracleDbContainer
    from testcontainers.clickhouse import ClickHouseContainer
    from testcontainers.postgres import PostgresContainer
    from testcontainers.mysql import MySqlContainer
    from testcontainers.mssql import SqlServerContainer
    from testcontainers.trino import TrinoContainer
    TESTCONTAINERS_AVAILABLE = True
except ImportError:
    TESTCONTAINERS_AVAILABLE = False
    DockerContainer = None
    OracleDbContainer = None  # Avoid type hint errors
    ClickHouseContainer = None
    PostgresContainer = None
    MySqlContainer = None
    SqlServerContainer = None
    TrinoContainer = None


def _is_arm64_host() -> bool:
    return platform.machine().lower() in {"arm64", "aarch64"}


@contextmanager
def _docker_platform_env(platform_value: str) -> Generator[None, None, None]:
    old_value = os.environ.get("DOCKER_DEFAULT_PLATFORM")
    os.environ["DOCKER_DEFAULT_PLATFORM"] = platform_value
    try:
        yield
    finally:
        if old_value is None:
            os.environ.pop("DOCKER_DEFAULT_PLATFORM", None)
        else:
            os.environ["DOCKER_DEFAULT_PLATFORM"] = old_value


@contextmanager
def _docker_amd64_on_arm64() -> Generator[None, None, None]:
    if _is_arm64_host():
        with _docker_platform_env("linux/amd64"):
            yield
    else:
        yield


def _wait_for_tcp(host: str, port: int, timeout_s: float = 120.0, interval_s: float = 0.5) -> None:
    deadline = time.time() + timeout_s
    last_error: Optional[Exception] = None
    while time.time() < deadline:
        try:
            with socket.create_connection((host, int(port)), timeout=2.0):
                return
        except OSError as exc:
            last_error = exc
            time.sleep(interval_s)
    raise TimeoutError(f"TCP endpoint {host}:{port} is not reachable after {timeout_s}s") from last_error


def _wait_for_sqlalchemy_connectivity(db_url: str, probe_sql: str, timeout_s: float = 120.0) -> None:
    import sqlalchemy

    deadline = time.time() + timeout_s
    last_error: Optional[Exception] = None
    while time.time() < deadline:
        try:
            engine = sqlalchemy.create_engine(db_url)
            with engine.connect() as connection:
                connection.execute(sqlalchemy.text(probe_sql))
            return
        except Exception as exc:
            last_error = exc
            time.sleep(1.0)
    raise TimeoutError(f"Database at {db_url} not ready after {timeout_s}s") from last_error


def _wait_for_http_ready(url: str, timeout_s: float = 240.0, interval_s: float = 1.0) -> None:
    deadline = time.time() + timeout_s
    last_error: Optional[Exception] = None
    while time.time() < deadline:
        try:
            request = urllib.request.Request(url, method="GET")
            with urllib.request.urlopen(request, timeout=5) as response:
                status = getattr(response, "status", response.getcode())
                if status == 200:
                    return
        except Exception as exc:
            last_error = exc
            time.sleep(interval_s)
    raise TimeoutError(f"HTTP endpoint {url} is not ready after {timeout_s}s") from last_error


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
        mysql_url = mysql.get_connection_url().replace("localhost", "127.0.0.1")
        host = mysql.get_container_host_ip()
        port = int(mysql.get_exposed_port(3306))
        _wait_for_tcp(host, port, timeout_s=180.0)
        # MySQL init scripts can still be applying right after TCP opens.
        # Keep this lightweight and driver-agnostic (no SQLAlchemy MySQLdb dependency).
        time.sleep(5.0)

        os.environ["MYSQL_URL"] = mysql_url

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


@pytest.fixture(scope="session")
def sqlite_db_session() -> str:
    """
    Fixture that initializes a SQLite database for tests.

    The database is seeded from scripts/sqlite.sql at test time.
    """
    sqlite_url = os.environ.get("SQLITE_URL", "sqlite:///tmp/test.db")
    db_path = sqlite_url.removeprefix("sqlite://")
    if not db_path:
        raise ValueError(f"Invalid SQLITE_URL: {sqlite_url}")

    init_script = Path(__file__).parent.parent.parent.parent / "scripts" / "sqlite.sql"
    if not init_script.exists():
        raise FileNotFoundError(f"SQLite init script not found: {init_script}")

    conn = sqlite3.connect(db_path)
    try:
        conn.executescript(init_script.read_text(encoding="utf-8"))
        conn.commit()
    finally:
        conn.close()

    os.environ["SQLITE_URL"] = sqlite_url
    return sqlite_url


@pytest.fixture(scope="module")
def sqlite_db(sqlite_db_session: str) -> str:
    """Fixture that returns the SQLite connection URL."""
    return sqlite_db_session


@pytest.fixture(scope="session")
def mssql_container() -> Generator[Optional[Any], None, None]:
    """
    Fixture that starts a SQL Server container for tests.

    This fixture is used at session level to avoid restarting the container
    for each test.
    """
    # If MSSQL_URL is already defined, use it (for backward compatibility)
    if os.environ.get("MSSQL_URL"):
        yield None
        return

    # If testcontainers is not available, skip
    if not TESTCONTAINERS_AVAILABLE:
        pytest.skip("testcontainers is not installed. Install with: pip install testcontainers")

    init_script = Path(__file__).parent.parent.parent.parent / "scripts" / "mssql.sql"
    if not init_script.exists():
        raise FileNotFoundError(f"MSSQL init script not found: {init_script}")

    # sqlcmd needs GO batch separator before CREATE FUNCTION in this script.
    mssql_init_script = Path(__file__).with_name("mssql-test.sql")
    mssql_init_script.write_text(
        init_script.read_text(encoding="utf-8").replace(
            "\nCREATE FUNCTION increment", "\nGO\n\nCREATE FUNCTION increment"
        ),
        encoding="utf-8",
    )

    mssql_container = SqlServerContainer(
        image="mcr.microsoft.com/mssql/server:2022-CU12-ubuntu-22.04",
        username="SA",
        password="1Secure*Password1",
        dbname="tempdb",
    ).with_volume_mapping(str(mssql_init_script), "/tmp/mssql.sql", mode="ro")

    with _docker_amd64_on_arm64():
        with mssql_container as mssql:
            status, output = mssql.exec(
                [
                    "bash",
                    "-c",
                    '/opt/mssql-tools*/bin/sqlcmd -S localhost -U "$SQLSERVER_USER" -P "$SA_PASSWORD" -d "$SQLSERVER_DBNAME" -C -b -i /tmp/mssql.sql',
                ]
            )
            if status != 0:
                raise RuntimeError(f"Could not initialize MSSQL database: {output.decode('utf-8', errors='ignore')}")

            os.environ["MSSQL_URL"] = mssql.get_connection_url()

            try:
                yield mssql
            finally:
                if "MSSQL_URL" in os.environ:
                    del os.environ["MSSQL_URL"]
                mssql_init_script.unlink(missing_ok=True)


@pytest.fixture(scope="module")
def mssql_url(mssql_container: Optional[Any]) -> str:
    """
    Fixture that returns the MSSQL connection URL.

    This fixture uses either the testcontainers container or a URL
    defined in the MSSQL_URL environment variable.
    """
    if os.environ.get("MSSQL_URL"):
        return os.environ["MSSQL_URL"]

    if mssql_container is not None:
        return mssql_container.get_connection_url()

    pytest.skip("No MSSQL database available for tests")


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

    def _is_server_starting(error_payload: dict) -> bool:
        if not error_payload:
            return False
        name = str(error_payload.get("errorName", ""))
        message = str(error_payload.get("message", "")).lower()
        return name == "SERVER_STARTING_UP" or "still initializing" in message

    deadline = time.time() + 180.0
    while True:
        try:
            with urllib.request.urlopen(request) as response:
                payload = json.loads(response.read().decode("utf-8"))
        except urllib.error.HTTPError as e:
            if e.code in (503, 502) and time.time() < deadline:
                time.sleep(1.0)
                continue
            raise RuntimeError(f"Trino initialization query failed: {statement}") from e

        error_payload = payload.get("error")
        if error_payload:
            if _is_server_starting(error_payload) and time.time() < deadline:
                time.sleep(1.0)
                continue
            raise RuntimeError(f"Trino initialization query error: {error_payload}")

        next_uri = payload.get("nextUri")
        while next_uri:
            poll_request = urllib.request.Request(next_uri, headers=headers, method="GET")
            with urllib.request.urlopen(poll_request) as poll_response:
                payload = json.loads(poll_response.read().decode("utf-8"))

            error_payload = payload.get("error")
            if error_payload:
                if _is_server_starting(error_payload) and time.time() < deadline:
                    time.sleep(1.0)
                    # restart the statement from scratch while Trino is warming up
                    break
                raise RuntimeError(f"Trino initialization query error: {error_payload}")

            next_uri = payload.get("nextUri")

        if not next_uri:
            return


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

    trino_container = DockerContainer("trinodb/trino:latest")
    trino_container = trino_container.with_volume_mapping(
        str(trino_catalog),
        "/etc/trino/catalog/test.properties",
        mode="ro",
    )
    trino_container = trino_container.with_env("USER", "test")
    trino_container = trino_container.with_exposed_ports(8080)

    init_script = Path(__file__).parent.parent.parent.parent / "scripts" / "trino.sql"

    with trino_container as trino:
        host = trino.get_container_host_ip()
        port = trino.get_exposed_port(8080)
        _wait_for_tcp(host, int(port), timeout_s=240.0)
        _wait_for_http_ready(f"http://{host}:{port}/v1/info", timeout_s=300.0)
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
    
    # Oracle startup is flaky in CI/local Docker; retry full container bootstrap a few times.
    max_attempts = 3
    last_error: Optional[Exception] = None
    for attempt in range(1, max_attempts + 1):
        try:
            with _docker_amd64_on_arm64():
                with OracleDbContainer() as oracle:
                    host = oracle.get_container_host_ip()
                    port = int(oracle.get_exposed_port(1521))
                    _wait_for_tcp(host, port, timeout_s=240.0)

                    # Execute initialization script
                    init_script = Path(__file__).parent.parent.parent.parent / "scripts" / "oracle.sql"
                    if init_script.exists():
                        import sqlalchemy

                        sqlalchemy_url = oracle.get_connection_url()
                        _wait_for_sqlalchemy_connectivity(sqlalchemy_url, "SELECT 1 FROM dual", timeout_s=240.0)
                        engine = sqlalchemy.create_engine(sqlalchemy_url)

                        # Read SQL script
                        with init_script.open("r", encoding="utf-8") as f:
                            sql_content = f.read()

                        # Remove DROP TABLE statements (tables don't exist yet in fresh container)
                        lines = sql_content.split("\n")
                        cleaned_lines = [
                            line
                            for line in lines
                            if not line.strip().upper().startswith("DROP TABLE")
                        ]
                        cleaned_sql = "\n".join(cleaned_lines)

                        # Execute using sqlalchemy - split by semicolon
                        with engine.begin() as connection:
                            for statement in cleaned_sql.split(";"):
                                statement = statement.strip()
                                if statement:
                                    connection.execute(sqlalchemy.text(statement))
                            # Ensure commit happens
                            connection.commit()

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
                        if "ORACLE_URL" in os.environ:
                            del os.environ["ORACLE_URL"]
                    return
        except Exception as exc:
            last_error = exc
            if attempt == max_attempts:
                raise
            time.sleep(5.0)

    # Unreachable in practice, but keeps type checkers happy.
    if last_error is not None:
        raise last_error


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
