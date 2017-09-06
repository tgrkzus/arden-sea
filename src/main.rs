mod game;
mod entity;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
