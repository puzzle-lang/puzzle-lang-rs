use crate::context::context_attachment::ContextAttachment;

pub struct DependenceAttachment {
    pub values: Vec<Dependence>,
}

impl ContextAttachment for DependenceAttachment {}

pub struct Dependence {
    project_name: String,
    module_name: String,
}
