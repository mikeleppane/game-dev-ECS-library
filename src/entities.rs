use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use color_eyre::Result;

use crate::custom_errors::CustomErrors;

pub mod query;

#[derive(Debug, Default)]
pub struct Entities {
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
    bit_masks: HashMap<TypeId, u32>,
}

impl Entities {
    pub fn register_component<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        let bit_mask = 1u32 << self.bit_masks.len() as u32;
        self.components.insert(type_id, vec![]);
        self.bit_masks.insert(type_id, bit_mask);
    }
    pub fn create_entity(&mut self) -> &mut Self {
        self.components
            .iter_mut()
            .for_each(|(key, component)| component.push(None));
        self
    }

    pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self> {
        let type_id = data.type_id();
        if let Some(components) = self.components.get_mut(&type_id) {
            let last_component = components
                .last_mut()
                .ok_or(CustomErrors::CreateComponentNeverCalled)?;
            *last_component = Some(Rc::new(RefCell::new(data)));
        } else {
            return Err(CustomErrors::ComponentNotRegistered.into());
        }
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use std::any::TypeId;
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn register_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let type_id = TypeId::of::<Health>();
        let health_components = entities.components.get(&type_id).unwrap();
        assert_eq!(health_components.len(), 0);
    }

    #[test]
    fn bitmask_updated_when_registering_entities() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let type_id = TypeId::of::<Health>();
        let health_mask = entities.bit_masks.get(&type_id).unwrap();
        assert_eq!(*health_mask, 1);

        entities.register_component::<Speed>();
        let type_id = TypeId::of::<Speed>();
        let mask = entities.bit_masks.get(&type_id).unwrap();
        assert_eq!(*mask, 2);

        entities.register_component::<Speed>();
        let type_id = TypeId::of::<Speed>();
        let mask = entities.bit_masks.get(&type_id).unwrap();
        assert_eq!(*mask, 4);
    }

    #[test]
    fn create_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity();
        let health = entities.components.get(&TypeId::of::<Health>()).unwrap();
        let speed = entities.components.get(&TypeId::of::<Speed>()).unwrap();
        assert_eq!(health.len(), speed.len());
        assert_eq!(health.len(), 1);
        assert!(health[0].is_none());
        assert!(speed[0].is_none());
    }

    #[test]
    fn with_component() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Speed(15))?;
        let first_health = &entities.components.get(&TypeId::of::<Health>()).unwrap()[0];
        let wrapped_health = first_health.as_ref().unwrap();
        let bh = wrapped_health.as_ref().borrow();
        let health = bh.borrow().downcast_ref::<Health>().unwrap();
        assert_eq!(health.0, 100);
        Ok(())
    }

    struct Health(u32);
    struct Speed(u32);
}
