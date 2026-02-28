use crate::context::context::{
    Context, ContextAttachmentMap, ContextRef, ContextWeak, HasChildren, HasParent,
};
use crate::context::file::FileContext;
use crate::context::project::ProjectContext;
use crate::extension::OptionExt;
use crate::impl_context_attachment;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct ModuleContext {
    parent: ContextWeak<ProjectContext>,
    attachments: ContextAttachmentMap<Self>,
    children: Option<Vec<ContextRef<FileContext>>>,
}

pub struct ModuleAttachment {
    pub name: String,
    pub group: Vec<String>,
}

impl_context_attachment!(ModuleAttachment, ModuleContext);

impl ModuleContext {
    pub fn new(parent: ContextWeak<ProjectContext>) -> Self {
        ModuleContext {
            parent,
            attachments: HashMap::default(),
            children: Option::default(),
        }
    }
}

impl Context for ModuleContext {
    fn attachments(&self) -> &ContextAttachmentMap<Self> {
        &self.attachments
    }

    fn attachments_mut(&mut self) -> &mut ContextAttachmentMap<Self> {
        &mut self.attachments
    }
}

impl HasChildren for ModuleContext {
    type Child = FileContext;

    fn set_children(&mut self, children: Vec<ContextRef<Self::Child>>) {
        self.children = children.some()
    }

    fn get_children(&self) -> &Vec<ContextRef<Self::Child>> {
        self.children.as_ref().unwrap()
    }
}

impl HasParent for ModuleContext {
    type Parent = ProjectContext;

    fn parent(&self) -> Arc<RwLock<Self::Parent>> {
        self.parent.upgrade().unwrap()
    }
}
