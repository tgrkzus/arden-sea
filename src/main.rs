extern crate tcod;

use tcod::*;
use tcod::input::*;

fn main() {

    let mut root = init_game();

    let mut x = 0;
    let mut y = 0;
    loop {
        // Clear
        root.clear();

        // Draw state
        root.put_char_ex(x, y, 
                         '@', 
                         colors::YELLOW, 
                         colors::BLACK);
        // Flush changes
        root.flush();
        
        // process input
        let c = root.wait_for_keypress(false);

        if c.code == KeyCode::Char {
            match c.printable {
                'A' | 'a' => x -= 1,
                'D' | 'd' => x += 1,
                'W' | 'w' => y -= 1,
                'S' | 's' => y += 1,
                _ => println!("Invalid input"),
            }
        }
        else {
            match c.code {
                KeyCode::Left  => x -= 1,
                KeyCode::Right => x += 1,
                KeyCode::Up    => y -= 1,
                KeyCode::Down  => y += 1,
                _ => println!("Invalid input"),
            }
        }


    }

    cleanup_game();
}

fn display_game_state() {
}

fn init_game() -> RootConsole {
    return RootConsole::initializer().
        size(80, 80).
        title("Game").init();
}

fn cleanup_game() {
}
