pub trait ContextAttachment {}

#[derive(Default)]
pub struct FileContextAttachment {
    pub builtin: bool,
    pub name: String,
    pub path: String,
    pub line_starts: Box<[i32]>,
}

impl ContextAttachment for FileContextAttachment {}
