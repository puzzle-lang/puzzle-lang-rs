#[macro_export]
macro_rules! ansi_println {
    ($($arg:tt)*) => {{
        let str = format!($($arg)*);
        println!("{}{}", str, Ansi::Reset);
    }};
}
