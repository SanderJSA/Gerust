use crate::Engine;

pub trait System {
    fn update(&self, engine: &Engine);
}
