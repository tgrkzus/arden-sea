mod game;
mod entity;
mod player;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
