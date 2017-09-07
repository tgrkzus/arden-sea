use component::*;

//#[derive (Debug, Clone)]
pub struct Entity {
    pub components: Vec<Box<Component>>,
}

impl Entity {
    pub fn new() -> Self {
        return Self {
            components : Vec::new(),
        };
    }

}
