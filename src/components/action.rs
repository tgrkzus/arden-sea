use std::process;

extern crate tcod;
use self::tcod::*;
use self::tcod::console::Root;
use self::tcod::input::KeyCode;

extern crate specs;
use self::specs::{Component, VecStorage, System, WriteStorage, ReadStorage, Fetch, FetchMut, Join,
                  EntitiesRes, Entity, Entities};

use components::position::CharacterPositionComponent;
use components::state::{TurnStateComponent, ActionState};
use components::information::InformationComponent;
use game::{WorldAttributes, LogContent};

use world::map::{TileType, Tile, Map};

#[derive(Debug)]
pub struct ControllerComponent {
    pub controller: Controllers,
}

impl Component for ControllerComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, PartialEq)]
pub enum Controllers {
    Passive,
    Player,
    Enemy,
}

#[derive(Debug, PartialEq)]
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
     ReadStorage<'a, InformationComponent>,
     Fetch<'a, WorldAttributes>,
     FetchMut<'a, Map>,
     FetchMut<'a, LogContent>,
     Entities<'a>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, turns, infos, attr, mut map, mut log, entities) = data;


        for turn in (&turns).join().filter(
            |&ref turn| turn.action == ActionState::None,
        )
        {
            println!("NONE action");
        }

        for (position, turn) in (&mut positions, &turns).join().filter(|&(_, ref turn)| {
            turn.action == ActionState::MoveBy
        })
        {
            let mut new = Self::add_direction(&(position.x, position.y), &turn.direction);

            // Check world bounds
            if !Self::check_valid_coords(new.0, new.1, 0, &attr) {
                new = (position.x, position.y);
            }

            // Check collisions
            match map.get_tile(new.0 as usize, new.1 as usize, 0)
                .unwrap()
                .tile_type {
                TileType::Wall | TileType::Air => new = (position.x, position.y),
                TileType::Ground => {}
            }


            position.x = new.0;
            position.y = new.1;
        }

        for (position, turn) in (&mut positions, &turns).join().filter(|&(_, ref turn)| {
            turn.action == ActionState::MoveTo
        })
        {
            //position.x = turn.vec.0;
            //position.y = turn.vec.1;
        }

        for (position, turn) in (&positions, &turns).join().filter(|&(_, ref turn)| {
            turn.action == ActionState::Examine
        })
        {
            let mut new = Self::add_direction(&(position.x, position.y), &turn.direction);

            let mut result: String = "There's nothing here".to_string();
            if Self::check_valid_coords(new.0, new.1, 0, &attr) {
                match map.get_tile(new.0 as usize, new.1 as usize, 0)
                    .unwrap()
                    .tile_type {
                    TileType::Wall => {
                        result = "This is a wall".to_string();
                    }
                    TileType::Air => {
                        result = "This is an empty space".to_string();
                    }
                    TileType::Ground => {
                        result = "There's some ground here".to_string();
                    }
                }
            }

            match turn.target {
                Some(id) => {
                    match (&positions, &infos, &*entities).join().find(
                        |&(_, _, ref e)| {
                            e.id() == id
                        },
                    ) {
                        Some((_, e_info, _)) => {
                            result = format!("There is {} here", e_info.name);
                        }

                        None => {
                            panic!("Can't find entity with id");
                        }
                    }
                }
                None => {}
            }

            log.add_message(result);
        }

        for (position, turn) in (&positions, &turns).join().filter(|&(_, ref turn)| {
            turn.action == ActionState::Attack
        })
        {
            let mut new = Self::add_direction(&(position.x, position.y), &turn.direction);
            log.add_message("You hit the wall. Destroying it!".to_string());
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
