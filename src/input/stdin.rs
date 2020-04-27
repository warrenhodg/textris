use super::Input;
use super::InputKey;

use std::io::Read;
use termion::AsyncReader;

pub fn new<'a>() -> AsyncReader {
    termion::async_stdin()
}

impl Input for AsyncReader {
    fn get_key(&mut self) -> Option<InputKey> {
        let mut buf: [u8; 1] = [0];
        let res = self.read(&mut buf);
        match res {
            Ok(n) => {
                if n == 0 {
                    Option::<InputKey>::None
                } else {
                    Option::<InputKey>::Some(buf[0] as char)
                }
            }
            _ => Option::<InputKey>::None,
        }
    }
}
