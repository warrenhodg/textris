use super::Output;

use std::io::Write;
use termion::raw::IntoRawMode;

pub fn new<'a>() -> termion::raw::RawTerminal<std::io::Stdout> {
    std::io::stdout().into_raw_mode().unwrap()
}

impl <'a> Output for termion::raw::RawTerminal<std::io::Stdout> {
    fn reset(&mut self) {
        write!(self, "{}", termion::style::Reset).unwrap();
    }

    fn show_main_menu(&mut self) {
        write!(self, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        //write!(self, "textris-{}\r\n", VERSION).unwrap();
        write!(self, "\r\n").unwrap();
        write!(self, "Menu:\r\n").unwrap();
        write!(self, "n. New game\r\n").unwrap();
        write!(self, "q. Quit\r\n").unwrap();
        write!(self, "> ").unwrap();
        self.flush().unwrap();
    }

    fn show_message(&mut self, message: String) {
        write!(self, "{}", message).unwrap();
        self.flush().unwrap();
    }
}
