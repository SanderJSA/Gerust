mod component;
mod entity;
mod system;

pub use component::Component;
pub use entity::Entity;
pub use system::System;

/// World represents the global state of the game
pub struct World {
    entities: u64,
}

impl World {
    /// Create a new World
    pub fn new() -> World {
        World { entities: 0 }
    }

    /// Get number of living entities
    pub fn entities(&self) -> u64 {
        self.entities
    }

    /// Create a new entity and return it
    pub fn add_entity(&mut self) -> Entity {
        let entity = Entity::new(self.entities);
        self.entities += 1;
        entity
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_world() {
        let world = World::new();

        assert!(world.entities() == 0);
    }

    #[test]
    fn get_index_one() {
        let mut world = World::new();

        let entity = world.add_entity();

        assert!(entity.index() == 0);
    }
}
