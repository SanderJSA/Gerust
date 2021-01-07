mod component;
mod entity;
mod storage;
mod system;

pub use component::Component;
pub use entity::{ComponentMask, EntityIndex};
pub use storage::{Storage, StorageTrait};
pub use system::System;

use entity::Entity;
use std::any::{Any, TypeId};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

/// World represents the global state of the game
pub struct World {
    /// Current entites in our World
    pub entities: HashMap<EntityIndex, Entity>,

    /// Component storage
    /// Note: There is an Any instead of a StorageTrait, as Rust doesn't support
    /// dyn with generic traits.
    /// This means we need to downcast to our Storage impl
    components: HashMap<TypeId, Box<dyn Any>>,

    /// The associated mask for each Component
    component_masks: HashMap<TypeId, ComponentMask>,

    /// The registered systems
    systems: Vec<Box<dyn System>>,

    /// The next available entity index
    next_free: EntityIndex,
}

impl World {
    /// Create a new World
    pub fn new() -> World {
        World {
            entities: HashMap::new(),
            components: HashMap::new(),
            component_masks: HashMap::new(),
            systems: vec![],
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

    /// Add a component to an entity
    /// Will panic if given index doesn't exist or Component has not been registered
    pub fn add_entity_component<T: 'static + Component>(
        &mut self,
        entity_index: EntityIndex,
        component: T,
    ) {
        self.entities
            .get_mut(&entity_index)
            .unwrap()
            .add_component(self.component_masks[&TypeId::of::<T>()]);
        self.get_component::<T>()
            .add_entity(entity_index, component);
    }

    /// Remove a component from an entity
    /// Will panic if given index doesn't exist or Component has not been registered
    pub fn remove_entity_component<T: 'static + Component>(&mut self, index: EntityIndex) {
        self.entities
            .get_mut(&index)
            .unwrap()
            .remove_component(self.component_masks[&TypeId::of::<T>()]);
    }

    /// Register a component for a World
    pub fn register_component<T: 'static + Component>(&mut self) {
        let storage: Storage<T> = Storage::new();
        self.components
            .insert(TypeId::of::<T>(), Box::new(RefCell::new(storage)));

        // Current ComponentMask creation relies on the fact that
        // a Component cannot be unregistered
        let mask = 1u64 << self.component_masks.len();
        self.component_masks.insert(TypeId::of::<T>(), mask);
    }

    /// Retrieve a component in a World
    /// Will unwrap if component has not been registered before
    pub fn get_component<T: 'static + Component>(&self) -> RefMut<Storage<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .expect("Could not get component, has it been registered ?")
            .downcast_ref::<RefCell<Storage<T>>>()
            .unwrap()
            .borrow_mut()
    }

    pub fn get_mask<T: 'static + Component>(&self) -> ComponentMask {
        self.component_masks[&TypeId::of::<T>()]
    }

    pub fn register_system<T: 'static + System>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn update(&self) {
        for system in self.systems.iter() {
            system.update(self);
        }
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
    impl Component for BasicComponent {}
    impl BasicComponent {
        fn new() -> BasicComponent {
            BasicComponent {}
        }
    }

    #[test]
    fn register_one_component() {
        let mut world = World::new();

        world.register_component::<BasicComponent>();

        assert!(world.components.len() == 1);
        assert!(world.component_masks[&TypeId::of::<BasicComponent>()] == 1);
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
        let world = World::new();

        world.get_component::<BasicComponent>();
    }

    #[test]
    fn add_entity_component() {
        let mut world = World::new();
        world.register_component::<BasicComponent>();
        let entity = world.create_entity();

        world.add_entity_component(entity, BasicComponent::new());

        assert!(
            world.entities[&entity].components_mask()
                == world.component_masks[&TypeId::of::<BasicComponent>()]
        );
    }

    #[test]
    fn remove_entity_component() {
        let mut world = World::new();
        world.register_component::<BasicComponent>();
        let entity = world.create_entity();
        world.add_entity_component(entity, BasicComponent::new());

        world.remove_entity_component::<BasicComponent>(entity);

        assert!(world.entities[&entity].components_mask() == 0);
    }
}
