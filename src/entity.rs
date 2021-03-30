/// An index pointing to a unique entity
pub type EntityIndex = u64;
pub type ComponentMask = u64;

/// A unique entity containing the components it's attached to
pub struct Entity {
    components_mask: ComponentMask,
}

impl Entity {
    /// Create a new Entity wit
    pub fn new() -> Entity {
        Entity { components_mask: 0 }
    }

    /// Get the mask of the entity
    pub fn components_mask(&self) -> ComponentMask {
        self.components_mask
    }

    pub fn add_component(&mut self, component_mask: ComponentMask) {
        self.components_mask |= component_mask;
    }

    pub fn remove_component(&mut self, component_mask: ComponentMask) {
        self.components_mask &= !component_mask;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_entity() {
        let entity = Entity::new();

        assert!(entity.components_mask() == 0);
    }

    #[test]
    fn add_components() {
        let mut entity = Entity::new();

        entity.add_component(0b100000);
        entity.add_component(0b1000);

        assert!(entity.components_mask() == 0b101000);
    }

    #[test]
    fn remove_components() {
        let mut entity = Entity::new();
        entity.add_component(0b100000);
        entity.add_component(0b1000);
        entity.add_component(0b100);

        entity.remove_component(0b100);
        entity.remove_component(0b100000);

        assert!(entity.components_mask() == 0b1000);
    }
}
