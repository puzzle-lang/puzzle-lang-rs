use serde::Deserialize;

#[derive(Deserialize)]
pub struct ModuleToml {
    pub module: Option<Module>,
}

#[derive(Deserialize)]
pub struct Module {
    pub version: Option<String>,
    pub name: Option<String>,
    pub group: Option<String>,
    pub ignores: Option<Vec<String>>,
    pub deps: Option<Vec<String>>,
}
