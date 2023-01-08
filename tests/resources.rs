extern crate core;

use ecs::World;

#[test]
fn create_and_get_resource_immutably() {
    let mut world = World::new();
    world.add_resource(FpsResource(60));
    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 60);
}

#[derive(Debug)]
struct FpsResource(pub u32);

impl std::ops::Deref for FpsResource {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
