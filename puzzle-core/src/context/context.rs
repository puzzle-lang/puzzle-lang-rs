use crate::context::attachment::ContextAttachment;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};

pub type ContextAttachmentMap<C: Context> = HashMap<TypeId, Box<dyn ContextAttachment<C>>>;

pub trait Context
where
    Self: 'static,
{
    fn attachments(&self) -> &ContextAttachmentMap<Self>;

    fn attachments_mut(&mut self) -> &mut ContextAttachmentMap<Self>;

    fn set<CA: ContextAttachment<Self> + 'static>(&mut self, value: CA) {
        let id = TypeId::of::<CA>();
        self.attachments_mut().insert(id, Box::new(value));
    }

    fn get<CA: ContextAttachment<Self> + 'static>(&self) -> &CA {
        let id = TypeId::of::<CA>();
        let boxed = &self.attachments()[&id].as_any();
        boxed.downcast_ref::<CA>().unwrap()
    }
}

pub static ROOT_CONTEXT: LazyLock<Arc<RwLock<RootContext>>> =
    LazyLock::new(|| Arc::new(RwLock::new(RootContext::default())));

pub fn read_root_context() -> RwLockReadGuard<'static, RootContext> {
    ROOT_CONTEXT.read().unwrap()
}

pub fn write_root_context() -> RwLockWriteGuard<'static, RootContext> {
    ROOT_CONTEXT.write().unwrap()
}

pub type ContextRef<T> = Arc<RwLock<T>>;
pub type ContextWeak<T> = Weak<RwLock<T>>;

#[derive(Default)]
pub struct RootContext {
    pub attachments: ContextAttachmentMap<Self>,
    pub children: Option<Vec<ContextRef<ProjectContext>>>,
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
    pub parent: ContextWeak<RootContext>,
    pub attachments: ContextAttachmentMap<Self>,
    pub children: Option<Vec<ContextRef<ModuleContext>>>,
}

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

pub struct ModuleContext {
    pub parent: ContextWeak<ProjectContext>,
    pub attachments: ContextAttachmentMap<Self>,
    pub children: Option<Vec<ContextRef<FileContext>>>,
}

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

pub struct FileContext {
    pub parent: ContextWeak<ModuleContext>,
    pub attachments: ContextAttachmentMap<Self>,
}

impl FileContext {
    pub fn new(parent: ContextWeak<ModuleContext>) -> Self {
        FileContext {
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

pub fn context_ref<C: Context>(context: C) -> ContextRef<C> {
    Arc::new(RwLock::new(context))
}
