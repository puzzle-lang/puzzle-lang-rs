use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};

pub type ContextAttachmentMap<C> = HashMap<TypeId, Box<dyn ContextAttachment<C>>>;

pub type ContextRef<T> = Arc<RwLock<T>>;
pub type ContextWeak<T> = Weak<RwLock<T>>;

pub fn context_ref<C: Context>(context: C) -> ContextRef<C> {
    Arc::new(RwLock::new(context))
}

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

pub trait ContextAttachment<C: Context + ?Sized>: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

#[macro_export]
macro_rules! impl_context_attachment {
    ($ca:ty, $c:ty) => {
        use crate::context::context::ContextAttachment;
        use std::any::Any;

        impl ContextAttachment<$c> for $ca {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}
pub trait HasChildren {
    type Child: Context;

    fn set_children(&mut self, children: Vec<ContextRef<Self::Child>>);

    fn get_children(&self) -> &Vec<ContextRef<Self::Child>>;

    fn read_children(&self) -> Vec<RwLockReadGuard<'_, Self::Child>> {
        self.get_children()
            .iter()
            .map(|c| c.read().unwrap())
            .collect()
    }

    fn write_children(&self) -> Vec<RwLockWriteGuard<'_, Self::Child>> {
        self.get_children()
            .iter()
            .map(|c| c.write().unwrap())
            .collect()
    }
}

pub trait HasParent {
    type Parent: Context;

    fn parent(&self) -> Arc<RwLock<Self::Parent>>;
}
