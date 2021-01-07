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
            positions.get_mut(entity.clone()).y -= 10;
            println!("{:?}", positions.get(entity.clone()));
        }
    }
}

#[test]
fn basic_ecs() {
    let mut world = World::new();

    world.register_component::<Position>();

    let entity = world.create_entity();
    world.add_entity_component(entity, Position::new(0, 100));

    world.register_system(Gravity {});

    for _ in 0..9 {
        world.update();
    }

    assert!(world.get_component::<Position>().get(entity) == &Position::new(0, 10));
}
