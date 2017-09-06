extern crate tcod;
use self::tcod::*;
use self::tcod::input::*;
use std::process;

use entity::*;
use player::*;

enum InputState {
    NORMAL,
    EXAMINE,
}

pub struct Game {
    root: RootConsole,
    entities: Vec<Entity>,
    player: Player,
    state: InputState,
}

impl Game {
    pub fn new() -> Self {
        return Self {
            root : Game::init_game(),
            entities : Vec::new(),
            player : Player::new("Player".to_string(), '@', 5, 5),
            state: InputState::NORMAL,
        };
    }

    fn init_game() -> RootConsole {
        return RootConsole::initializer().
            size(80, 80).
            title("Game").init();
    }

    pub fn run(&mut self) {
        self.entities.push(Entity::new("C".to_string(), 'C', 2, 3));
        self.entities.push(Entity::new("F".to_string(), 'F', 9, 7));
        self.entities.push(Entity::new("L".to_string(), 'L', 4, 9));
        self.entities.push(Entity::new("B".to_string(), 'B', 10, 3));
        loop {
            // Clear
            self.root.clear();

            // Draw state
            self.display_game_state();

            // Flush changes
            self.root.flush();

            // process input
            let c = self.root.wait_for_keypress(false);
            
            let mut p = (0, 0);

            // player entity
            let mut p_ent = self.player.get_entity();
            if c.code == KeyCode::Char {
                match c.printable {
                    // Cardinals
                    'A' | 'a' => p = (-1,  0),
                    'D' | 'd' => p = ( 1,  0),
                    'W' | 'w' => p = ( 0, -1),
                    'S' | 's' => p = ( 0,  1),
                    'e' => {
                        self.state = InputState::EXAMINE;
                        continue;
                    },
                    _ => {
                        println!("Invalid input");
                        continue;
                    },
                }
            }
            else {
                match c.code {
                    // Cardinals
                    KeyCode::Left   => p = (-1,  0),
                    KeyCode::Right  => p = ( 1,  0),
                    KeyCode::Up     => p = ( 0, -1),
                    KeyCode::Down   => p = ( 0,  1),

                    KeyCode::NumPad5 => p = ( 0,  0),

                    KeyCode::NumPad4 => p = (-1,  0),
                    KeyCode::NumPad6 => p = ( 1,  0),
                    KeyCode::NumPad8 => p = ( 0, -1),
                    KeyCode::NumPad2 => p = ( 0,  1),

                    KeyCode::NumPad7 => p = (-1, -1),
                    KeyCode::NumPad9 => p = ( 1, -1),
                    KeyCode::NumPad1 => p = (-1,  1),
                    KeyCode::NumPad3 => p = ( 1,  1),

                    KeyCode::Escape => process::exit(0),
                    _ => {
                        println!("Invalid input");
                        continue;
                    },

                }
            }

            match self.state {
                InputState::NORMAL => {
                    p_ent.offset(p.0, p.1);
                },
                InputState::EXAMINE => {
                    let mut ent_name = String::new();
                    let newP = (p_ent.x + p.0, p_ent.y + p.1);
                    for e in self.entities.iter() {
                        if (e.x, e.y) == newP {
                            ent_name = e.ident.clone();
                            break;
                        }
                    }
                    println!("Examine {} {}: {}", newP.0, newP.1, ent_name);
                    self.state = InputState::NORMAL;
                },
                _ => println!("what"),
            }

            println!("Tick!");


        }
    }

    fn display_game_state(&mut self) {
        for e in self.entities.iter() {
            //println!("{:?}", e);
            self.root.put_char_ex(e.x, e.y, e.c, 
                                  colors::RED, colors::BLACK); 
        }

        let mut p_ent = self.player.get_entity();
        println!("{:?}", p_ent);
        self.root.put_char_ex(p_ent.x, p_ent.y, p_ent.c, 
                              colors::YELLOW, colors::BLACK);
    }
}
