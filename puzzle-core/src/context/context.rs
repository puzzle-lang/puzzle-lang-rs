use crate::context::attachment::ContextAttachment;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type ContextAttachmentMap<C: Context> = HashMap<TypeId, Box<dyn ContextAttachment<C>>>;

pub trait Context
where
    Self: 'static,
{
    fn attachments(&self) -> &ContextAttachmentMap<Self>;

    fn attachments_mut(&mut self) -> &mut ContextAttachmentMap<Self>;

    fn insert<CA: ContextAttachment<Self> + 'static>(&mut self, value: CA) {
        let id = TypeId::of::<CA>();
        self.attachments_mut().insert(id, Box::new(value));
    }

    fn get<CA: ContextAttachment<Self> + 'static>(&self) -> &CA {
        let id = TypeId::of::<CA>();
        let boxed = &self.attachments()[&id] as &dyn Any;
        boxed.downcast_ref::<CA>().unwrap()
    }
}

static ROOT_CONTEXT: LazyLock<RwLock<RootContext>> =
    LazyLock::new(|| RwLock::new(RootContext::default()));

pub fn read_root_context() -> RwLockReadGuard<'static, RootContext> {
    ROOT_CONTEXT.read().unwrap()
}

pub fn write_root_context() -> RwLockWriteGuard<'static, RootContext> {
    ROOT_CONTEXT.write().unwrap()
}

#[derive(Default)]
pub struct RootContext {
    pub attachments: ContextAttachmentMap<Self>,
    pub children: Option<Vec<ProjectContext>>,
}

impl Context for RootContext {
    fn attachments(&self) -> &ContextAttachmentMap<Self> {
        &self.attachments
    }
    fn attachments_mut(&mut self) -> &mut ContextAttachmentMap<Self> {
        &mut self.attachments
    }
}

pub struct ProjectContext {
    pub parent: RootContext,
    pub attachments: ContextAttachmentMap<Self>,
    pub children: Option<Vec<ProjectContext>>,
}

impl ProjectContext {
    pub fn new(parent: RootContext) -> Self {
        Self {
            parent,
            attachments: ContextAttachmentMap::new(),
            children: None,
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

pub struct ModuleContext {
    pub parent: ProjectContext,
    pub attachments: ContextAttachmentMap<Self>,
    pub children: Option<Vec<FileContext>>,
}

impl ModuleContext {
    pub fn new(parent: ProjectContext) -> Self {
        Self {
            parent,
            attachments: ContextAttachmentMap::new(),
            children: None,
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

pub struct FileContext {
    pub parent: ModuleContext,
    pub attachments: ContextAttachmentMap<Self>,
}

impl FileContext {
    pub fn new(parent: ModuleContext) -> Self {
        Self {
            parent,
            attachments: ContextAttachmentMap::new(),
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
