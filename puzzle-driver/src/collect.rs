use puzzle_core::config::ignore::{IgnoreKind, IgnoreRule};
use puzzle_core::config::toml::module::ModuleToml;
use puzzle_core::config::toml::project::ProjectToml;
use puzzle_core::config_error;
use puzzle_core::context::context::{Context, FileContext, ModuleContext, ProjectContext};
use puzzle_core::context::context_attachment::FileContextAttachment;
use puzzle_core::error::config_error;
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::sync::LazyLock;
use toml::from_str;

pub struct SourcesResult {}

pub fn collect_sources(project_path: &PathBuf) -> SourcesResult {
    let projects = collect_projects(project_path.clone(), true);
    todo!()
}

struct Project {
    path: PathBuf,
    toml: ProjectToml,
}

fn collect_projects(project_path: PathBuf, is_root_project: bool) -> Vec<Project> {
    let mut projects = Vec::new();
    let project_toml = get_project_toml(&project_path, is_root_project);
    projects.push(Project {
        path: project_path,
        toml: project_toml,
    });
    projects
}

static NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z][a-z0-9-]*$").unwrap());
static VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+\.\d+\.\d+$").unwrap());

fn get_project_toml(project_path: &PathBuf, is_root_project: bool) -> ProjectToml {
    let toml_path = project_path.join("puzzle.toml");
    if !(toml_path.exists() && toml_path.is_file()) {
        config_error!("puzzle.toml 项目配置文件缺失");
    }
    let content = read_to_string(&toml_path).unwrap();
    let toml = from_str::<ProjectToml>(&content).unwrap();
    match toml.project.name {
        None => config_error!(
            r#"{:?} 项目的 "puzzle.toml" 配置文件中, [project] 缺少必要属性 "name""#,
            project_path.file_name().unwrap(),
        ),
        Some(name) if !NAME_REGEX.is_match(&name) => {
            config_error!(
                r#"{:?} 项目的 "puzzle.toml" 配置文件中, [project] 的 "name" 属性违反模式: "{}""#,
                project_path.file_name().unwrap(),
                NAME_REGEX.as_str()
            )
        }
        _ => {}
    };
    match toml.project.version {
        None => config_error!(
            r#"{:?} 项目的 "puzzle.toml" 配置文件中, [project] 缺少必要属性 "version""#,
            project_path.file_name().unwrap(),
        ),
        Some(version) if !VERSION_REGEX.is_match(&version) => {
            config_error!(
                r#"{:?} 项目的 "puzzle.toml" 配置文件中, [project] 的 "version" 属性违反模式: "{}""#,
                project_path.file_name().unwrap(),
                VERSION_REGEX.as_str()
            )
        }
        _ => {}
    };
    let modules = &toml.modules;
    if modules.is_empty() {
        config_error!(
            r#"{:?} 项目的 "puzzle.toml" 配置文件中, [modules] 至少需要配置一个模块"#,
            project_path.file_name().unwrap(),
        )
    }
    if !is_root_project {
        return toml;
    }
    match toml.project.entry {
        None => config_error!(
            r#"{:?} 主项目的 "puzzle.toml" 配置文件中, [project] 缺少必要属性 "entry""#,
            project_path.file_name().unwrap(),
        ),
        Some(entry) if !modules.contains_key(&entry) => {
            config_error!(
                r#"{:?} 项目的 "puzzle.toml" 配置文件中, [project] 的 "entry" 属性必须在 [modules] 模块配置中选择"#,
                project_path.file_name().unwrap()
            )
        }
        _ => {}
    };
    toml
}

fn get_module_context(parent: ProjectContext, path: PathBuf, config: ModuleToml) -> ModuleContext {
    let source_path = path.join("src/main");
    if !(source_path.exists() && source_path.is_dir()) {
        config_error!("{:?} 源目录不存在", source_path.file_name().unwrap());
    }
    let ignore_rules = config
        .module
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
    attachment.name = path.file_name().unwrap().to_string_lossy().into_owned();
    context.insert(attachment);
    context.into()
}
