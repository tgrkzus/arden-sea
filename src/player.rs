use entity::*;

pub struct Player {
    entity: Entity,
}

impl Player {
    pub fn new(ident: String, c: char, x: i32, y: i32) -> Self {
        return Self {
            entity: Entity::new(ident, c, x, y),
        };
    }

    /// Returns the internal mutable entity object for the player
    pub fn get_entity(&mut self) -> &mut Entity {
        return &mut self.entity;
    }
}
