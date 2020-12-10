pub struct Entity {
    index: u64,
}

impl Entity {
    pub fn new(index: u64) -> Entity {
        Entity { index }
    }

    pub fn index(&self) -> u64 {
        self.index
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn new_entity() {
        let entity = Entity::new(0);

        assert!(entity.index() == 0);
    }
}
