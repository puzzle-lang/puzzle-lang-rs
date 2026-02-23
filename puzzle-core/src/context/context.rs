use crate::context::context_attachment::ContextAttachment;
use std::any::TypeId;
use std::collections::HashMap;

pub enum Context {
    Root {
        attachment: HashMap<TypeId, Box<dyn ContextAttachment>>,
    },
    Project {
        parent: Box<Context>,
        attachment: HashMap<TypeId, Box<dyn ContextAttachment>>,
    },
    Module {
        parent: Box<Context>,
        attachment: HashMap<TypeId, Box<dyn ContextAttachment>>,
    },
    File {
        parent: Box<Context>,
        attachment: HashMap<TypeId, Box<dyn ContextAttachment>>,
    },
}

impl Context {
    pub fn set(&mut self, key: TypeId, value: Box<dyn ContextAttachment>) {
        let attachment = match self {
            Context::Root { attachment, .. } => attachment,
            Context::Project { attachment, .. } => attachment,
            Context::Module { attachment, .. } => attachment,
            Context::File { attachment, .. } => attachment,
        };
        attachment.insert(key, value);
    }

    pub fn get<CA: ContextAttachment + 'static>(&self) -> &CA {
        let attachment = match self {
            Context::Root { attachment, .. } => attachment,
            Context::Project { attachment, .. } => attachment,
            Context::Module { attachment, .. } => attachment,
            Context::File { attachment, .. } => attachment,
        };
        let boxed = &attachment[&TypeId::of::<CA>()];
        boxed.as_any().downcast_ref::<CA>().unwrap()
    }
}
