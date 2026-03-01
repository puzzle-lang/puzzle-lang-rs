use crate::context::context::{Context, ContextAttachmentMap, ContextWeak, HasParent};
use crate::context::module::ModuleContext;
use crate::impl_context_attachment;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct FileContext {
    parent: ContextWeak<ModuleContext>,
    attachments: ContextAttachmentMap<Self>,
}

pub struct FileAttachment {
    pub builtin: bool,
    pub name: String,
    pub path: PathBuf,
    pub line_starts: Option<Box<[u32]>>,
}

impl_context_attachment!(FileAttachment, FileContext);

impl FileContext {
    pub fn new(parent: ContextWeak<ModuleContext>) -> Self {
        Self {
            parent,
            attachments: HashMap::default(),
        }
    }
}

impl Context for FileContext {
    fn attachments(&self) -> &ContextAttachmentMap<Self> {
        &self.attachments
    }
    fn attachments_mut(&mut self) -> &mut ContextAttachmentMap<Self> {
        &mut self.attachments
    }
}

impl HasParent for FileContext {
    type Parent = ModuleContext;

    fn parent(&self) -> Arc<RwLock<Self::Parent>> {
        self.parent.upgrade().unwrap()
    }
}
