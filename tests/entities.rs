use ecs::World;

#[test]
fn create_entity() {
    let location = Location(42.0, 42.0);
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();
    world
        .create_entity()
        .with_component(Location(42.0, 42.0))
        .with_size(Size(10.0));
}

struct Location(pub f32, pub f32);

struct Size(pub f32);
