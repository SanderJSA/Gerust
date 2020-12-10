struct World {}

impl World {
    pub fn new() -> World {
        World {}
    }

    pub fn entities(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_world() {
        let world = World::new();

        assert!(world.entities() == 0);
    }
}
