mod game;
mod entity;
mod component;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
