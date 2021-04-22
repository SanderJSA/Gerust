use gerust::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const SQUARE_SIZE: u32 = 25;
const MAX_VELOCITY: i32 = 25;

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {}
impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Velocity {
    x: i32,
    y: i32,
    is_movable: bool,
}

impl Component for Velocity {}
impl Velocity {
    fn new(is_movable: bool) -> Velocity {
        Velocity {
            x: 0,
            y: 0,
            is_movable,
        }
    }
}

struct Gravity;
impl System for Gravity {
    fn update(&self, engine: &Engine, _: &[Event]) -> Result<UpdateStatus, String> {
        let mask = engine.get_mask::<Velocity>();
        let mut velocities = engine.get_component::<Velocity>();

        for (entity, _) in engine
            .entities
            .borrow()
            .iter()
            .filter(|(_, entity)| entity.components_mask() & mask != 0)
        {
            let velocity = velocities.get_mut(*entity);
            if velocity.is_movable && velocity.y < MAX_VELOCITY {
                velocity.y += 1;
            }
        }
        Ok(UpdateStatus::Continue)
    }
}

struct Render;
impl System for Render {
    fn update(&self, engine: &Engine, _: &[Event]) -> Result<UpdateStatus, String> {
        let mask = engine.get_mask::<Position>();
        let positions = engine.get_component::<Position>();
        let mut canvas = engine.canvas.borrow_mut();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for (entity, _) in engine
            .entities
            .borrow()
            .iter()
            .filter(|(_, entity)| entity.components_mask() & mask == mask)
        {
            let entity_pos = positions.get(*entity);
            canvas
                .fill_rect(Rect::new(
                    entity_pos.x as i32,
                    entity_pos.y as i32,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                ))
                .unwrap();
        }
        canvas.present();
        Ok(UpdateStatus::Continue)
    }
}

struct Exit;
impl System for Exit {
    fn update(&self, _: &Engine, events: &[Event]) -> Result<UpdateStatus, String> {
        for event in events {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => return Ok(UpdateStatus::Exit),
                _ => {}
            }
        }
        Ok(UpdateStatus::Continue)
    }
}

struct SpawnOnClick;
impl System for SpawnOnClick {
    fn update(&self, engine: &Engine, events: &[Event]) -> Result<UpdateStatus, String> {
        for event in events {
            match event {
                Event::MouseButtonDown { x, y, .. } => {
                    let entity = engine.create_entity();
                    engine.add_entity_component(entity, Position::new(*x, *y));
                    engine.add_entity_component(entity, Velocity::new(true));
                }
                _ => {}
            }
        }
        Ok(UpdateStatus::Continue)
    }
}

struct ApplyVelocity;
impl System for ApplyVelocity {
    fn update(&self, engine: &Engine, _: &[Event]) -> Result<UpdateStatus, String> {
        let mask = engine.get_mask::<Position>() | engine.get_mask::<Velocity>();
        let mut positions = engine.get_component::<Position>();
        let velocities = engine.get_component::<Velocity>();

        for (entity, _) in engine
            .entities
            .borrow()
            .iter()
            .filter(|(_, entity)| entity.components_mask() & mask == mask)
        {
            let entity_pos = positions.get_mut(*entity);
            let entity_velocity = velocities.get(*entity);
            entity_pos.x += entity_velocity.x;
            entity_pos.y += entity_velocity.y;
        }
        Ok(UpdateStatus::Continue)
    }
}

/// Collider for our engine, is run after ApplyVelocity and fixes any colliding rectangle
struct Collision;
impl System for Collision {
    fn update(&self, engine: &Engine, _: &[Event]) -> Result<UpdateStatus, String> {
        let mask = engine.get_mask::<Position>() | engine.get_mask::<Velocity>();
        let mut positions = engine.get_component::<Position>();
        let mut velocities = engine.get_component::<Velocity>();

        let entities_ref = engine.entities.borrow();
        let entities: Vec<&u64> = entities_ref
            .iter()
            .filter(|(_, entity)| entity.components_mask() & mask == mask)
            .map(|(entity_id, _)| entity_id)
            .collect();
        for i in 0..entities.len() as u64 {
            for j in (i + 1)..entities.len() as u64 {
                let square1 = positions.get(i);
                let square2 = positions.get(j);
                if is_colliding(square1, square2) {
                    if velocities.get_mut(i).y < velocities.get_mut(j).y {
                        positions.get_mut(j).y = square1.y - SQUARE_SIZE as i32;
                        velocities.get_mut(j).y = 0;
                    } else {
                        positions.get_mut(i).y = square2.y - SQUARE_SIZE as i32;
                        velocities.get_mut(i).y = 0;
                    }
                }
            }
        }

        Ok(UpdateStatus::Continue)
    }
}

fn is_colliding(square1: &Position, square2: &Position) -> bool {
    let size = SQUARE_SIZE as i32;
    square1.x < square2.x + size
        && square1.x + size > square2.x
        && square1.y < square2.y + size
        && square1.y + size > square2.y
}

fn main() {
    let mut engine = Engine::new("Basic Engine", 640, 480).expect("Could not initialize engine");

    engine.register_component::<Position>();
    engine.register_component::<Velocity>();

    let entity1 = engine.create_entity();
    engine.add_entity_component(entity1, Position::new(100, 1000));
    engine.add_entity_component(entity1, Velocity::new(true));
    let entity2 = engine.create_entity();
    engine.add_entity_component(entity2, Position::new(56, 800));
    engine.add_entity_component(entity2, Velocity::new(true));

    // Add bottom row if immovable objects
    for i in 0..(640 / SQUARE_SIZE) {
        let entity = engine.create_entity();
        engine.add_entity_component(entity, Position::new((i * SQUARE_SIZE) as i32, 470));
        engine.add_entity_component(entity, Velocity::new(false));
    }

    engine.register_system(Exit {});
    engine.register_system(SpawnOnClick {});
    engine.register_system(Gravity {});
    engine.register_system(ApplyVelocity {});
    engine.register_system(Collision {});
    engine.register_system(Render {});
    engine.run().expect("Could not run engine");
}
