use crate::context::context_attachment::FileContextAttachment;
use std::process::exit;

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
    let position = if let Some(attachment) = attachment {
        format!("\n位置: {}", attachment.path)
    } else {
        String::new()
    };
    eprintln!("[{}错误] {}{}", kind, msg, position);
    exit(1)
}
