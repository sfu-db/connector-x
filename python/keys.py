import numpy as np
from typing import Any, List


KEYS = [
    "raw/book.BTC-PERPETUAL.raw/[2020-11-30 21:45:06.698628639 UTC]~[2020-11-30 23:55:00.711449067 UTC].first.json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-11-30 23:55:00.711458648 UTC]~[2020-12-01 01:20:00.753265335 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-01 01:20:00.753274983 UTC]~[2020-12-01 03:15:00.733890875 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-01 03:15:00.733900406 UTC]~[2020-12-01 05:12:00.723458713 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-01 05:12:00.723468470 UTC]~[2020-12-01 07:37:00.735621952 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-01 07:37:00.735632789 UTC]~[2020-12-01 09:54:00.722288899 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-01 09:54:00.722299282 UTC]~[2020-12-01 11:21:39.762742203 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-04 16:03:46.574615522 UTC]~[2020-12-04 18:14:46.854636335 UTC].first.json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-04 18:14:46.854646284 UTC]~[2020-12-04 21:10:41.277542212 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-04 21:10:41.277552033 UTC]~[2020-12-04 23:01:41.319182604 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-04 23:01:41.319193059 UTC]~[2020-12-05 00:46:41.281435066 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-05 00:46:41.281444155 UTC]~[2020-12-05 03:15:41.287697846 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-05 03:15:41.287708469 UTC]~[2020-12-05 06:04:41.285495754 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-05 06:04:41.285505733 UTC]~[2020-12-05 09:21:41.282337073 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-05 09:21:41.282348833 UTC]~[2020-12-05 12:10:41.278671757 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-05 12:10:41.278682395 UTC]~[2020-12-05 14:57:41.297445131 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-05 14:57:41.297455495 UTC]~[2020-12-05 17:56:41.657608470 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-05 17:56:41.657617626 UTC]~[2020-12-05 21:35:41.294855534 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-05 21:35:41.294866618 UTC]~[2020-12-06 00:54:41.276280248 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-06 00:54:41.276289899 UTC]~[2020-12-06 04:03:41.282999733 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-06 04:03:41.283009294 UTC]~[2020-12-06 08:09:41.281373861 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-06 08:09:41.281384232 UTC]~[2020-12-06 11:06:41.277674127 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-06 11:06:41.277684504 UTC]~[2020-12-06 14:16:46.923793757 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-06 14:16:46.923804754 UTC]~[2020-12-06 17:05:41.671974748 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-06 17:05:41.671985265 UTC]~[2020-12-06 21:16:41.425760259 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-06 21:16:41.425769436 UTC]~[2020-12-06 23:52:41.418770349 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-06 23:52:41.418779638 UTC]~[2020-12-07 02:27:41.282984378 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-07 02:27:41.282994339 UTC]~[2020-12-07 06:16:41.282845434 UTC].json.gz",
    "raw/book.BTC-PERPETUAL.raw/[2020-12-07 06:16:41.282857531 UTC]~[2020-12-07 07:20:52.909281760 UTC].json.gz",
]
BUCKET = "dovahcrow"

PG_CONN = "postgresql://postgres:postgres@localhost:6666/tpch"

def get_sqls(count: int) -> List[str]:
    sqls = []
    split = np.linspace(0, 60000000, num=count+1, endpoint=True, dtype=int)
    for i in range(len(split)-1):
        sqls.append(f"select * from lineitem_s10 where l_orderkey > {split[i]} and l_orderkey <= {split[i+1]}")
    return sqls

# def get_sqls(count: int) -> List[str]:
#     sqls = []
#     split = np.linspace(0, 6000000, num=count+1, endpoint=True, dtype=int)
#     for i in range(len(split)-1):
#         sqls.append(f"select * from lineitem where l_orderkey > {split[i]} and l_orderkey <= {split[i+1]}")
#     return sqls


