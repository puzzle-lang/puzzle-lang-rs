use crate::context::context::{
    Context, ContextAttachmentMap, ContextRef, ContextWeak, HasChildren, HasParent,
};
use crate::context::module::ModuleContext;
use crate::context::root::RootContext;
use crate::extension::OptionExt;
use crate::impl_context_attachment;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct ProjectContext {
    parent: ContextWeak<RootContext>,
    attachments: ContextAttachmentMap<Self>,
    children: Option<Vec<ContextRef<ModuleContext>>>,
}

pub struct ProjectAttachment {
    pub name: String,
    pub path: PathBuf,
}

impl_context_attachment!(ProjectAttachment, ProjectContext);

impl ProjectContext {
    pub fn new(parent: ContextWeak<RootContext>) -> Self {
        ProjectContext {
            parent,
            attachments: HashMap::default(),
            children: Option::default(),
        }
    }
}

impl Context for ProjectContext {
    fn attachments(&self) -> &ContextAttachmentMap<Self> {
        &self.attachments
    }
    fn attachments_mut(&mut self) -> &mut ContextAttachmentMap<Self> {
        &mut self.attachments
    }
}

impl HasChildren for ProjectContext {
    type Child = ModuleContext;

    fn set_children(&mut self, children: Vec<ContextRef<Self::Child>>) {
        self.children = children.some();
    }

    fn get_children(&self) -> &Vec<ContextRef<Self::Child>> {
        self.children.as_ref().unwrap()
    }
}

impl HasParent for ProjectContext {
    type Parent = RootContext;

    fn parent(&self) -> Arc<RwLock<Self::Parent>> {
        self.parent.upgrade().unwrap()
    }
}
