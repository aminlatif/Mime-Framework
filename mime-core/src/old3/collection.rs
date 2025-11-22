pub struct Collection<ModelType> {
    pub items: Vec<ModelType>,
    pub skip: u32,
    pub page: u32,
    pub per_page: u32,
}

impl<ModelType> Collection<ModelType> {
    pub fn new(skip: u32, page: u32, per_page: u32) -> Self {
        Self {
            items: vec![],
            skip,
            page,
            per_page,
        }
    }
}
