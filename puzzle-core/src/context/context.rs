use crate::context::context_attachment::ContextAttachment;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type ContextAttachmentMap<C: Context> = HashMap<TypeId, Box<dyn ContextAttachment<C>>>;

pub trait Context
where
    Self: 'static,
{
    fn attachments(&self) -> &ContextAttachmentMap<Self>;

    fn attachments_mut(&mut self) -> &mut ContextAttachmentMap<Self>;

    fn insert<CA: ContextAttachment<Self> + 'static>(&mut self, value: CA)
    where
        Self: Sized,
    {
        let id = TypeId::of::<CA>();
        self.attachments_mut().insert(id, Box::new(value));
    }

    fn get<CA: ContextAttachment<Self> + 'static>(&self) -> &CA {
        let id = TypeId::of::<CA>();
        let boxed = &self.attachments()[&id] as &dyn Any;
        boxed.downcast_ref::<CA>().unwrap()
    }
}

pub struct RootContext {
    pub attachments: ContextAttachmentMap<Self>,
    pub children: Vec<ProjectContext>,
}

impl RootContext {
    pub fn new() -> Self {
        Self {
            attachments: ContextAttachmentMap::default(),
            children: Vec::default(),
        }
    }
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
    pub children: Vec<ProjectContext>,
}

impl ProjectContext {
    pub fn new(parent: RootContext) -> Self {
        Self {
            parent,
            attachments: ContextAttachmentMap::default(),
            children: Vec::default(),
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
    pub children: Vec<FileContext>,
}

impl ModuleContext {
    pub fn new(parent: ProjectContext) -> Self {
        Self {
            parent,
            attachments: ContextAttachmentMap::default(),
            children: Vec::default(),
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
            attachments: ContextAttachmentMap::default(),
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
