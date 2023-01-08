use ecs::World;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_and_get_resource_immutably() {
        let world = initialize_world();
        let fps = world.get_resource::<FpsResource>().unwrap();
        assert_eq!(fps.0, 60);
    }

    #[test]
    fn get_resources_mutably() {
        let mut world = initialize_world();
        let fps = world.get_resource_mut::<FpsResource>().unwrap();
        fps.0 = 65;
        assert_eq!(fps.0, 65);
        let new_fps = world.get_resource_mut::<FpsResource>().unwrap();
        assert_eq!(new_fps.0, 65);
    }
}

fn initialize_world() -> World {
    let mut world = World::new();
    world.add_resource(FpsResource(60));
    world
}

#[derive(Debug)]
struct FpsResource(pub u32);

impl std::ops::Deref for FpsResource {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
