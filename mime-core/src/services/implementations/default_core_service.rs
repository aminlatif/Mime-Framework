use crate::services::CoreService;

pub struct DefaultCoreService {}

impl DefaultCoreService {
    pub fn new() -> Self {
        Self {}
    }
}

impl CoreService for DefaultCoreService {}
