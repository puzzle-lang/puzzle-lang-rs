use crate::collect::collect_sources;
use puzzle_core::options::CliOptions;
use puzzle_core::time::measure_time;

pub fn start_building() {
    let duration = measure_time(compile_frontend);
    if CliOptions::enable_info_progress() {
        println!("执行用时: {:?}", duration);
    }
}

fn compile_frontend() {
    let project_path = CliOptions::project_path();
    collect_sources(&project_path);
}
