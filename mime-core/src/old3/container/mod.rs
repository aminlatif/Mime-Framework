use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::component::Component;

pub struct Container {
    pub components: HashMap<TypeId, Box<dyn Any>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    // Register a service implementing some trait
    pub fn register<T: 'static>(&mut self, component: T) {
        self.components
            .insert(TypeId::of::<T>(), Box::new(component));
    }

    // Resolve a service
    pub fn resolve<T: 'static>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}
