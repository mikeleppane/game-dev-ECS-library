use std::any::Any;

use crate::resource::Resource;

mod resource;

#[derive(Default)]
pub struct World {
    resources: Resource,
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
}
