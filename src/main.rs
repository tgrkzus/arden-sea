extern crate tcod;

use tcod::*;

fn main() {

    let mut root = init_game();

    let mut x = 0;
    let mut y = 0;
    loop {
        root.put_char_ex(x, y, 
                         '@', 
                         colors::YELLOW, 
                         colors::BLACK);
        root.flush();
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
