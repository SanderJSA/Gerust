mod component;
mod entity;
mod system;

pub use component::Component;
use entity::Entity;
pub use entity::EntityIndex;
pub use system::System;

use std::any::TypeId;
use std::collections::HashMap;

/// World represents the global state of the game
pub struct World {
    entities: HashMap<EntityIndex, Entity>,

    components: HashMap<TypeId, Box<dyn Component>>,

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
    pub fn create_entity(&mut self) -> u64 {
        let index = self.next_free;
        self.entities.insert(index, Entity::new(self.next_free));
        self.next_free += 1;
        index
    }

    /// Register a component for a World
    pub fn register_component<T: 'static + Component>(&mut self) {
        self.components
            .insert(TypeId::of::<T>(), Box::new(T::new()));
    }

    /// Retrieve a component in a World
    /// Will unwrap if component has not been registered before
    pub fn get_component<T: 'static + Component>(&mut self) -> &mut dyn Component {
        self.components
            .get_mut(&TypeId::of::<T>())
            .expect("Could not get component, has it been registered ?")
            .as_mut()
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
    }

    #[test]
    fn create_two_entity() {
        let mut world = World::new();

        let entity1 = world.create_entity();
        let entity2 = world.create_entity();

        assert!(entity1 == 0);
        assert!(entity2 == 1);
    }

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
