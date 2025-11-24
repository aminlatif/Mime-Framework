use std::{collections::HashMap, fs, path::PathBuf};

use tracing::{debug, error, info, warn};

use crate::types::TemplatePath;

pub async fn load_template_files(
    templates_list_list: &Vec<Vec<TemplatePath>>,
    mut templates: tera::Tera,
) -> anyhow::Result<tera::Tera> {
    debug!("Loading template files...");

    let mut template_file_list: HashMap<String, (u32, String)> = HashMap::new();

    for templates_list in templates_list_list {
        for template_info in templates_list {
            let template_path = template_info.path.replace("\\", "/");
            let template_path = template_path.replace("\"", "");
            let template_path = template_path.replace("/**", "").replace("/*", "");

            let template_namespace = template_info.namespace.clone();
            let template_namespace = template_namespace.unwrap_or("".to_string());

            let template_name = template_info.name.clone();

            let metadata = fs::metadata(&template_path);

            if metadata.is_err() {
                error!(" * error on reading metadata for {template_path}: {metadata:?}");
                continue;
            }

            let is_directory = metadata.unwrap().is_dir();

            if is_directory {
                load_template_directory(
                    template_path,
                    template_namespace,
                    template_name,
                    vec![],
                    &mut template_file_list,
                )
                .await?;
            } else {
                load_template_file(
                    template_path,
                    template_namespace,
                    template_name,
                    vec![],
                    &mut template_file_list,
                )
                .await?;
            }
        }
    }

    let mut items: Vec<(&String, &(u32, String))> = template_file_list.iter().collect();
    items.sort_by(|a, b| a.1.0.cmp(&b.1.0));

    debug!("Templates file list:");
    for (template_file_name, (sort_order, template_file_path)) in items {
        debug!("\t{}. {}: {}", sort_order, template_file_name, template_file_path);
        let result = templates.add_template_file(template_file_path, Some(template_file_name));
        if result.is_err() {
            error!("  * error: {result:?}");
        }
    }

    debug!("Templates loaded:");
    for template in templates.get_template_names() {
        debug!("\t{template}");
    }

    Ok(templates)
}

pub async fn load_template_directory(
    directory_path: String,
    namespace: String,
    name: Option<String>,
    relative_directory_path: Vec<String>,
    template_file_list: &mut HashMap<String, (u32, String)>,
) -> anyhow::Result<()> {
    info!("* loading templates directory: {directory_path}");
    let mut directory_paths: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(directory_path.clone()).unwrap() {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("tera") {
            let file_path = path.to_str().unwrap().replace("\\", "/");

            load_template_file(
                file_path,
                namespace.clone(),
                name.clone(),
                relative_directory_path.clone(),
                template_file_list,
            )
            .await?;
        } else if path.is_dir() {
            directory_paths.push(path);
        }
    }

    for directory_path_buf in directory_paths {
        let directory_path = directory_path_buf.to_str().unwrap().replace("\\", "/");
        let mut temp_relative_directory_path = relative_directory_path.clone();
        temp_relative_directory_path.push(directory_path_buf.file_name().unwrap().to_str().unwrap().to_string());

        Box::pin(load_template_directory(
            directory_path,
            namespace.clone(),
            name.clone(),
            temp_relative_directory_path,
            template_file_list,
        ))
        .await?;
    }

    Ok(())
}

pub async fn load_template_file(
    file_path: String,
    namespace: String,
    name: Option<String>,
    mut relative_directory_path: Vec<String>,
    template_file_list: &mut HashMap<String, (u32, String)>,
) -> anyhow::Result<()> {
    info!("  * loading template file: {file_path}");

    let path: PathBuf = PathBuf::from(&file_path);

    let file_name = path.file_name().unwrap().to_str().unwrap();
    let file_name_without_extension = file_name.replace(".html.tera", "");
    relative_directory_path.push(file_name_without_extension.clone());
    let relative_template_path = relative_directory_path.join("/");
    let mut template_name = relative_template_path;
    if namespace != "" {
        template_name = format!("{}::{}", namespace, template_name);
    }

    if name.is_some() {
        template_name = name.unwrap();
    }

    let mut sort_order = template_file_list.len() as u32;

    for (template_file_name, (template_file_sort_order, template_file_path)) in template_file_list.iter() {
        if template_file_path == file_path.as_str() {
            if template_file_name == template_name.as_str() {
                warn!("  * template already loaded: {file_path} as \"{template_name}\". skipping...");
                return Ok(());
            } else {
                warn!(
                    "  * template already loaded: {}, but as \"{}\". reloading anyway as \"{}\"...",
                    file_path, template_file_name, template_name
                );
            }
        } else if template_file_name == template_name.as_str() {
            warn!(
                "  * template with name {} already loaded from {}. overwriting...",
                template_file_name, template_file_path
            );
            sort_order = *template_file_sort_order;
        }
    }

    template_file_list.insert(template_name.clone(), (sort_order, file_path.clone()));

    Ok(())
}
