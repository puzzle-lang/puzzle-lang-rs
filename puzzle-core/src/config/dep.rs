use crate::context::attachment::ContextAttachment;
use crate::context::context::ModuleContext;
use crate::impl_context_attachment;
use std::any::Any;

pub struct DependenceAttachment {
    pub deps: Vec<Dependence>,
}

impl_context_attachment!(DependenceAttachment, ModuleContext);

pub struct Dependence {
    project_name: String,
    module_name: String,
}
