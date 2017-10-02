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
use components::action::{ActionControllerSystem, ControllerComponent, Controllers, Direction};
use components::information::{InformationComponent};

use gui::gui::{Gui, GuiKey};
use gui::target::{TargetGui};

use game::{InputStatus, GuiType};

pub struct PlayerActionGeneratorSystem;
impl<'a> System<'a> for PlayerActionGeneratorSystem {
    type SystemData = (WriteStorage<'a, TurnStateComponent>,
                       ReadStorage<'a, ControllerComponent>,
                       ReadStorage<'a, CharacterPositionComponent>,
                       ReadStorage<'a, InformationComponent>,
                       FetchMut<'a, RootConsole>,
                       FetchMut<'a, InputStatus>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut turns, controller, positions, infos, mut console, mut status) = data;

        // We assume our input failed and maybe prove otherwise!
        let mut new_status = InputStatus::Fail;
        //let (player, others) = (&positions, &infos, &mut turns, &controller).join().partition(|&(_, _, _, c)| c.controller == Controllers::Player);
        let (mut player, mut others) : (Vec<_>, Vec<_>) = (&controller).join().partition(|&c| c.controller == Controllers::Player);
        for (p, o) in player.iter_mut().zip(others.iter_mut()) {
            println!("{:?}, {:?}", p, o);
        }
                /*
        match player.last() {
            Some((pos, info, mut turn, controller)) => {
                let key = (*console).wait_for_keypress(false);
                Self::check_input(&key, &mut turn, &mut new_status, &status);
                *status = new_status;
            },
            _ => {
                panic!("No player!");
            }
        }
        */


        /*
           match new_status {
        // Gui actions
        InputStatus::Gui(ref action, ref mut guiType) => {
        match *guiType {
        GuiType::Target(ref mut target) => {
        let mut new = ActionControllerSystem::add_direction(&(position.x, position.y), &turn.direction);
        target.clear_list();
        for (e_pos, e_info) in (&positions, &infos).join().filter(|&(ref x, _)| (x.x, x.y) == new) {
        if new == (e_pos.x, e_pos.y) {
        target.add_to_list(e_info.name.clone());
        }
        }
        if target.list_count() <= 1 {
        // Just examine immediately
        }
        }
        }
        }
        _ => { },
        }
        }
        */
        // Set our new status
    }
}

impl PlayerActionGeneratorSystem {
    pub fn check_input(key: &tcod::input::Key, mut turn: &mut TurnStateComponent, mut new_status: &mut InputStatus, status: &mut InputStatus) {
        match *status {
            // Gui actions
            InputStatus::Gui(ref action, ref mut guiType) => {
                match *guiType {
                    GuiType::Target(ref mut target) => {
                        match target.process_input(&key) {
                            Some(result) => {
                                *new_status = result;
                                return;
                            }
                            None => {
                            }
                        }
                    }
                }
                // Clone gui to new status (TODO possible to move this?)
                *new_status = InputStatus::Gui(Box::new(InputStatus::Examine), guiType.clone());
            }

            // Examine action
            InputStatus::Examine => {
                match Self::check_directions(&key) {
                    // Process and move
                    Some(p) => {
                        turn.action = ActionState::Examine;
                        turn.direction = p;

                        // Build target GUI
                        let mut target = TargetGui::new("Pick a target".to_string());

                        *new_status = InputStatus::Gui(Box::new(InputStatus::Examine), GuiType::Target(target));
                    }
                    // Do nothing if no value
                    None => { 
                    },
                }
            },

            // Attack action
            InputStatus::Attack => {
                match Self::check_directions(&key) {
                    // Process and move
                    Some(p) => {
                        turn.action = ActionState::Attack;
                        turn.direction = p;
                        *new_status = InputStatus::Ok;
                    }
                    // Do nothing if no value
                    None => { 
                    },
                }
            },
            _ => Self::perform_normal_action(&key, &mut turn, &mut new_status),
        }
    }

    pub fn perform_normal_action(key: &tcod::input::Key, turn: &mut TurnStateComponent, new_status: &mut InputStatus) {
        // Exit check (TODO disable)
        if key.code == KeyCode::Escape {
            process::exit(0);
        }

        // Check for directions
        match Self::check_directions(key) {
            // Process and move
            Some(p) => {
                turn.action = ActionState::MoveBy;
                turn.direction = p;
                *new_status = InputStatus::Ok;
            }
            // Do nothing if no value
            None => { 
            },
        }

        match Self::check_state_keys(key) {
            // Set our returned state
            Some(new_state) => {
                *new_status = new_state;
            }
            // Do nothing if no value
            None => { 
            },
        }
    }

    /// Checks our state transition keys (i.e. multiple input required)
    /// Examples include:   Examining (which is followed by a direction)
    ///                     Opening a menu (e.g. Inventory)
    ///                     Attacking
    pub fn check_state_keys(key: &tcod::input::Key) -> Option<InputStatus> {
        let status: InputStatus;
        if key.code == KeyCode::Char {
            match key.printable {
                // Examine
                'e' => {
                    status = InputStatus::Examine;
                }

                // Attack
                'a' => {
                    status = InputStatus::Attack;
                }

                // No key found
                _ => { 
                    return None; 
                },
            }
        }
        else {
            match key.code {
                _ => {
                    return None;
                },
            }
        }

        return Some(status);
    }

    /// Checks the given key input for movement keys.
    ///
    /// Returns movement direction
    ///         None if no valid key
    pub fn check_directions(key: &tcod::input::Key) -> Option<Direction> {
        let p: Direction; 
        if key.code == KeyCode::Char {
            match key.printable {
                /*
                   'w' => p = Direction::N,
                   'a' => p = Direction::W,
                   's' => p = Direction::S,
                   'd' => p = Direction::E,
                   */
                _ => { return None; },
            }
        }
        else {
            match key.code {
                KeyCode::NumPad1 => {
                    p = Direction::SW;
                },
                KeyCode::NumPad2 => {
                    p = Direction::S;
                },
                KeyCode::NumPad3 => {
                    p = Direction::SE;
                },

                KeyCode::NumPad4 => {
                    p = Direction::W;
                },
                KeyCode::NumPad5 => {
                    // Wait action?
                    p = Direction::None;
                },
                KeyCode::NumPad6 => {
                    p = Direction::E;
                },

                KeyCode::NumPad7 => {
                    p = Direction::NW;
                },
                KeyCode::NumPad8 => {
                    p = Direction::N;
                },
                KeyCode::NumPad9 => {
                    p = Direction::NE;
                },
                _ => { return None; },
            }
        }
        return Some(p);
    }
}
