build-release:
    cargo build --release

build-debug:
    cargo build

build-rust: build-release
    cp target/release/libconnector_agent_python.so python/connector_agent.so

build-rust-debug: build-debug
    cp target/debug/libconnector_agent_python.so python/connector_agent.so

test-pandas:
    python python/test_pandas.py

dask:
    python python/multiprocessing_dask.py

pandas:
    python python/multiprocessing_pandas.py

pandas-single:
    python python/singlethread_pandas.py

pyarrow:
    python python/multiprocessing_pyarrow.py

arrowfs:
    python python/arrowfs.py

rust:
    python python/rust.py
 
pg_pandas:
    python python/pg_pandas.py

pg_copy:
    python python/pg_copy.py

pg_multi_copy t_num:
    python python/pg_multi_copy.py {{t_num}}

pg_pyarrow:
    python python/pg_pyarrow.py

pg_multi_pyarrow t_num:
    python python/pg_multi_pyarrow.py {{t_num}}

pg_multi_pyarrow_thread t_num:
    python python/pg_multi_pyarrow_thread.py {{t_num}}

pg_multi_pyarrow2 t_num:
    python python/pg_multi_pyarrow2.py {{t_num}}

pg_multi_pyarrow3 t_num:
    python python/pg_multi_pyarrow3.py {{t_num}}

pg_multi_pyarrow4 t_num:
    python python/pg_multi_pyarrow4.py {{t_num}}   

pg_multi_pyarrow5 t_num:
    python python/pg_multi_pyarrow5.py {{t_num}}      

pg_rust t_num:
    python python/pg_rust.py {{t_num}}

pg_modin:
    python python/pg_modin.py