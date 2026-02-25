use puzzle_core::error::pzl_error_impl;

#[macro_export]
macro_rules! cli_error {
    ($($arg:tt)+) => {{
        cli_error(&format!($($arg)+))
    }};
}

#[inline]
pub fn cli_error(msg: &str) -> ! {
    pzl_error_impl("命令", None, None, msg)
}
