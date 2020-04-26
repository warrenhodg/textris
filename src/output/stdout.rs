use super::Output;
use super::super::tris::Colour;
use super::Game;

use std::io::Write;
use termion::raw::IntoRawMode;
use termion::color;


const BLOCK: &'static str = "\u{2588}";

pub fn new<'a>() -> termion::raw::RawTerminal<std::io::Stdout> {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    stdout
}

fn block_style(i: u16) -> String {
    match i {
        0 => color::Fg(color::Rgb(196, 0, 0)),
        1 => color::Fg(color::Rgb(196, 196, 0)),
        2 => color::Fg(color::Rgb(0, 196, 0)),
        3 => color::Fg(color::Rgb(0, 196, 196)),
        4 => color::Fg(color::Rgb(0, 0, 196)),
        5 => color::Fg(color::Rgb(196, 0, 196)),
        _ => color::Fg(color::Rgb(196, 196, 196)),
    }.to_string()
}

impl <'a> Output for termion::raw::RawTerminal<std::io::Stdout> {
    fn reset(&mut self) {
        write!(self, "{}", termion::style::Reset).unwrap();
    }

    fn show_main_menu(&mut self) {
        write!(self, "{}{}{}",
               termion::clear::All,
               termion::color::Fg(termion::color::Rgb(196, 196, 196)),
               termion::cursor::Goto(1, 1),
               ).unwrap();
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

    fn show_game(&mut self, game: &mut dyn Game) {
        let (width, height) = game.dims();

        write!(self, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        write!(self, "{}T{}E{}X{}T{}R{}I{}S{}!\r\n\r\n",
               color::Fg(color::Rgb(255, 0, 0)),
               color::Fg(color::Rgb(255, 255, 0)),
               color::Fg(color::Rgb(0, 255, 0)),
               color::Fg(color::Rgb(0, 255, 255)),
               color::Fg(color::Rgb(0, 0, 255)),
               color::Fg(color::Rgb(255, 0, 255)),
               color::Fg(color::Rgb(255, 0, 0)),
               color::Fg(color::Rgb(255, 255, 0)),
               ).unwrap();

        for y in 0..height {
            // Display left wall
            write!(self, "{}{}",
                   color::Fg(color::Rgb(96, 96, 96)),
                   BLOCK).unwrap();

            // Display contents
            for x in 0..width {
                match game.get(x, y) {
                    Colour::Empty => write!(self, " ").unwrap(),
                    Colour::Value(i) => write!(self, "{}{}",
                        block_style(i),
                        BLOCK).unwrap(),
                }
            }

            // Display right wall
            write!(self, "{}{}\r\n",
                   color::Fg(color::Rgb(96, 96, 96)),
                   BLOCK).unwrap();
        }

        //Display bottom wall
        write!(self, "{}", color::Fg(color::Rgb(96, 96, 96))).unwrap();
        for _ in 0..width+2 {
            write!(self, "{}", BLOCK).unwrap();
        }


        self.flush().unwrap();
    }
}
