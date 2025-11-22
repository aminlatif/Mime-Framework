use super::data_collector::DataCollector;

pub struct SqlDataCollector {
    connection_string: String,
}

impl SqlDataCollector {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

impl DataCollector for SqlDataCollector {
    fn collect_data(&self) -> Vec<String> {
        vec!["sql_data1".to_string(), "sql_data2".to_string()]
    }
}
