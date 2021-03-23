mod component;
mod entity;
mod storage;
mod system;

pub use component::Component;
pub use entity::{ComponentMask, EntityIndex};
pub use storage::{Storage, StorageTrait};
pub use system::System;

use entity::Entity;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::video::Window;
use sdl2::{event::Event, pixels::Color};
use std::any::{Any, TypeId};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

/// Engine represents the global state of the game
pub struct Engine {
    /// Current entites in our Engine
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

    /// Rendering is done on the canvas,
    canvas: Canvas<Window>,
}

impl Engine {
    /// Create a new Engine
    pub fn new(title: &str, width: u32, height: u32) -> Result<Engine, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let mut events = sdl_context.event_pump()?;

        Ok(Engine {
            entities: HashMap::new(),
            components: HashMap::new(),
            component_masks: HashMap::new(),
            systems: vec![],
            next_free: 0,
            canvas,
        })
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

    /// Register a component for a Engine
    pub fn register_component<T: 'static + Component>(&mut self) {
        let storage: Storage<T> = Storage::new();
        self.components
            .insert(TypeId::of::<T>(), Box::new(RefCell::new(storage)));

        // Current ComponentMask creation relies on the fact that
        // a Component cannot be unregistered
        let mask = 1u64 << self.component_masks.len();
        self.component_masks.insert(TypeId::of::<T>(), mask);
    }

    /// Retrieve a component in a Engine
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

impl Default for Engine {
    fn default() -> Self {
        Engine::new("SDL2", 640, 480).expect("Could not initialize engine")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_engine() {
        let engine = Engine::default();

        assert!(engine.entities.len() == 0);
        assert!(engine.components.len() == 0);
        assert!(engine.next_free == 0);
    }

    #[test]
    fn create_one_entity() {
        let mut engine = Engine::default();

        let entity = engine.create_entity();

        assert!(entity == 0);
        assert!(engine.entities.get(&entity).is_some());
    }

    #[test]
    fn create_two_entity() {
        let mut engine = Engine::default();

        let entity1 = engine.create_entity();
        let entity2 = engine.create_entity();

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
        let mut engine = Engine::default();

        engine.register_component::<BasicComponent>();

        assert!(engine.components.len() == 1);
        assert!(engine.component_masks[&TypeId::of::<BasicComponent>()] == 1);
    }

    #[test]
    fn get_one_component() {
        let mut engine = Engine::default();

        engine.register_component::<BasicComponent>();

        engine.get_component::<BasicComponent>();
    }

    #[test]
    #[should_panic(expected = "Could not get component, has it been registered ?")]
    fn get_component_not_registered() {
        let engine = Engine::default();

        engine.get_component::<BasicComponent>();
    }

    #[test]
    fn add_entity_component() {
        let mut engine = Engine::default();
        engine.register_component::<BasicComponent>();
        let entity = engine.create_entity();

        engine.add_entity_component(entity, BasicComponent::new());

        assert!(
            engine.entities[&entity].components_mask()
                == engine.component_masks[&TypeId::of::<BasicComponent>()]
        );
    }

    #[test]
    fn remove_entity_component() {
        let mut engine = Engine::default();
        engine.register_component::<BasicComponent>();
        let entity = engine.create_entity();
        engine.add_entity_component(entity, BasicComponent::new());

        engine.remove_entity_component::<BasicComponent>(entity);

        assert!(engine.entities[&entity].components_mask() == 0);
    }
}

/*
*
       'mainloop: loop {
           for event in events.poll_iter() {
               match event {
                   Event::Quit { .. }
                   | Event::KeyDown {
                       keycode: Option::Some(Keycode::Escape),
                       ..
                   } => break 'mainloop,
                   Event::MouseButtonDown { x, y, .. } => {
                       canvas.fill_rect(Rect::new(x, y, 1, 1))?;
                       canvas.present();
                   }
                   _ => {}
               }
           }
       }

       */

/*
*
*
   pub fn run(self) {
       let delay = Duration::from_secs_f64(1. / self.framerate);
       loop {
           let frame_start = Instant::now();
           self.engine.update();
           thread::sleep(delay - frame_start.elapsed());
       }
   }
   */
