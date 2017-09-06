pub struct Entity {
    pub c : char,
    pub x : i32,
    pub y : i32,
}

impl Entity {
    pub fn new(character : char, x_point : i32, y_point : i32) -> Self {
        return Self {
            c : character,
            x : x_point,
            y : y_point,
        };
    }
}
