use std::process;

extern crate tcod;
use self::tcod::*;
use self::tcod::console::Root;
use self::tcod::input::KeyCode;

extern crate specs;
use self::specs::{Component, VecStorage, System, WriteStorage, ReadStorage,
                    Fetch, FetchMut, Join};

use components::position::CharacterPositionComponent;
use components::state::{TurnStateComponent, ActionState};
use components::action::{ControllerComponent, Controllers};

use game::InputStatus;

pub struct PlayerActionGeneratorSystem;
impl<'a> System<'a> for PlayerActionGeneratorSystem {
    type SystemData = (WriteStorage<'a, TurnStateComponent>,
                       ReadStorage<'a, ControllerComponent>,
                       FetchMut<'a, RootConsole>,
                       FetchMut<'a, InputStatus>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut turn, controller, mut console, mut status) = data;

        for (turn, controller) in (&mut turn, &controller).join() {
            match controller.controller {
                Controllers::Player => {
                    if PlayerActionGeneratorSystem::generate_player_action(turn, &mut console) {
                        *status = InputStatus::Ok;
                    }
                    else {
                        *status = InputStatus::Fail;
                    }
                }
                _ => { },
            }
        }
    }

}

impl PlayerActionGeneratorSystem {
    fn generate_player_action(turn: &mut TurnStateComponent, console: &mut Root) -> bool {
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
                _ => { return false; },
            }
        }
        else {
            match key.code {
                KeyCode::NumPad1 => {
                    p = (-1, 1);
                },
                KeyCode::NumPad2 => {
                    p = (0, 1);
                },
                KeyCode::NumPad3 => {
                    p = (1, 1);
                },

                KeyCode::NumPad4 => {
                    p = (-1, 0);
                },
                KeyCode::NumPad5 => {
                    // Wait action?
                    p = (0, 0);
                },
                KeyCode::NumPad6 => {
                    p = (1, 0);
                },

                KeyCode::NumPad7 => {
                    p = (-1, -1);
                },
                KeyCode::NumPad8 => {
                    p = (0, -1);
                },
                KeyCode::NumPad9 => {
                    p = (1, -1);
                },
                _ => { return false; },
            }
        }

        turn.action = ActionState::MoveBy;
        turn.vec = p;
        return true;
    }
}
