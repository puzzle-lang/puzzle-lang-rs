use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleToml {
    pub module: Option<Module>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub version: Option<String>,
    pub name: Option<String>,
    pub group: Option<String>,
    pub ignores: Option<Vec<String>>,
    pub deps: Option<Vec<String>>,
}
