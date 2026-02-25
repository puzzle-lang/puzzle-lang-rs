use crate::cli_error;
use crate::error::cli_error;
use puzzle_core::options::CliOptions;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{LazyLock, RwLockWriteGuard};

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
        let key = option
            .map(|&it| it)
            .unwrap_or_else(|| cli_error!("未知选项: {}", arg));
        if used_arg_types.contains(key) {
            cli_error!("重复的选项: {}", key)
        }
        used_arg_types.insert(key);
        let value = arg.strip_prefix(&format!("{}=", key)).unwrap();
        if value.chars().any(|c| c.is_whitespace()) {
            cli_error!("参数 {} 中不允许出现空白字符", arg)
        }
        if value.is_empty() {
            cli_error!("参数 {}=<option1,option2,...> 缺少值", key)
        }
        AVAILABLE_ARG_PARSE_MAP[key](value.trim());
    });
    check_args();
}

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

fn check_args() {
    if !CliOptions::is_exists_project_path() {
        cli_error!("缺少 {} 参数", KEY_PATH)
    }
}

fn parse_path_option(project_path: &str) {
    let mut options = CliOptions::write_cli_options();
    let mut project_path = PathBuf::from(project_path);
    if !(project_path.exists() && project_path.is_dir()) {
        cli_error!("{:?} 项目不存在", project_path.file_name().unwrap());
    }
    if !project_path.is_absolute() {
        project_path = project_path.canonicalize().unwrap();
    }
    options.project_path = project_path.into();
}

static OPTION_ANSI_COLOR: &str = "ensi-color";

fn parse_features_options(value: &str) {
    with_bool_options(
        value,
        KEY_FEATURES,
        &[OPTION_ANSI_COLOR],
        |options, enables| {
            options.enable_ansi_color = enables[0];
        },
    )
}

static OPTION_AST: &str = "ast";

fn parse_exports_options(value: &str) {
    with_bool_options(value, KEY_EXPORTS, &[OPTION_AST], |options, enables| {
        options.enable_export_ast = enables[0];
    })
}

static OPTION_PROGRESS: &str = "progress";
static OPTION_IGNORE: &str = "ignore";
static OPTION_FILE: &str = "file";

fn parse_infos_options(value: &str) {
    with_bool_options(
        value,
        KEY_INFOS,
        &[OPTION_PROGRESS, OPTION_IGNORE, OPTION_FILE],
        |options, enables| {
            options.enable_info_progress = enables[0];
            options.enable_info_ignore = enables[1];
            options.enable_info_file = enables[2];
        },
    )
}

fn with_bool_options(
    value: &str,
    key: &str,
    available_options: &[&str],
    on_action: fn(options: &mut RwLockWriteGuard<CliOptions>, enables: Vec<bool>),
) {
    let mut options = CliOptions::write_cli_options();
    let len = available_options.len();
    match value {
        "all" => return on_action(&mut options, vec![true; len]),
        "none" => return,
        _ => {}
    }

    let mut enables = vec![false; len];
    value.split(',').enumerate().for_each(|(idx, v)| {
        if !available_options.contains(&v) {
            cli_error!("{}={} 不可用的值", key, v)
        }
        if enables[idx] {
            cli_error!("{}={} 重复的值", key, v)
        } else {
            enables[idx] = true;
        }
    });

    on_action(&mut options, enables);
}
