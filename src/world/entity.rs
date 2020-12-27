/// An index pointing to a unique entity
pub type EntityIndex = u64;

/// A unique entity
pub struct Entity {
    index: u64,
}

impl Entity {
    /// Create a new Entity
    /// Note: Only the World should create Entities
    pub fn new(index: u64) -> Entity {
        Entity { index }
    }

    /// Get the index of the entity
    pub fn index(&self) -> u64 {
        self.index
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn new_entity() {
        let entity = Entity::new(0);

        assert!(entity.index() == 0);
    }
}
