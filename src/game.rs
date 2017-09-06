extern crate tcod;
use self::tcod::*;
use self::tcod::input::*;
use std::process;

use entity::*;
use player::*;

pub struct Game {
    root : RootConsole,
    entities : Vec<Entity>,
    player : Player,
}

impl Game {
    pub fn new() -> Self {
        return Self {
            root : Game::init_game(),
            entities : Vec::new(),
            player : Player::new('@', 5, 5),
        };
    }

    fn init_game() -> RootConsole {
        return RootConsole::initializer().
            size(80, 80).
            title("Game").init();
    }

    pub fn run(&mut self) {
        self.entities.push(Entity::new('C', 2, 3));
        self.entities.push(Entity::new('F', 9, 7));
        self.entities.push(Entity::new('L', 4, 9));
        self.entities.push(Entity::new('B', 10, 3));
        loop {
            // Clear
            self.root.clear();

            // Draw state
            self.display_game_state();

            // Flush changes
            self.root.flush();

            // process input
            let c = self.root.wait_for_keypress(false);

            // player entity
            let mut p_ent = self.player.get_entity();
            if c.code == KeyCode::Char {
                match c.printable {
                    // Cardinals
                    'A' | 'a' => p_ent.offset(-1,  0),
                    'D' | 'd' => p_ent.offset( 1,  0),
                    'W' | 'w' => p_ent.offset( 0, -1),
                    'S' | 's' => p_ent.offset( 0,  1),
                    _ => {
                        println!("Invalid input");
                        continue;
                    },
                }
            }
            else {
                match c.code {
                    // Cardinals
                    KeyCode::Left   => p_ent.offset(-1,  0),
                    KeyCode::Right  => p_ent.offset( 1,  0),
                    KeyCode::Up     => p_ent.offset( 0, -1),
                    KeyCode::Down   => p_ent.offset( 0,  1),

                    KeyCode::NumPad4 => p_ent.offset(-1,  0),
                    KeyCode::NumPad6 => p_ent.offset( 1,  0),
                    KeyCode::NumPad8 => p_ent.offset( 0, -1),
                    KeyCode::NumPad2 => p_ent.offset( 0,  1),

                    KeyCode::NumPad7 => p_ent.offset(-1, -1),
                    KeyCode::NumPad9 => p_ent.offset( 1, -1),
                    KeyCode::NumPad1 => p_ent.offset(-1,  1),
                    KeyCode::NumPad3 => p_ent.offset( 1,  1),

                    KeyCode::Escape => process::exit(0),
                    _ => {
                        println!("Invalid input");
                        continue;
                    },

                }
            }

            println!("Tick!");


        }
    }

    fn display_game_state(&mut self) {
        for e in self.entities.iter() {
            println!("{:?}", e);
            self.root.put_char_ex(e.x, e.y, e.c, 
                                  colors::RED, colors::BLACK); 
        }

        let mut p_ent = self.player.get_entity();
        println!("{:?}", p_ent);
        self.root.put_char_ex(p_ent.x, p_ent.y, p_ent.c, 
                              colors::YELLOW, colors::BLACK);
    }
}
