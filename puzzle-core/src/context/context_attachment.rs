use std::path::PathBuf;

pub trait ContextAttachment {}

#[derive(Default)]
pub struct FileContextAttachment {
    pub builtin: bool,
    pub name: String,
    pub path: PathBuf,
    pub line_starts: Box<[u32]>,
}

impl ContextAttachment for FileContextAttachment {}
