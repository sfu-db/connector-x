# from distributed import Client
# client = Client(n_workers=5)
import time
import modin.pandas as pd
from sqlalchemy import create_engine
from keys import *

if __name__ == '__main__':
    # client = Client(processes=False)
    # engine = create_engine('postgresql://postgres:postgres@localhost:6666/tpch')
    # conn = engine.connect()
    then = time.time()
    # df = pd.read_sql("select * from lineitem_s10;", conn)
    df = pd.read_sql("select * from lineitem_s10", PG_CONN)
    print("time", time.time() - then)
    conn.close()
    print(df)

# import io
# import time
# import modin.pandas as pd
# from sqlalchemy import create_engine

# if __name__ == '__main__':
#     engine = create_engine('postgresql://postgres:postgres@localhost:6666/tpch')
#     conn = engine.connect()
#     cur = conn.connection.cursor()
#     store = io.BytesIO()
#     then = time.time()
#     cur.copy_expert("COPY (select * from lineitem) TO STDOUT WITH CSV HEADER;", store)
#     print("finish copy", time.time() - then)
#     store.seek(0)
#     df = pd.read_csv(store)
#     print("finish read_csv", time.time() - then)
#     conn.close()
#     print(df)

