use std::any::Any;

use crate::entities::query::Query;
use crate::entities::Entities;
use crate::resource::Resource;

mod custom_errors;
mod entities;
mod resource;

#[derive(Default)]
pub struct World {
    resources: Resource,
    entities: Entities,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, resource_date: impl Any) {
        self.resources.add(resource_date);
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /// Query of a resource and get a mutable to it. The type of the resource must be added.
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn delete_resource<T: Any>(&mut self) {
        self.resources.remove::<T>()
    }

    pub fn register_component<T: Any>(&mut self) {
        self.entities.register_component::<T>();
    }

    pub fn create_entity(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }

    pub fn query(&self) -> Query {
        Query::new()
    }
}
