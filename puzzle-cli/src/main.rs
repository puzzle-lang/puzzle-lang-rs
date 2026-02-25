mod info;
mod parse;
mod error;

use crate::info::{help, unknown, version};
use crate::parse::parse_cli_args;
use puzzle_driver::driver::start_building;
use std::env::args;

fn main() {
    let args = args().collect::<Vec<String>>();
    dispatch_cli_command(args);
}

/// 调度CLI命令
fn dispatch_cli_command(args: Vec<String>) {
    let Some(command) = args.get(1) else {
        return help();
    };
    match command.as_str() {
        "build" => build(&args[2..]),
        "help" => help(),
        "version" => version(),
        _ => unknown(),
    }
}

/// 构建
fn build(args: &[String]) {
    parse_cli_args(args);
    start_building();
}
