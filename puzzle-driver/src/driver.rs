use crate::collect::collect_sources;
use puzzle_core::options::CliOptions;
use puzzle_core::time::{measure_time, measure_timed_value};

pub fn start_building() {
    let duration = measure_time(compile_frontend);
    compile_frontend();
    if CliOptions::enable_info_progress() {
        println!("执行用时: {:?}", duration);
    }
}

fn compile_frontend() {
    let project_path = CliOptions::project_path();
    let sources_value = measure_timed_value(|| collect_sources(&project_path));
    if CliOptions::enable_info_progress() {
        println!("项目源收集用时: {:?}", sources_value.duration)
    }
}
