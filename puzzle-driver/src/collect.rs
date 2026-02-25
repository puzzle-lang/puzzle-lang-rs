use crate::config_error;
use crate::error::config_error;
use puzzle_core::config::ignore::{IgnoreKind, IgnoreRule};
use puzzle_core::config::toml::module::ModuleToml;
use puzzle_core::config::toml::project::ProjectToml;
use puzzle_core::context::attachment::FileContextAttachment;
use puzzle_core::context::context::{
    write_root_context, Context, FileContext, ModuleContext, ProjectContext,
};
use puzzle_core::extension::{OptionExt, PathBufExt};
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::sync::LazyLock;
use toml::from_str;

pub fn collect_sources(project_path: &PathBuf) {
    let projects = collect_all_projects(project_path.clone(), true);
    let project_contexts = projects.iter().map(get_project_context).collect::<Vec<_>>();
    let mut root = write_root_context();
    root.children = project_contexts.some();
    todo!()
}

#[derive(Debug)]
struct Project {
    path: PathBuf,
    toml: ProjectToml,
}

fn collect_all_projects(project_path: PathBuf, is_root_project: bool) -> Vec<Project> {
    let mut projects = Vec::new();
    let toml_path = project_path.join("puzzle.toml");
    let name = project_path.file_name_string();
    let project_toml = get_project_toml(&toml_path, name, is_root_project);
    let deps = project_toml.deps.as_ref();
    if deps.is_some() {
        deps.unwrap().iter().for_each(|(name, dep)| {
            let path = dep
                .path
                .as_ref()
                .map(|p| p)
                .unwrap_or_else(|| config_error!(toml_path.some_ref(), ""));
            let mut dep_project_path = PathBuf::from(path);
            if !(dep_project_path.exists() && dep_project_path.is_dir()) {
                dep_project_path = project_path.join(dep_project_path);
                if !(dep_project_path.exists() && dep_project_path.is_dir()) {
                    config_error!(project_path.some_ref(), r#""{:?}" 依赖的项目不存在"#, name);
                }
            }
            if !dep_project_path.is_absolute() {
                dep_project_path = dep_project_path.canonicalize().unwrap();
            }
            let sub_projects = collect_all_projects(dep_project_path, false);
            projects.extend(sub_projects);
        })
    }
    let project = Project {
        path: project_path,
        toml: project_toml,
    };
    projects.push(project);
    projects
}

static NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z][a-z0-9-]*$").unwrap());
static VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+\.\d+\.\d+$").unwrap());

fn get_project_toml(toml_path: &PathBuf, name: String, is_root_project: bool) -> ProjectToml {
    if !(toml_path.exists() && toml_path.is_file()) {
        config_error!(None, "puzzle.toml 项目配置文件缺失");
    }
    let content = read_to_string(&toml_path).unwrap();
    let toml = from_str::<ProjectToml>(&content).unwrap();
    if toml.project.is_none() {
        config_error!(
            toml_path.some_ref(),
            r#"{:?} 项目的 "puzzle.toml" 配置文件中缺少 [project] 表"#,
            name
        )
    }
    let project = toml.project.as_ref().unwrap();
    match &project.name {
        None => config_error!(
            None,
            r#"{:?} 项目的 "puzzle.toml" 配置文件中, [project] 缺少必要属性 "name""#,
            name
        ),
        Some(name) if !NAME_REGEX.is_match(&name) => {
            config_error!(
                toml_path.some_ref(),
                r#"{:?} 项目的 "puzzle.toml" 配置文件中, [project] 的 "name" 属性违反模式: {:?}"#,
                name,
                NAME_REGEX.as_str()
            )
        }
        _ => {}
    };
    match &project.version {
        None => config_error!(
            toml_path.some_ref(),
            r#"{:?} 项目的 "puzzle.toml" 配置文件中, [project] 缺少必要属性 "version""#,
            name
        ),
        Some(version) if !VERSION_REGEX.is_match(&version) => {
            config_error!(
                toml_path.some_ref(),
                r#"{:?} 项目的 "puzzle.toml" 配置文件中, [project] 的 "version" 属性违反模式: {:?}"#,
                name,
                VERSION_REGEX.as_str()
            )
        }
        _ => {}
    };
    let modules = toml.modules.as_ref().unwrap();
    if modules.is_empty() {
        config_error!(
            toml_path.some_ref(),
            r#"{:?} 项目的 "puzzle.toml" 配置文件中, [modules] 至少需要配置一个模块"#,
            name,
        )
    }
    if !is_root_project {
        return toml;
    }
    match &project.entry {
        None => config_error!(
            toml_path.some_ref(),
            r#"{:?} 主项目的 "puzzle.toml" 配置文件中, [project] 缺少必要属性 "entry""#,
            name
        ),
        Some(entry) if !modules.contains_key(entry) => {
            config_error!(
                toml_path.some_ref(),
                r#"{:?} 项目的 "puzzle.toml" 配置文件中, [project] 的 "entry" 属性必须在 [modules] 模块配置中选择"#,
                name
            )
        }
        _ => {}
    };
    toml
}

fn get_project_context(project: &Project) -> ProjectContext {
    todo!()
}

fn get_module_context(parent: ProjectContext, path: PathBuf, config: ModuleToml) -> ModuleContext {
    let source_path = path.join("src/main");
    if !(source_path.exists() && source_path.is_dir()) {
        config_error!(None, "{:?} 源目录不存在", source_path.file_name().unwrap());
    }
    let ignore_rules = config
        .module
        .unwrap()
        .ignores
        .map(|v| to_ignore_rules(v, path))
        .unwrap_or_default();
    todo!()
}

fn to_ignore_rules(vec: Vec<String>, path: PathBuf) -> HashSet<IgnoreRule> {
    vec.iter()
        .enumerate()
        .map(|(idx, ignore)| match ignore.as_str() {
            "**" => IgnoreRule {
                path: path.clone(),
                kind: IgnoreKind::RECURSIVE,
                raw: ignore.clone(),
            },
            "*" => IgnoreRule {
                path: path.clone(),
                kind: IgnoreKind::CHILDREN,
                raw: ignore.clone(),
            },
            _ => todo!(),
        })
        .collect()
}

fn get_file_context(parent: ModuleContext, path: &PathBuf) -> FileContext {
    let mut context = FileContext::new(parent);
    let mut attachment = FileContextAttachment::default();
    attachment.name = path.file_name_string();
    attachment.path = path.clone();
    context.insert(attachment);
    context.into()
}
