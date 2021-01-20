mod world;

use std::thread;
use std::time::{Duration, Instant};

pub use world::{Component, ComponentMask, EntityIndex, Storage, StorageTrait, System, World};

pub struct Engine {
    pub world: World,
    framerate: f64,
}

impl Engine {
    pub fn new(framerate: f64) -> Engine {
        Engine {
            world: World::new(),
            framerate,
        }
    }

    pub fn run(self) {
        let delay = Duration::from_secs_f64(1. / self.framerate);
        loop {
            let frame_start = Instant::now();
            self.world.update();
            thread::sleep(delay - frame_start.elapsed());
        }
    }
}
