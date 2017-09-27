extern crate specs;
use self::specs::*;
use components::*;

#[derive(Debug, PartialEq)]
pub struct InformationComponent {
    pub name: String,
    pub description: String,
}

impl Component for InformationComponent {
    type Storage = VecStorage<Self>;
}

