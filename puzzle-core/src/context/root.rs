use crate::context::context::{
    context_ref, Context, ContextAttachmentMap, ContextRef, ContextWeak, HasChildren,
};
use crate::context::project::ProjectContext;
use crate::extension::OptionExt;
use crate::impl_context_attachment;
use std::sync::{Arc, LazyLock, RwLockReadGuard, RwLockWriteGuard};

static ROOT_CONTEXT: LazyLock<ContextRef<RootContext>> =
    LazyLock::new(|| context_ref(RootContext::default()));

pub fn read_root_context() -> RwLockReadGuard<'static, RootContext> {
    ROOT_CONTEXT.read().unwrap()
}

pub fn write_root_context() -> RwLockWriteGuard<'static, RootContext> {
    ROOT_CONTEXT.write().unwrap()
}

pub fn weak_root_context() -> ContextWeak<RootContext> {
    Arc::downgrade(&ROOT_CONTEXT)
}

#[derive(Default)]
pub struct RootContext {
    attachments: ContextAttachmentMap<Self>,
    children: Option<Vec<ContextRef<ProjectContext>>>,
}

pub struct RootAttachment {
    pub max_path_length: usize,
}

impl_context_attachment!(RootAttachment, RootContext);

impl Context for RootContext {
    fn attachments(&self) -> &ContextAttachmentMap<Self> {
        &self.attachments
    }
    fn attachments_mut(&mut self) -> &mut ContextAttachmentMap<Self> {
        &mut self.attachments
    }
}

impl HasChildren for RootContext {
    type Child = ProjectContext;

    fn set_children(&mut self, children: Vec<ContextRef<Self::Child>>) {
        self.children = children.some()
    }

    fn get_children(&self) -> &Vec<ContextRef<Self::Child>> {
        self.children.as_ref().unwrap()
    }
}
