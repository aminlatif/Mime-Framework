pub struct TemplatePath{
    pub path: String,
    pub namespace: Option<String>,
    pub name: Option<String>,
}

impl TemplatePath {
    pub fn new(path: String, namespace: Option<String>, name: Option<String>) -> Self {
        Self {
            path,
            namespace,
            name,
        }
    }
}
