extern crate tcod;

use self::tcod::*;
use self::tcod::input::*;

pub struct Game {
    root : RootConsole,
    x : i32,
    y : i32,
}

impl Game {
    pub fn new() -> Self {
        return Game {
            root : Game::init_game(),
            x : 0,
            y : 0,
        };
    }

    fn init_game() -> RootConsole {
        return RootConsole::initializer().
            size(80, 80).
            title("Game").init();
    }

    pub fn cleanup_game(&self) {
    }

    pub fn run(&mut self) {
        loop {
            // Clear
            self.root.clear();

            // Draw state
            self.root.put_char_ex(self.x, self.y, 
                             '@', 
                             colors::YELLOW, 
                             colors::BLACK);
            // Flush changes
            self.root.flush();

            // process input
            let c = self.root.wait_for_keypress(false);

            if c.code == KeyCode::Char {
                match c.printable {
                    'A' | 'a' => self.x -= 1,
                    'D' | 'd' => self.x += 1,
                    'W' | 'w' => self.y -= 1,
                    'S' | 's' => self.y += 1,
                    _ => println!("Invalid input"),
                }
            }
            else {
                match c.code {
                    KeyCode::Left  => self.x -= 1,
                    KeyCode::Right => self.x += 1,
                    KeyCode::Up    => self.y -= 1,
                    KeyCode::Down  => self.y += 1,
                    _ => println!("Invalid input"),
                }
            }


        }
    }

    fn display_game_state(&self) {
    }

}
