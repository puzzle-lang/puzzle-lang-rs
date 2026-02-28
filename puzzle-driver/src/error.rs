use puzzle_core::error::pzl_error_impl;
use puzzle_core::extension::OptionExt;
use std::path::PathBuf;

#[macro_export]
macro_rules! config_error {
    ($path: expr, $($arg:tt)+) => {{
        use crate::error::config_error;
        
        config_error($path, &format!($($arg)+))
    }};
}

#[inline]
pub fn config_error(path: &PathBuf, msg: &str) -> ! {
    pzl_error_impl("配置", path.some(), None, msg)
}
