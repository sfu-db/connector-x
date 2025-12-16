"""Pytest configuration for tests with testcontainers."""
import os
from pathlib import Path
from typing import Generator, Any, Optional

import pytest

# Check if Docker is available
try:
    from testcontainers.oracle import OracleDbContainer
    TESTCONTAINERS_AVAILABLE = True
except ImportError:
    TESTCONTAINERS_AVAILABLE = False
    OracleDbContainer = None  # Avoid type hint errors


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
