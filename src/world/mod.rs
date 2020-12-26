mod component;
mod entity;
mod system;

pub use component::Component;
pub use entity::Entity;
pub use system::System;

use std::any::TypeId;
use std::collections::HashMap;

/// World represents the global state of the game
pub struct World {
    entities: HashMap<u64, Entity>,

    components: HashMap<TypeId, Box<dyn Component>>,

    next_free: u64,
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
        self.entities
            .insert(self.next_free, Entity::new(self.next_free));
        self.next_free += 1;
        self.next_free - 1
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
}
