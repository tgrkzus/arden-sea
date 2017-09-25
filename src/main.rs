mod state;
mod game;
mod components;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
