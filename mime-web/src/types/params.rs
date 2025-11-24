use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Params {
    pub page: Option<u64>,
    pub items_per_page: Option<u64>,
}