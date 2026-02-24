use crate::context::context_attachment::FileContextAttachment;
use std::process::exit;

#[macro_export]
macro_rules! cli_error {
    ($($arg:tt)+) => {{
        cli_error(&format!($($arg)+))
    }};
}

#[macro_export]
macro_rules! config_error {
    ($($arg:tt)+) => {{
        config_error(&format!($($arg)+))
    }};
}

#[macro_export]
macro_rules! lex_error {
    ($attachment: expr, $($arg:tt)+) => {{
        lex_error($attachment, &format!($($arg)+))
    }};
}

#[macro_export]
macro_rules! syntax_error {
    ($attachment: expr, $($arg:tt)+) => {{
        syntax_error($attachment, &format!($($arg)+))
    }};
}

#[macro_export]
macro_rules! sema_error {
    ($attachment: expr, $($arg:tt)+) => {{
        sema_error($attachment, &format!($($arg)+))
    }};
}

pub fn cli_error(msg: &str) -> ! {
    pzl_error_impl(None, "命令", msg)
}

pub fn config_error(msg: &str) -> ! {
    pzl_error_impl(None, "配置", msg)
}

pub fn lex_error(attachment: Option<FileContextAttachment>, msg: &str) -> ! {
    pzl_error_impl(attachment, "词法", msg)
}

pub fn syntax_error(attachment: Option<FileContextAttachment>, msg: &str) -> ! {
    pzl_error_impl(attachment, "语法", msg)
}

pub fn sema_error(attachment: Option<FileContextAttachment>, msg: &str) -> ! {
    pzl_error_impl(attachment, "语义", msg)
}

fn pzl_error_impl(attachment: Option<FileContextAttachment>, kind: &str, msg: &str) -> ! {
    let position = attachment
        .map(|it| format!("\n位置: {}", it.path))
        .unwrap_or_default();
    eprintln!("[{}错误] {}{}", kind, msg, position);
    exit(1)
}
