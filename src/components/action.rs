use std::process;

extern crate tcod;
use self::tcod::*;
use self::tcod::console::Root;
use self::tcod::input::KeyCode;

extern crate specs;
use self::specs::{Component, VecStorage, System, WriteStorage, ReadStorage,
                    Fetch, FetchMut, Join, EntitiesRes};

use components::position::CharacterPositionComponent;
use components::state::{TurnStateComponent, ActionState};
use game::WorldAttributes;

use world::map::{TileType, Tile, Map};

#[derive(Debug)]
pub struct ControllerComponent {
    pub controller: Controllers,
}

impl Component for ControllerComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub enum Controllers {
    Passive,
    Player,
    Enemy,
}

#[derive(Debug)]
pub enum Direction {
    N,
    E,
    W,
    S,
    NE,
    NW,
    SE,
    SW,
    None,
}


pub struct ActionControllerSystem;
impl<'a> System<'a> for ActionControllerSystem {
    type SystemData = (WriteStorage<'a, CharacterPositionComponent>,
                       ReadStorage<'a, TurnStateComponent>,
                       Fetch<'a, WorldAttributes>,
                       FetchMut<'a, Map>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, turns, attr, mut map) = data;


        for (position, turn) in (&mut positions, &turns).join() {
            match turn.action {
                ActionState::None => {
                    println!("NONE action");
                }
                ActionState::MoveBy => {
                    let mut new = Self::add_direction(&(position.x, position.y), &turn.direction);
                    
                    // Check world bounds
                    if !Self::check_valid_coords(new.0, new.1, 0, &attr) {
                        new = (position.x, position.y);
                    }

                    // Check collisions
                    match map.get_tile(new.0 as usize, new.1 as usize, 0).unwrap().tile_type {
                        TileType::Wall | TileType::Air => new = (position.x, position.y),
                        TileType::Ground => { },
                    }


                    position.x = new.0;
                    position.y = new.1;
                }
                ActionState::MoveTo => {
                    //position.x = turn.vec.0;
                    //position.y = turn.vec.1;
                }
                ActionState::Examine => {
                    let mut new = Self::add_direction(&(position.x, position.y), &turn.direction);

                    if Self::check_valid_coords(new.0, new.1, 0, &attr) {
                        map.set_tile(Tile { tile_type: TileType::Wall, }, new.0 as usize, new.1 as usize, 0);
                    }
                }
                ActionState::Attack => {
                    println!("ATTACK action");
                }
            }
        }
    }
}

pub struct ActionGeneratorSystem;
impl<'a> System<'a> for ActionGeneratorSystem {
    type SystemData = (WriteStorage<'a, TurnStateComponent>,
                       ReadStorage<'a, ControllerComponent>,
                       FetchMut<'a, RootConsole>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut turns, controllers, mut console) = data;

        for (turn, controller) in (&mut turns, &controllers).join() {
            match controller.controller {
                Controllers::Passive => {
                    ActionControllerSystem::generate_passive_action(turn);
                }
                Controllers::Player => {
                    // Action has been pregened!
                    //ActionControllerSystem::generate_player_action(turn, &mut console);
                }
                Controllers::Enemy => {
                    ActionControllerSystem::generate_enemy_action(turn);
                }
            }
        }
    }

}

impl ActionControllerSystem {
    pub fn direction_to_coords(dir: &Direction) -> (i32, i32) {
        match *dir {
            Direction::N => {
                return (0, -1);
            }
            Direction::E => {
                return (1, 0);
            }
            Direction::W => {
                return (-1, 0);
            }
            Direction::S => {
                return (0, 1);
            }
            Direction::NE => {
                return (1, -1);
            }
            Direction::NW => {
                return (-1, -1);
            }
            Direction::SE => {
                return (1, 1);
            }
            Direction::SW => {
                return (-1, 1);
            }
            Direction::None => {
                return (0, 0);
            }

        }
    }

    pub fn add_direction(v: &(i32, i32), dir: &Direction) -> (i32, i32) {
        let d_coords = Self::direction_to_coords(dir);
        return (v.0 + d_coords.0, v.1 + d_coords.1);
    }

    fn check_valid_coords(x: i32, y: i32, z: i32, attr: &WorldAttributes) -> bool {
        return !(x < 0 || x >= attr.size.0 as i32 || y < 0 || y >= attr.size.1 as i32);
    }

    fn generate_passive_action(turn: &mut TurnStateComponent) {
        turn.action = ActionState::MoveBy;
        turn.direction = Direction::E;
    }

    fn generate_enemy_action(turn: &mut TurnStateComponent) {
        turn.action = ActionState::MoveBy;
        turn.direction = Direction::None;
   }
}
