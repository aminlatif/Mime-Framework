use dotenvy::dotenv;

pub fn read_environment_variables(
    prefix: Option<&str>,
) -> std::collections::HashMap<String, String> {
    let mut environemnt_variables = std::collections::HashMap::new();
    let modified_prefix = prefix.unwrap_or("MIME_");
    dotenv().ok();
    for (key, value) in std::env::vars() {
        if key.starts_with(modified_prefix) {
            let modified_key = key.replace(modified_prefix, "");
            environemnt_variables.insert(modified_key, value);
        }
    }

    environemnt_variables
}
