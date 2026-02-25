use crate::context::context::{Context, FileContext};
use std::path::PathBuf;

pub trait ContextAttachment<C: Context + ?Sized> {}

#[derive(Default)]
pub struct FileContextAttachment {
    pub builtin: bool,
    pub name: String,
    pub path: PathBuf,
    pub line_starts: Box<[u32]>,
}

impl ContextAttachment<FileContext> for FileContextAttachment {}
