// https://github.com/PyO3/pyo3-built/issues/21

fn main() {
    // let src = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // println!("src: {}", src);
    // let dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("built.rs");
    // let mut opts = built::Options::default();
    // println!("out: {:?}", dst);
    // opts.set_dependencies(true).set_compiler(true).set_env(true);

    // built::write_built_file_with_opts(&opts, std::path::Path::new(&src), &dst)
    //     .expect("Failed to acquire build-time information");
    pyo3_build_config::use_pyo3_cfgs();
}
