extern crate tcod;
use self::tcod::*;
use self::tcod::input::*;
use std::process;

use entity::*;

pub struct Game {
    root : RootConsole,
    entities : Vec<Entity>,
    player : Entity,
}

impl Game {
    pub fn new() -> Self {
        return Self {
            root : Game::init_game(),
            entities : Vec::new(),
            player : Entity::new('@', 5, 5),
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

            if c.code == KeyCode::Char {
                match c.printable {
                    'A' | 'a' => self.player.x -= 1,
                    'D' | 'd' => self.player.x += 1,
                    'W' | 'w' => self.player.y -= 1,
                    'S' | 's' => self.player.y += 1,
                    _ => println!("Invalid input"),
                }
            }
            else {
                match c.code {
                    KeyCode::Left   => self.player.x -= 1,
                    KeyCode::Right  => self.player.x += 1,
                    KeyCode::Up     => self.player.y -= 1,
                    KeyCode::Down   => self.player.y += 1,
                    KeyCode::Escape => process::exit(0),
                    _ => println!("Invalid input"),
                }
            }


        }
    }

    fn display_game_state(&mut self) {
        for e in self.entities.iter() {
            self.root.put_char_ex(e.x, e.y, e.c, 
                                  colors::RED, colors::BLACK); 
        }

        self.root.put_char_ex(self.player.x, self.player.y, self.player.c, 
                              colors::YELLOW, colors::BLACK);
    }
}
