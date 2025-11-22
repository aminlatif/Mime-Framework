use super::data_collector::DataCollector;

pub struct SimpleDataCollector {
    api_key: String,
}

impl SimpleDataCollector {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl DataCollector for SimpleDataCollector {
    fn collect_data(&self) -> Vec<String> {
        vec!["data1".to_string(), "data2".to_string()]
    }
}