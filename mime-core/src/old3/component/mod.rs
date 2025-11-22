use std::{any::Any, ops::{Deref, DerefMut}};

mod component_scope;
mod component_type;

pub use component_scope::ComponentScope;
pub use component_type::ComponentType;

use crate::container::Container;

pub trait Component: Send + Sync + 'static {
    // fn new(container: &Container) -> Self;
}


// pub struct ComponentBox {
//     pub component: Box<dyn Component>,
//     pub scope: ComponentScope,
//     pub component_type: ComponentType,
// }

// impl<T: Component> ComponentBox<T> {
//     pub fn new(component: T, scope: ComponentScope, component_type: ComponentType) -> Self {
//         Self {
//             component: Box::new(component),
//             scope,
//             component_type,
//         }
//     }
// }

// impl<T: Component> Deref for ComponentBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.component
//     }
// }

// impl<T: Component> DerefMut for ComponentBox<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.component
//     }
// }
