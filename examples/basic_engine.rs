use gerust::*;

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
    fn update(&self, engine: &Engine) {
        let mask = engine.get_mask::<Position>();
        let mut positions = engine.get_component::<Position>();

        for (entity, _) in engine
            .entities
            .iter()
            .filter(|(_, entity)| entity.components_mask() == mask)
        {
            positions.get_mut(entity.clone()).y =
                positions.get(entity.clone()).y.saturating_sub(10);
            println!("{:?}", positions.get(entity.clone()));
        }
    }
}

fn main() {
    let mut engine = Engine::new("Basic Engine", 640, 480).expect("Could not initialize engine");

    engine.register_component::<Position>();

    let entity1 = engine.create_entity();
    engine.add_entity_component(entity1, Position::new(0, 1000));
    let entity2 = engine.create_entity();
    engine.add_entity_component(entity2, Position::new(56, 1789));

    engine.register_system(Gravity {});
    //engine.run();
}
