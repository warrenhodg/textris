use super::input::Input;
use super::output::Output;
use termion::event::Key;

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
    pub fn run(&mut self) {
        'run_loop: loop {
            self.output.show_main_menu();

            match self.input.get_key() {
                None => (),
                Some(k) => {
                    match k.unwrap() {
                        Key::Char('q') => break 'run_loop,
                        Key::Char('n') => self.play_game(),
                        _ => self.message(format!("{}Unknown command\r\n", termion::clear::CurrentLine).to_string()),
                    }
                }
            }
        }

        self.output.reset();
    }

    fn play_game(&mut self) {
        self.output.show_message("playing".to_string());
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }

    fn message(&mut self, message: String) {
        self.output.show_message(message);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
