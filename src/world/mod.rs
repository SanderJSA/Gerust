mod component;
mod entity;
mod storage;
mod system;

pub use component::Component;
pub use entity::EntityIndex;
pub use system::System;

use entity::Entity;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use storage::{Storage, StorageTrait};

/// World represents the global state of the game
pub struct World {
    /// Current entites in our World
    entities: HashMap<EntityIndex, Entity>,

    /// Component storage
    /// Note: There is an Any instead of a StorageTrait, as Rust doesn't support
    /// dyn with generic traits.
    /// This means we need to downcast to our Storage impl
    components: HashMap<TypeId, Box<dyn Any>>,

    /// The next available entity index
    next_free: EntityIndex,
}

impl World {
    /// Create a new World
    pub fn new() -> World {
        World {
            entities: HashMap::new(),
            components: HashMap::new(),
            next_free: 0,
        }
    }

    /// Create a new entity and return its index
    pub fn create_entity(&mut self) -> EntityIndex {
        let index = self.next_free;
        self.entities.insert(index, Entity::new());
        self.next_free += 1;
        index
    }

    /// Register a component for a World
    pub fn register_component<T: 'static + Component>(&mut self) {
        let storage: Storage<T> = Storage::new();
        self.components.insert(TypeId::of::<T>(), Box::new(storage));
    }

    /// Retrieve a component in a World
    /// Will unwrap if component has not been registered before
    pub fn get_component<T: 'static + Component>(&mut self) -> &mut Storage<T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .expect("Could not get component, has it been registered ?")
            .downcast_mut()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_world() {
        let world = World::new();

        assert!(world.entities.len() == 0);
        assert!(world.components.len() == 0);
        assert!(world.next_free == 0);
    }

    #[test]
    fn create_one_entity() {
        let mut world = World::new();

        let entity = world.create_entity();

        assert!(entity == 0);
        assert!(world.entities.get(&entity).is_some());
    }

    #[test]
    fn create_two_entity() {
        let mut world = World::new();

        let entity1 = world.create_entity();
        let entity2 = world.create_entity();

        assert!(entity1 == 0);
        assert!(entity2 == 1);
    }

    #[derive(Debug)]
    struct BasicComponent();
    impl Component for BasicComponent {
        fn new() -> BasicComponent {
            BasicComponent {}
        }
    }

    #[test]
    fn register_one_component() {
        let mut world = World::new();

        world.register_component::<BasicComponent>();

        assert!(world.components.len() == 1);
    }

    #[test]
    fn get_one_component() {
        let mut world = World::new();

        world.register_component::<BasicComponent>();

        world.get_component::<BasicComponent>();
    }

    #[test]
    #[should_panic(expected = "Could not get component, has it been registered ?")]
    fn get_component_not_registered() {
        let mut world = World::new();

        world.get_component::<BasicComponent>();
    }
}
