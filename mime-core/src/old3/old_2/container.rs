use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Container {
    services: HashMap<TypeId, Box<dyn Any>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    // Register a service implementing some trait
    pub fn register<T: 'static>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }

    // Resolve a service
    pub fn resolve<T: 'static>(&self) -> Option<&T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}
