mod game;
mod components;
mod world;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
