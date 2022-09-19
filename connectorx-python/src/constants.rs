// PyString buffer size in MB
pub const PYSTRING_BUFFER_SIZE: usize = 4;

#[cfg(not(debug_assertions))]
pub const J4RS_BASE_PATH: &str = "./target/release";
#[cfg(debug_assertions)]
pub const J4RS_BASE_PATH: &str = "./target/debug";
