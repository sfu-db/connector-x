import os

import pandas as pd
import pytest
from pandas.testing import assert_frame_equal

from .. import read_sql

@pytest.fixture(scope="module")  # type: ignore
def mysql_url() -> str:
    conn = os.environ["ORACLE_URL"]
    return conn