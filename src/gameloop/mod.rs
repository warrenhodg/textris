use super::input::Input;
use super::output::Output;
use super::tris::VecGame;

const GAME_LOOP_PERIOD: std::time::Duration = std::time::Duration::from_millis(10);

pub struct GameLoop<'a> {
    input: &'a mut dyn Input,
    output: &'a mut dyn Output,
}

pub fn new<'a>(input: &'a mut dyn Input, output: &'a mut dyn Output) -> GameLoop<'a> {
    GameLoop {
        input: input,
        output: output,
    }
}

impl <'a> GameLoop<'a> {
    pub fn run(&mut self, width: isize, height: isize) {
        let mut changed = true;
        let mut message = format!("");

        'run_loop: loop {
            if changed {
                self.output.show_main_menu();
                self.output.show_message(message.to_string());
                changed = false;
            }

            match self.input.get_key() {
                None => {
                    std::thread::sleep(GAME_LOOP_PERIOD);
                },
                Some(k) => match k {
                    'q' => break 'run_loop,
                    'n' => {
                        self.play_game(width, height);
                        changed = true;
                    },
                    _ => {
                        message = format!("{}Unknown command {}\r\n", termion::clear::CurrentLine, k as u32);
                        changed = true;
                    },
                }
            }
        }

        self.output.reset();
    }

    fn play_game(&mut self, width: isize, height: isize) {
        let mut changed = true;
        let g = &mut VecGame::new(width, height).unwrap();

        self.output.show_game(g);

        'play_loop: loop {
            if changed {
                self.output.show_game(g);
                changed = false;
            }

            match self.input.get_key() {
                None => {
                    std::thread::sleep(GAME_LOOP_PERIOD);
                },
                Some(k) => match k {
                    'q' => break 'play_loop,
                    _ => (),
                }
            }
        }
    }
}
