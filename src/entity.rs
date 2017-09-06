#[derive (Debug, Clone)]
pub struct Entity {
    pub ident: String,
    pub c: char,
    pub x: i32,
    pub y: i32,
}

impl Entity {
    pub fn new(ident: String, c: char, x: i32, y: i32) -> Self {
        return Self {
            ident: ident,
            c: c,
            x: x,
            y: y,
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
