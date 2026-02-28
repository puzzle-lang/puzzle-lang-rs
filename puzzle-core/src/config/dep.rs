use crate::context::module::ModuleContext;
use crate::impl_context_attachment;

pub struct DependenceAttachment {
    pub deps: Vec<Dependence>,
}

impl_context_attachment!(DependenceAttachment, ModuleContext);

pub struct Dependence {
    project_name: String,
    module_name: String,
}
