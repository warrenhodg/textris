pub mod stdout;

pub trait Output {
    fn reset(&mut self);
    fn show_main_menu(&mut self);
    fn show_message(&mut self, message: String);
}

