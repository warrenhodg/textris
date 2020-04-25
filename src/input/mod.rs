pub mod stdin;

use termion::event::Key;


pub trait Input {
    fn get_key(&mut self) -> Option<Result<Key, std::io::Error>>;
}
