use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ProjectToml {
    pub project: Option<Project>,
    pub modules: Option<HashMap<String, Module>>,
    pub deps: Option<HashMap<String, Dep>>,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub version: Option<String>,
    pub name: Option<String>,
    pub entry: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Module {
    pub path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Dep {
    pub path: Option<String>,
    pub module: Option<String>,
}
