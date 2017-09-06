use entity::*;

pub struct Player {
    entity: Entity,
}

impl Player {
    pub fn new(character: char, x_point: i32, y_point: i32) -> Self {
        return Self {
            entity: Entity::new(character, x_point, y_point),
        };
    }

    /// Returns the internal mutable entity object for the player
    pub fn get_entity(&mut self) -> &mut Entity {
        return &mut self.entity;
    }
}
