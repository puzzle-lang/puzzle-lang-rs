use crate::context::context::ModuleContext;
use crate::context::attachment::ContextAttachment;
use std::path::PathBuf;

pub struct IgnoreRuleAttachment {
    pub values: Vec<IgnoreRule>,
}

impl ContextAttachment<ModuleContext> for IgnoreRuleAttachment {}

#[derive(Eq, PartialEq, Hash)]
pub struct IgnoreRule {
    pub path: PathBuf,
    pub kind: IgnoreKind,
    pub raw: String,
}

#[derive(Eq, PartialEq, Hash)]
pub enum IgnoreKind {
    EXACT,
    CHILDREN,
    RECURSIVE,
}
