use gerust::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: u32,
    y: u32,
}

impl Component for Position {}
impl Position {
    fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }
}

struct Gravity {}
impl System for Gravity {
    fn update(&self, engine: &Engine, _: &[Event]) -> Result<UpdateStatus, String> {
        let mask = engine.get_mask::<Position>();
        let mut positions = engine.get_component::<Position>();

        for (entity, _) in engine
            .entities
            .iter()
            .filter(|(_, entity)| entity.components_mask() == mask)
        {
            positions.get_mut(*entity).y = positions.get(*entity).y.saturating_sub(2);
        }
        Ok(UpdateStatus::Continue)
    }
}

struct Render {}
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
            .iter()
            .filter(|(_, entity)| entity.components_mask() == mask)
        {
            let entity_pos = positions.get(*entity);
            canvas
                .fill_rect(Rect::new(entity_pos.x as i32, entity_pos.y as i32, 10, 10))
                .unwrap();
        }
        canvas.present();
        Ok(UpdateStatus::Continue)
    }
}

struct Exit {}
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

fn main() {
    let mut engine = Engine::new("Basic Engine", 640, 480).expect("Could not initialize engine");

    engine.register_component::<Position>();

    let entity1 = engine.create_entity();
    engine.add_entity_component(entity1, Position::new(100, 1000));
    let entity2 = engine.create_entity();
    engine.add_entity_component(entity2, Position::new(56, 800));

    engine.register_system(Gravity {});
    engine.register_system(Render {});
    engine.register_system(Exit {});
    engine.run().expect("Could not run engine");
}
