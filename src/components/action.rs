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
                       Fetch<'a, WorldAttributes>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, turns, attr) = data;


        for (position, turn) in (&mut positions, &turns).join() {
            match turn.action {
                ActionState::None => {
                    println!("NONE action");
                }
                ActionState::MoveBy => {
                    let mut newP = (position.x + turn.vec.0, position.y + turn.vec.1);

                    /*
                    for tPos in &occupied {
                        match tPos.0 {
                            TileType::Impassable => {
                                if tPos.1 == newP.0 && tPos.2 == newP.1 {
                                    newP = (position.x, position.y);
                                }
                            },
                            _ => { },
                        }
                    }
                    */

                    if newP.0 < 0 || newP.0 >= attr.size.0 as i32 || newP.1 < 0 || newP.1 >= attr.size.1 as i32 {
                        newP = (position.x, position.y);
                    }

                    position.x = newP.0;
                    position.y = newP.1;
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
    fn generate_passive_action(turn: &mut TurnStateComponent) {
        turn.action = ActionState::MoveBy;
        turn.vec = (0, 1);
    }

    fn generate_enemy_action(turn: &mut TurnStateComponent) {
        turn.action = ActionState::MoveBy;
        turn.vec = (0, 0);
   }
}
