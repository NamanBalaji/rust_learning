use std::ops::Deref;

use ecs::World;

#[test]
fn create_and_get_resource_immutably() {
    let world = initialize_world();
    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 60);
}

#[test]
fn get_resource_mutably() {
    let mut world = initialize_world();
    {
        let fps = world.get_resource_mut::<FpsResource>().unwrap();
        fps.0 += 1;
    }

    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 61)
}

#[test]
fn delete_resource() {
    let mut world = initialize_world();
    world.delete_resource::<FpsResource>();
    let delete_resource = world.get_resource::<FpsResource>();
    assert!(delete_resource.is_none());
}

fn initialize_world() -> World {
    let mut world = World::new();
    world.add_resource(FpsResource(60));

    world
}

#[derive(Debug)]
struct FpsResource(pub u32);

impl Deref for FpsResource {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
