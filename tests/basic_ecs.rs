use gerust::*;

struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Position {}
impl Position {
    fn new(x: f32, y: f32, z: f32) -> Position {
        Position { x, y, z }
    }
}

struct Gravity {}
impl System for Gravity {
    fn update(&self, world: &mut World) {
        let mask = world.get_mask::<Position>();
        let mut positions = world.get_component::<Position>();

        for (entity, _) in world
            .entities
            .iter()
            .filter(|(_, entity)| entity.components_mask() == mask)
        {
            positions.get_mut(entity.clone()).y -= 9.8;
        }
    }
}

#[test]
fn basic_ecs() {
    let mut world = World::new();

    world.register_component::<Position>();

    let entity = world.create_entity();
    world.add_entity_component(entity, Position::new(0.0, 100.0, 0.0));
}
