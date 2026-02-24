use std::path::PathBuf;
use std::sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// CLI 参数
#[derive(Default, Debug)]
pub struct CliOptions {
    pub project_path: Option<PathBuf>,
    pub enable_export_ast: bool,
    pub enable_ansi_color: bool,
    pub enable_info_progress: bool,
    pub enable_info_ignore: bool,
    pub enable_info_file: bool,
}

static CLI_OPTIONS: LazyLock<RwLock<CliOptions>> =
    LazyLock::new(|| RwLock::new(CliOptions::default()));

impl CliOptions {
    pub fn read_cli_options() -> RwLockReadGuard<'static, CliOptions> {
        CLI_OPTIONS.read().unwrap()
    }

    pub fn write_cli_options() -> RwLockWriteGuard<'static, CliOptions> {
        CLI_OPTIONS.write().unwrap()
    }

    #[inline]
    #[must_use]
    pub fn is_exists_project_path() -> bool {
        Self::read_cli_options().project_path.is_some()
    }

    #[must_use]
    pub fn project_path() -> PathBuf {
        Self::read_cli_options().project_path.clone().unwrap()
    }

    #[inline]
    pub fn enable_export_ast() -> bool {
        Self::read_cli_options().enable_export_ast
    }

    #[inline]
    pub fn enable_ansi_color() -> bool {
        Self::read_cli_options().enable_ansi_color
    }

    #[inline]
    pub fn enable_info_progress() -> bool {
        Self::read_cli_options().enable_info_progress
    }

    #[inline]
    pub fn enable_info_ignore() -> bool {
        Self::read_cli_options().enable_info_ignore
    }

    #[inline]
    pub fn enable_info_file() -> bool {
        Self::read_cli_options().enable_info_file
    }
}
