use crate::ansi::ansi::Ansi;
use crate::ansi_println;
use crate::extension::PathBufExt;
use crate::location::SourcePosition;
use std::path::PathBuf;
use std::process::exit;

pub fn pzl_error_impl(
    kind: &str,
    path: Option<&PathBuf>,
    position: Option<SourcePosition>,
    msg: &str,
) -> ! {
    let position = match (path, position) {
        (Some(path), Some(position)) => format!(
            "\n位置: {}:{}:{}",
            path.file_name_string(),
            position.line,
            position.column
        ),
        (Some(path), None) => format!("\n位置: {}", path.file_name_string()),
        (_, _) => String::new(),
    };
    ansi_println!("{}[{}错误] {}{}", Ansi::Red, kind, msg, position);
    exit(1)
}
