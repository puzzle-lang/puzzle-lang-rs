use crate::context::context::ModuleContext;
use crate::context::attachment::ContextAttachment;

pub struct DependenceAttachment {
    pub values: Vec<Dependence>,
}

impl ContextAttachment<ModuleContext> for DependenceAttachment {}

pub struct Dependence {
    project_name: String,
    module_name: String,
}
