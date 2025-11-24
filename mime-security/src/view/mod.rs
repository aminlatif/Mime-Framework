use mime_web::types::TemplatePath;

pub fn get_templates() -> Vec<TemplatePath> {
    vec![
        TemplatePath::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*").to_string(), Some("security".to_string()), None),
        // TemplatePath::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/user_layout.html").to_string(), None, Some("layout".to_string())),
        // TemplatePath::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/user_layout.html").to_string(), Some("layout".to_string()), None),
    ]
}