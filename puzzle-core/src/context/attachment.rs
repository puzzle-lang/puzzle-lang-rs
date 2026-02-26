use crate::context::context::{Context, FileContext, ModuleContext, ProjectContext};
use std::any::Any;
use std::path::PathBuf;

pub trait ContextAttachment<C: Context + ?Sized>: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

#[macro_export]
macro_rules! impl_context_attachment {
    ($ca:ty, $c:ty) => {
        impl ContextAttachment<$c> for $ca {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}

#[derive(Default)]
pub struct ProjectAttachment {
    pub name: String,
    pub path: PathBuf,
}

impl_context_attachment!(ProjectAttachment, ProjectContext);

#[derive(Default)]
pub struct ModuleAttachment {
    pub name: String,
    pub group: Vec<String>,
}

impl_context_attachment!(ModuleAttachment, ModuleContext);

pub struct FileAttachment {
    pub builtin: bool,
    pub name: String,
    pub path: PathBuf,
    pub line_starts: Option<Box<[u32]>>,
}

impl_context_attachment!(FileAttachment, FileContext);
