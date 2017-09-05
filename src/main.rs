mod game;
use game::*;

fn main() {
    let mut game = Game::new();
    game.run();
    game.cleanup_game();
}
