use std::process;

extern crate specs;
use self::specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub enum TileType {
    Impassable,
    Passable,
}

#[derive(Debug)]
pub struct TileComponent {
    pub tile_type: TileType,
}

impl Component for TileComponent {
    type Storage = VecStorage<Self>;
}

