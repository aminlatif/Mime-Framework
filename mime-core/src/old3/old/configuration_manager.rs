pub struct ConfigurationManager {
    email_service_username: String,
    email_service_password: String,
    database_connection_string: Option<String>,
    api_key: Option<String>,
}

impl ConfigurationManager {
    pub fn new() -> Self {
        // This should use something like clap to read the configuration from
        // the command line
        // But this is good enough to show a configuration manager.
        Self {
            email_service_username: "user".to_string(),
            email_service_password: "pass".to_string(),
            database_connection_string: None,
            api_key: Some("api_key".to_string()),
        }
    }

    pub fn get_email_service_username(&self) -> &str {
        &self.email_service_username
    }

    pub fn get_email_service_password(&self) -> &str {
        &self.email_service_password
    }

    pub fn get_database_connection_string(&self) -> Option<&str> {
        self.database_connection_string.as_ref().map(|s| s.as_str())
    }

    pub fn get_api_key(&self) -> Option<&str> {
        self.api_key.as_ref().map(|s| s.as_str())
    }
}
