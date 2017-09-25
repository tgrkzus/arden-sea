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
use components::tile::{TileComponent, TileType};

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


pub struct ActionControllerSystem;
impl<'a> System<'a> for ActionControllerSystem {
    type SystemData = (WriteStorage<'a, CharacterPositionComponent>,
                       ReadStorage<'a, TurnStateComponent>,
                       ReadStorage<'a, TileComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, turns, tiles) = data;

        let mut occupied: Vec<(TileType, i32, i32)> = Vec::new();
        for (tile, tPos) in (&tiles, &positions).join() {
            occupied.push((tile.tile_type.clone(), tPos.x, tPos.y));
        }
        println!("{:?}", occupied);

        for (position, turn) in (&mut positions, &turns).join() {
            println!("{:?}", position);
            println!("{:?}", turn);
            match turn.action {
                ActionState::None => {
                    println!("NONE action");
                }
                ActionState::MoveBy => {
                    let mut p = (turn.vec.0, turn.vec.1);

                    for tPos in &occupied {
                        match tPos.0 {
                            TileType::Impassable => {
                                if tPos.1 == position.x + p.0 && tPos.2 == position.y + p.1 {
                                    p = (0, 0);
                                }
                            },
                            _ => { },
                        }
                    }

                    position.x += p.0;
                    position.y += p.1;
                }
                ActionState::MoveTo => {
                    position.x = turn.vec.0;
                    position.y = turn.vec.1;
                }
                ActionState::Examine => {
                    println!("EXAMINE action");
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
                    println!("PASSIVE controller");
                    ActionControllerSystem::generate_passive_action(turn);
                }
                Controllers::Player => {
                    println!("PLAYER controller");
                    //ActionControllerSystem::generate_player_action(turn, &mut console);
                }
                Controllers::Enemy => {
                    println!("ENEMY controller");
                    ActionControllerSystem::generate_enemy_action(turn);
                }
            }
        }
    }

}

impl ActionControllerSystem {
    fn generate_passive_action(turn: &mut TurnStateComponent) {
        turn.action = ActionState::MoveBy;
        turn.vec = (0, 1);
    }

    fn generate_enemy_action(turn: &mut TurnStateComponent) {
        turn.action = ActionState::MoveBy;
        turn.vec = (0, 0);
   }
}
