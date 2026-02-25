use puzzle_core::error::pzl_error_impl;
use std::path::PathBuf;

#[macro_export]
macro_rules! config_error {
    ($path: expr, $($arg:tt)+) => {{
        config_error($path, &format!($($arg)+))
    }};
}

#[inline]
pub fn config_error(path: Option<&PathBuf>, msg: &str) -> ! {
    pzl_error_impl("配置", path, None, msg)
}
