use super::{Component, EntityIndex};
use std::collections::HashMap;

pub trait StorageTrait<T: Component> {
    /// Create a new component Storage
    fn new() -> Self;

    /// Add an entity component to Storage
    fn add_entity(&mut self, index: EntityIndex);

    /// Remove an entity component from Storage
    fn remove_entity(&mut self, index: EntityIndex);

    /// Get a ref to an entity component from Storage
    /// Will panic if entity does not exist
    fn get(&self, index: EntityIndex) -> &T;

    /// Get a mut ref to an entity component from Storage
    /// Will panic if entity does not exist
    fn get_mut(&mut self, index: EntityIndex) -> &mut T;
}

pub struct Storage<T: Component> {
    entity_components: HashMap<EntityIndex, T>,
}

impl<T: Component> StorageTrait<T> for Storage<T> {
    fn new() -> Storage<T> {
        Storage {
            entity_components: HashMap::new(),
        }
    }

    fn add_entity(&mut self, index: EntityIndex) {
        self.entity_components.insert(index, T::new());
    }

    fn remove_entity(&mut self, index: EntityIndex) {
        self.entity_components.remove(&index);
    }

    fn get(&self, index: EntityIndex) -> &T {
        self.entity_components.get(&index).unwrap()
    }

    fn get_mut(&mut self, index: EntityIndex) -> &mut T {
        self.entity_components.get_mut(&index).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct BasicComponent();
    impl Component for BasicComponent {
        fn new() -> BasicComponent {
            BasicComponent {}
        }
    }

    #[test]
    fn add_entities() {
        let mut storage: Storage<BasicComponent> = Storage::new();

        storage.add_entity(10);
        storage.add_entity(3);
        storage.add_entity(5420);

        assert!(storage.entity_components.len() == 3)
    }
}
