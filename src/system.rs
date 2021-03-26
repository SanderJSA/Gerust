use crate::{Engine, UpdateStatus};
use sdl2::event::Event;

pub trait System {
    /// Called on every frame, Returning Ok(UpdateStatus::Exit) exits the engine
    fn update(&self, engine: &Engine, events: &[Event]) -> Result<UpdateStatus, String>;
}
