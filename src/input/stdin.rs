use super::Input;

use termion::event::Key;
use termion::input::TermRead;

pub fn new<'a>() -> std::io::Stdin {
    std::io::stdin()
}

impl Input for std::io::Stdin {
    fn get_key(&mut self) -> Option<Result<Key, std::io::Error>> {
        self.keys().next()
    }
}

