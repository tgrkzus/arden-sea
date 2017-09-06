#[derive (Debug)]
pub struct Entity {
    pub c: char,
    pub x: i32,
    pub y: i32,
}

impl Entity {
    pub fn new(character: char, x_point: i32, y_point: i32) -> Self {
        return Self {
            c : character,
            x : x_point,
            y : y_point,
        };
    }

    /// Move's a entity to the given coords
    ///
    /// No verification of location
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Offsets the entity
    ///
    /// No verification of location
    pub fn offset(&mut self, x: i32, y: i32) {
        self.x = self.x + x;
        self.y = self.y + y;
    }
}
