use super::World;

pub trait System {
    fn update(&self, world: &mut World);
}
