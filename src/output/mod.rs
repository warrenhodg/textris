pub mod stdout;

pub use super::tris::Game;

pub trait Output {
    fn reset(&mut self);
    fn show_main_menu(&mut self);
    fn show_message(&mut self, message: String);
    fn show_game(&mut self, game: &mut dyn Game);
}

