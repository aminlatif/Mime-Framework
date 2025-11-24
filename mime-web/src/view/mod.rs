use crate::types::TemplatePath;

pub fn get_templates() -> Vec<TemplatePath> {
    vec![
        TemplatePath::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/templates/layout.html.tera").to_string(),
            None,
            None,
        ),
        TemplatePath::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*").to_string(),
            None,
            None,
        ),
        TemplatePath::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*").to_string(),
            Some("base".to_string()),
            None,
        ),
    ]
}
