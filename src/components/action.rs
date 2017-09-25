use std::process;

extern crate tcod;
use self::tcod::*;
use self::tcod::console::Root;
use self::tcod::input::KeyCode;

extern crate specs;
use self::specs::{Component, VecStorage, System, WriteStorage, ReadStorage, Fetch, FetchMut, Join};

use components::position::CharacterPositionComponent;
use components::state::{TurnStateComponent, ActionState};

#[derive(Debug)]
pub struct ControllerComponent {
    pub controller: Controllers,
}

impl Component for ControllerComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub enum Controllers {
    PASSIVE,
    PLAYER,
    ENEMY,
}


pub struct ActionControllerSystem;
impl<'a> System<'a> for ActionControllerSystem {
    type SystemData = (WriteStorage<'a, CharacterPositionComponent>,
     ReadStorage<'a, TurnStateComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut position, turn) = data;

        for (position, turn) in (&mut position, &turn).join() {
            println!("{:?}", position);
            println!("{:?}", turn);
            match turn.action {
                ActionState::NONE => {
                    println!("NONE action");
                }
                ActionState::MOVE_BY => {
                    println!("MOVE action");
                    position.x += turn.vec.0;
                    position.y += turn.vec.1;
                }
                ActionState::MOVE_TO => {
                    println!("MOVE action");
                    position.x = turn.vec.0;
                    position.y = turn.vec.1;
                }
                ActionState::EXAMINE => {
                    println!("EXAMINE action");
                }
                ActionState::ATTACK => {
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
        let (mut turn, controller, mut console) = data;

        for (turn, controller) in (&mut turn, &controller).join() {
            match controller.controller {
                Controllers::PASSIVE => {
                    println!("PASSIVE controller");
                    ActionControllerSystem::generate_passive_action(turn);
                }
                Controllers::PLAYER => {
                    println!("PLAYER controller");
                    ActionControllerSystem::generate_player_action(turn, &mut console);
                }
                Controllers::ENEMY => {
                    println!("ENEMY controller");
                    ActionControllerSystem::generate_enemy_action(turn);
                }
            }
        }
    }

}

impl ActionControllerSystem {
    fn generate_passive_action(turn: &mut TurnStateComponent) {
        turn.action = ActionState::MOVE_BY;
        turn.vec = (0, 1);
    }

    fn generate_player_action(turn: &mut TurnStateComponent, console: &mut Root) {
        let key = (*console).wait_for_keypress(false);

        if key.code == KeyCode::Escape {
            process::exit(0);
        }

        let mut p: (i32, i32) = (0, 0);
        if key.code == KeyCode::Char {
            match key.printable {
                'w' => p = (0, -1),
                'a' => p = (-1, 0),
                's' => p = (0, 1),
                'd' => p = (1, 0),
                _ => println!("Invalid key"),
            }
        }
        else {
            // Stuff
        }

        turn.action = ActionState::MOVE_BY;
        turn.vec = p;
    }

    fn generate_enemy_action(turn: &mut TurnStateComponent) {
        turn.action = ActionState::MOVE_BY;
        turn.vec = (1, 1);
   }
}
