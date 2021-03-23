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
            positions.get_mut(entity.clone()).y -= 10;
            println!("{:?}", positions.get(entity.clone()));
        }
    }
}

#[test]
fn basic_ecs() {
    let mut engine = Engine::default();

    engine.register_component::<Position>();

    let entity = engine.create_entity();
    engine.add_entity_component(entity, Position::new(0, 100));

    engine.register_system(Gravity {});

    for _ in 0..9 {
        engine.update();
    }

    assert!(engine.get_component::<Position>().get(entity) == &Position::new(0, 10));
}
