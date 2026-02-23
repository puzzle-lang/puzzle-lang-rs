use puzzle_core::environment::{read_env, write_env, Environment};
use puzzle_core::error::cli_error;
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, RwLockWriteGuard};

static KEY_PATH: &str = "--path";
static KEY_FEATURES: &str = "--features";
static KEY_EXPORTS: &str = "--exports";
static KEY_INFOS: &str = "--infos";

static AVAILABLE_ARG_PARSE_MAP: LazyLock<HashMap<&str, fn(&str)>> = LazyLock::new(|| {
    HashMap::from([
        (KEY_PATH, parse_path_option as fn(&str)),
        (KEY_FEATURES, parse_features_options),
        (KEY_EXPORTS, parse_exports_options),
        (KEY_INFOS, parse_infos_options),
    ])
});

pub fn parse_cli_args(args: &[String]) {
    let mut used_arg_types = HashSet::new();
    let arg_keys = AVAILABLE_ARG_PARSE_MAP
        .keys()
        .copied()
        .collect::<HashSet<_>>();
    args.into_iter().for_each(|arg| {
        let option = arg_keys
            .iter()
            .find(|&&key| arg.starts_with(&format!("{}=", key)));
        let key = match option {
            None => cli_error(&format!("未知选项: {}", arg)),
            Some(&key) => key,
        };
        if used_arg_types.contains(key) {
            cli_error(&format!("重复的选项: {}", key))
        }
        used_arg_types.insert(key);
        let value = arg.strip_prefix(&format!("{}=", key)).unwrap();
        if value.chars().any(|c| c.is_whitespace()) {
            cli_error(&format!("参数 {} 中不允许出现空白字符", arg))
        }
        if value.is_empty() {
            cli_error(&format!("参数 {}=<option1,option2,...> 缺少值", key))
        }
        AVAILABLE_ARG_PARSE_MAP[key](value.trim());
    });
    check_args();
}

fn check_args() {
    let env = read_env();
    if env.project_path.is_none() {
        cli_error(&format!("缺少 {} 参数", KEY_PATH))
    }
}

fn parse_path_option(project_path: &str) {
    let mut env = write_env();
    env.project_path = project_path.to_string().into();
}

static OPTION_ANSI_COLOR: &str = "ensi-color";

fn parse_features_options(value: &str) {
    parse_and_check_options(
        value,
        KEY_FEATURES,
        [OPTION_ANSI_COLOR].into_iter().collect(),
        |env| {
            env.enable_ansi_color = true;
            env.enable_stack_trace = true;
        },
        |env, values| {
            env.enable_ansi_color = values.contains(OPTION_ANSI_COLOR);
        },
    )
}

static OPTION_AST: &str = "ast";

fn parse_exports_options(value: &str) {
    parse_and_check_options(
        value,
        KEY_EXPORTS,
        [OPTION_AST].into_iter().collect(),
        |env| {
            env.enable_export_ast = true;
        },
        |env, values| {
            env.enable_export_ast = values.contains(OPTION_AST);
        },
    )
}

static OPTION_PROGRESS: &str = "progress";
static OPTION_IGNORE: &str = "ignore";
static OPTION_FILE: &str = "file";

fn parse_infos_options(value: &str) {
    parse_and_check_options(
        value,
        KEY_INFOS,
        [OPTION_PROGRESS, OPTION_IGNORE, OPTION_FILE]
            .into_iter()
            .collect(),
        |env| {
            env.enable_info_progress = true;
            env.enable_info_ignore = true;
            env.enable_info_file = true;
        },
        |env, values| {
            env.enable_info_progress = values.contains(OPTION_PROGRESS);
            env.enable_info_ignore = values.contains(OPTION_IGNORE);
            env.enable_info_file = values.contains(OPTION_FILE);
        },
    )
}

fn parse_and_check_options(
    value: &str,
    key: &str,
    available_options: HashSet<&str>,
    on_all: fn(env: &mut RwLockWriteGuard<Environment>),
    on_action: fn(env: &mut RwLockWriteGuard<Environment>, values: HashSet<&str>),
) {
    let mut env = write_env();
    match value {
        "all" => return on_all(&mut env),
        "none" => return,
        _ => {}
    }
    let mut residues = available_options.clone();
    let values = value.split(',').collect::<Vec<_>>();
    values.iter().for_each(|v| {
        if !available_options.contains(v) {
            cli_error(&format!("{}={} 不可用的值", key, v))
        }
        if residues.contains(v) {
            residues.remove(v);
        } else {
            cli_error(&format!("{}={} 重复的值", key, v))
        }
    });
    on_action(&mut env, values.into_iter().collect());
}
