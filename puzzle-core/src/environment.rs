use std::sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Default)]
pub struct Environment {
    pub project_path: Option<String>,
    pub enable_export_ast: bool,
    pub enable_ansi_color: bool,
    pub enable_stack_trace: bool,
    pub enable_info_progress: bool,
    pub enable_info_ignore: bool,
    pub enable_info_file: bool,
}

static ENVIRONMENT: LazyLock<RwLock<Environment>> =
    LazyLock::new(|| RwLock::new(Environment::default()));

pub fn read_env() -> RwLockReadGuard<'static, Environment> {
    ENVIRONMENT.read().unwrap()
}

pub fn write_env() -> RwLockWriteGuard<'static, Environment> {
    ENVIRONMENT.write().unwrap()
}
