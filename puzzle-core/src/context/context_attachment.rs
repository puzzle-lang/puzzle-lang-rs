use std::any::Any;

pub trait ContextAttachment {
    fn as_any(&self) -> &dyn Any;
}

pub struct FileContextAttachment {
    pub builtin: bool,
    pub name: String,
    pub path: String,
    pub line_starts: Box<[i32]>,
}

impl ContextAttachment for FileContextAttachment {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
