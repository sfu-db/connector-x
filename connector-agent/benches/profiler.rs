use criterion::profiler::Profiler;
use pprof::ProfilerGuard;
use std::{fs::File, path::Path};

pub struct FlamegraphProfiler<'a> {
    frequency: i32,
    active_profiler: Option<ProfilerGuard<'a>>,
}

impl<'a> FlamegraphProfiler<'a> {
    #[allow(dead_code)]
    pub fn new(frequency: i32) -> Self {
        FlamegraphProfiler {
            frequency,
            active_profiler: None,
        }
    }
}

impl<'a> Profiler for FlamegraphProfiler<'a> {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        self.active_profiler = Some(ProfilerGuard::new(self.frequency).unwrap());
    }

    fn stop_profiling(&mut self, _benchmark_id: &str, benchmark_dir: &Path) {
        std::fs::create_dir_all(benchmark_dir).unwrap();
        let flamegraph_path = benchmark_dir.join("flamegraph.svg");
        let flamegraph_file = File::create(&flamegraph_path)
            .expect("File system error while creating flamegraph.svg");
        if let Some(profiler) = self.active_profiler.take() {
            profiler
                .report()
                .build()
                .unwrap()
                .flamegraph(flamegraph_file)
                .expect("Error writing flamegraph");
        }
    }
}
