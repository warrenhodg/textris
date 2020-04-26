pub mod stdin;

pub type InputKey = char;

pub trait Input {
    fn get_key(&mut self) -> Option<InputKey>;
}
