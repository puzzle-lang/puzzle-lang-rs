use crate::context::attachment::ContextAttachment;
use crate::context::context::ModuleContext;
use crate::impl_context_attachment;
use std::any::Any;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct IgnoreRulesAttachment {
    pub file_paths: HashSet<PathBuf>,
    pub dir_rules: Vec<IgnoreRule>,
}

impl_context_attachment!(IgnoreRulesAttachment, ModuleContext);

#[derive(Eq, PartialEq, Hash)]
pub struct IgnoreRule {
    pub path: PathBuf,
    pub kind: IgnoreKind,
}

#[derive(Eq, PartialEq, Hash)]
pub enum IgnoreKind {
    File,
    Children,
    Recursive,
}
