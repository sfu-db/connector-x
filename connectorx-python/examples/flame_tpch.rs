mod tpch;

use pprof::protos::Message;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let guard = pprof::ProfilerGuard::new(10).unwrap();

    tpch::run(10, &args[1]);

    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();

        let mut file = File::create("profile.pb").unwrap();
        let profile = report.pprof().unwrap();

        let mut content = Vec::new();
        profile.encode(&mut content).unwrap();
        file.write_all(&content).unwrap();
    };
}
