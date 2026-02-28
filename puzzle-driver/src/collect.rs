use crate::config_error;
use puzzle_core::config::ignore::{IgnoreKind, IgnoreRule, IgnoreRulesAttachment};
use puzzle_core::config::toml::module::ModuleToml;
use puzzle_core::config::toml::project::{Module, ProjectToml};
use puzzle_core::context::context::{context_ref, Context, ContextRef, HasChildren};
use puzzle_core::context::file::{FileAttachment, FileContext};
use puzzle_core::context::module::{ModuleAttachment, ModuleContext};
use puzzle_core::context::project::{ProjectAttachment, ProjectContext};
use puzzle_core::context::root::{
    read_root_context, weak_root_context, write_root_context, RootAttachment,
};
use puzzle_core::extension::{PathBufExt, StringExt};
use regex::Regex;
use std::collections::HashSet;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};
use toml::from_str;

pub fn collect_sources(project_path: &PathBuf) {
    let projects = collect_all_projects(project_path.clone(), true);
    let project_contexts = projects.iter().map(get_project_context).collect::<Vec<_>>();
    let mut guard = write_root_context();
    guard.set_children(project_contexts);

    let max_path_length = calc_max_path_length();
    println!("{}", max_path_length);
    guard.set(RootAttachment { max_path_length });
}

#[derive(Debug)]
struct ProjectTemp {
    path: PathBuf,
    toml: ProjectToml,
    toml_path: PathBuf,
}

fn collect_all_projects(project_path: PathBuf, is_root_project: bool) -> Vec<ProjectTemp> {
    let mut projects = Vec::new();
    let name = project_path.file_name_string();
    let toml_path = project_path.join("puzzle.toml");
    let project_toml = get_project_toml(&project_path, &toml_path, name, is_root_project);
    let deps = project_toml.deps.as_ref();
    if deps.is_some() {
        deps.unwrap().iter().for_each(|(name, dep)| {
            let path = dep
                .path
                .as_ref()
                .map(|p| p)
                .unwrap_or_else(|| config_error!(&toml_path, ""));
            let mut dep_project_path = PathBuf::from(path);
            if !(dep_project_path.exists() && dep_project_path.is_dir()) {
                dep_project_path = project_path.join(dep_project_path);
                if !(dep_project_path.exists() && dep_project_path.is_dir()) {
                    config_error!(&project_path, r#""{:?}" 依赖的项目不存在"#, name);
                }
            }
            if !dep_project_path.is_absolute() {
                dep_project_path = dep_project_path.canonicalize().unwrap();
            }
            let sub_projects = collect_all_projects(dep_project_path, false);
            projects.extend(sub_projects);
        })
    }
    let project = ProjectTemp {
        path: project_path,
        toml: project_toml,
        toml_path,
    };
    projects.push(project);
    projects
}

fn get_project_context(project: &ProjectTemp) -> ContextRef<ProjectContext> {
    let context = context_ref(ProjectContext::new(weak_root_context()));
    let project_toml = &project.toml;
    let project_name = project_toml
        .project
        .as_ref()
        .and_then(|p| p.name.as_ref())
        .unwrap();

    let mut guard = context.write().unwrap();

    guard.set(ProjectAttachment {
        name: project_name.clone(),
        path: project.path.clone(),
    });

    let modules = project_toml
        .modules
        .as_ref()
        .unwrap()
        .iter()
        .map(|(name, module)| {
            let module_path = resolve_module_path(project, module, project_name, name);
            let toml_path = module_path.join("puzzle.toml");
            let module_toml = get_module_toml(&module_path, &toml_path, name, project_toml);
            get_module_context(&context, &module_path, &toml_path, module_toml, name)
        })
        .collect::<Vec<_>>();
    guard.set_children(modules);

    drop(guard);
    context
}

fn resolve_module_path(
    project: &ProjectTemp,
    module: &Module,
    project_name: &String,
    module_name: &String,
) -> PathBuf {
    let path = module.path.as_ref().unwrap();
    let mut module_path = PathBuf::from(path);
    if !(module_path.exists() && module_path.is_absolute() && module_path.is_dir()) {
        module_path = project.path.join(path);
        if module_path.exists() && module_path.is_dir() {
            module_path = module_path.canonicalize().unwrap();
        } else {
            config_error!(
                &project.toml_path,
                r#"{:?} 项目的 "puzzle.toml" 配置文件中 [modules] 的 {:?} 属性配置的 "path" 路径不存在"#,
                project_name,
                module_name
            )
        }
    }
    module_path
}

fn get_module_context(
    parent: &ContextRef<ProjectContext>,
    module_path: &PathBuf,
    toml_path: &PathBuf,
    toml: ModuleToml,
    module_name: &String,
) -> ContextRef<ModuleContext> {
    let source_path = module_path.join("src");
    if !(source_path.exists() && source_path.is_dir()) {
        config_error!(module_path, "源目录不存在");
    }
    let module = toml.module.unwrap();
    let ignore_rules = module
        .ignores
        .map(|ignores| to_ignore_rules(&source_path, toml_path, module_name, ignores))
        .unwrap_or_default();

    let context = context_ref(ModuleContext::new(Arc::downgrade(parent)));

    let mut guard = context.write().unwrap();
    guard.set(ModuleAttachment {
        name: module.name.unwrap(),
        group: module.group.map(|it| it.split_to_vec(".")).unwrap(),
    });

    let mut file_paths = HashSet::new();
    let mut dir_rules = Vec::new();
    for rule in ignore_rules {
        if rule.kind == IgnoreKind::File {
            file_paths.insert(rule.path);
        } else {
            dir_rules.push(rule);
        }
    }

    guard.set(IgnoreRulesAttachment {
        file_paths,
        dir_rules,
    });

    let paths = collect_all_source_paths(source_path, guard.get());

    // guard.set_children(paths);

    drop(guard);

    context
}

static IGNORE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-z][a-z0-9]*/)*(?:\*\*|\*|[a-zA-Z][a-zA-Z0-9]*\.pzl)$").unwrap()
});

fn to_ignore_rules(
    source_path: &PathBuf,
    toml_path: &PathBuf,
    module_name: &String,
    ignores: Vec<String>,
) -> Vec<IgnoreRule> {
    ignores
        .into_iter()
        .enumerate()
        .map(|(index, ignore)| {
            if !IGNORE_REGEX.is_match(&ignore) {
                config_error!(
                    toml_path,
                    r#"{:?} 模块的 "puzzle.toml" 配置文件中 [module] 表的 "ignores[{}]" 属性违反模式: {:?}"#,
                    module_name,
                    index,
                    IGNORE_REGEX.as_str()
                );
            }
            match ignore.as_str() {
                "**" => IgnoreRule {
                    path: source_path.clone(),
                    kind: IgnoreKind::Recursive,
                },
                "*" => IgnoreRule {
                    path: source_path.clone(),
                    kind: IgnoreKind::Children,
                },
                _ if ignore.ends_with("/**") => IgnoreRule {
                    path: source_path.join(ignore.strip_suffix("/**").unwrap()),
                    kind: IgnoreKind::Recursive,
                },
                _ if ignore.ends_with("/*") => IgnoreRule {
                    path: source_path.join(ignore.strip_suffix("/*").unwrap()),
                    kind: IgnoreKind::Children,
                },
                _ => IgnoreRule {
                    path: source_path.join(&ignore),
                    kind: IgnoreKind::File,
                }
            }
        })
        .collect()
}

fn get_file_context(parent: &ContextRef<ModuleContext>, path: &PathBuf) -> ContextRef<FileContext> {
    let context = context_ref(FileContext::new(Arc::downgrade(parent)));

    let mut guard = context.write().unwrap();
    guard.set(FileAttachment {
        builtin: false,
        name: path.file_name_string(),
        path: path.clone(),
        line_starts: None,
    });

    drop(guard);

    context
}

static NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z][a-z0-9-]*$").unwrap());
static VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+\.\d+\.\d+$").unwrap());
static GROUP_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z][a-z0-9]*(?:\.[a-z][a-z0-9]*)*$").unwrap());

fn get_project_toml(
    project_path: &PathBuf,
    toml_path: &PathBuf,
    project_name: String,
    is_root_project: bool,
) -> ProjectToml {
    if !(toml_path.exists() && toml_path.is_file()) {
        config_error!(project_path, "puzzle.toml 项目配置文件缺失");
    }
    let content = read_to_string(&toml_path).unwrap();
    let toml = from_str::<ProjectToml>(&content).unwrap();
    let Some(ref project) = toml.project else {
        config_error!(
            toml_path,
            r#"{:?} 项目的 "puzzle.toml" 配置文件中缺少 [project] 表"#,
            project_name
        )
    };
    match &project.name {
        None => config_error!(
            project_path,
            r#"{:?} 项目的 "puzzle.toml" 配置文件中 [project] 表缺少必要属性 "name""#,
            project_name
        ),
        Some(name) if !NAME_REGEX.is_match(&name) => {
            config_error!(
                toml_path,
                r#"{:?} 项目的 "puzzle.toml" 配置文件中 [project] 表的 "name" 属性违反模式: {:?}"#,
                project_name,
                NAME_REGEX.as_str()
            )
        }
        _ => {}
    };
    match &project.version {
        None => config_error!(
            toml_path,
            r#"{:?} 项目的 "puzzle.toml" 配置文件中 [project] 表中缺少必要属性 "version""#,
            project_name
        ),
        Some(version) if !VERSION_REGEX.is_match(&version) => {
            config_error!(
                toml_path,
                r#"{:?} 项目的 "puzzle.toml" 配置文件中 [project] 表的 "version" 属性违反模式: {:?}"#,
                project_name,
                VERSION_REGEX.as_str()
            )
        }
        _ => {}
    };

    let Some(ref modules) = toml.modules else {
        config_error!(
            toml_path,
            r#"{:?} 项目的 "puzzle.toml" 配置文件中缺少 [modules] 表"#,
            project_name
        )
    };
    if modules.is_empty() {
        config_error!(
            toml_path,
            r#"{:?} 项目的 "puzzle.toml" 配置文件中 [modules] 表中至少需要配置一个模块"#,
            project_name,
        )
    }
    if !is_root_project {
        return toml;
    }
    match &project.entry {
        None => config_error!(
            toml_path,
            r#"{:?} 主项目的 "puzzle.toml" 配置文件中 [project] 表中缺少必要属性 "entry""#,
            project_name
        ),
        Some(entry) if !modules.contains_key(entry) => {
            config_error!(
                toml_path,
                r#"{:?} 项目的 "puzzle.toml" 配置文件中 [project] 的 "entry" 属性必须在 [modules] 表中选择"#,
                project_name
            )
        }
        _ => {}
    };
    toml
}

fn get_module_toml(
    module_path: &PathBuf,
    toml_path: &PathBuf,
    module_name: &String,
    project_toml: &ProjectToml,
) -> ModuleToml {
    if !(toml_path.exists() && toml_path.is_file()) {
        config_error!(module_path, "puzzle.toml 模块配置文件缺失");
    }
    let content = read_to_string(&toml_path).unwrap();
    let mut toml = from_str::<ModuleToml>(&content).unwrap();
    let Some(ref mut module) = toml.module else {
        config_error!(
            toml_path,
            r#"{:?} 模块的 "puzzle.toml" 配置文件中缺少 [module] 表"#,
            module_name
        );
    };
    match &module.name {
        None => config_error!(
            toml_path,
            r#"{:?} 模块的 "puzzle.toml" 配置文件中 [module] 表中缺少必要属性 "name""#,
            module_name
        ),
        Some(name) if !NAME_REGEX.is_match(&name) => {
            config_error!(
                toml_path,
                r#"{:?} 模块的 "puzzle.toml" 配置文件中 [module] 表的 "name" 属性违反模式: {:?}"#,
                module_name,
                NAME_REGEX.as_str()
            )
        }
        _ => {}
    }

    match &module.version {
        None => {
            module.version = project_toml.project.as_ref().unwrap().version.clone();
        }
        Some(version) if !VERSION_REGEX.is_match(&version) => {
            config_error!(
                toml_path,
                r#"{:?} 模块的 "puzzle.toml" 配置文件中 [module] 表的 "version" 属性违反模式: {:?}"#,
                module_name,
                VERSION_REGEX.as_str()
            )
        }
        _ => {}
    };

    match &module.group {
        None => config_error!(
            toml_path,
            r#"{:?} 模块的 "puzzle.toml" 配置文件中 [module] 表中缺少必要属性 "group""#,
            module_name
        ),
        Some(group) if !GROUP_REGEX.is_match(&group) => {
            config_error!(
                toml_path,
                r#"{:?} 模块的 "puzzle.toml" 配置文件中 [module] 表的 "group" 属性违反模式: {:?}"#,
                module_name,
                GROUP_REGEX.as_str()
            )
        }
        _ => {}
    };

    toml
}

fn collect_all_source_paths(path: PathBuf, attachment: &IgnoreRulesAttachment) -> Vec<PathBuf> {
    if !path.exists() {
        return Vec::new();
    }
    if path.is_file() && path.file_name_string().ends_with(".pzl") {
        if attachment.file_paths.contains(&path) {
            Vec::new()
        } else {
            vec![path]
        }
    } else if path.is_dir() {
        let Some(IgnoreRule { kind, .. }) = attachment.dir_rules.iter().find(|it| it.path == path)
        else {
            return read_dir(&path)
                .unwrap()
                .filter_map(|it| it.ok())
                .flat_map(|it| collect_all_source_paths(it.path(), attachment))
                .collect();
        };
        if *kind == IgnoreKind::Children {
            read_dir(&path)
                .unwrap()
                .filter_map(|it| it.ok())
                .map(|it| it.path())
                .filter(|it| it.is_dir())
                .flat_map(|it| collect_all_source_paths(it, attachment))
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

fn calc_max_path_length() -> usize {
    let mut max_path_length = 0;
    for project in read_root_context().read_children() {
        for module in project.read_children() {
            for file in module.read_children() {
                let attachment = file.get::<FileAttachment>();
                let length = attachment.path.canonicalize_string().len();
                if length > max_path_length {
                    max_path_length = length;
                }
            }
        }
    }
    max_path_length
}
