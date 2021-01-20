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
    fn update(&self, world: &World) {
        let mask = world.get_mask::<Position>();
        let mut positions = world.get_component::<Position>();

        for (entity, _) in world
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
    let mut engine = Engine::new(30.);

    engine.world.register_component::<Position>();

    let entity1 = engine.world.create_entity();
    engine
        .world
        .add_entity_component(entity1, Position::new(0, 1000));
    let entity2 = engine.world.create_entity();
    engine
        .world
        .add_entity_component(entity2, Position::new(56, 1789));

    engine.world.register_system(Gravity {});
    engine.run();
}
