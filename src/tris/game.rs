use super::Colour;
use super::Block;
use super::UBlock;
use super::BlockType;

pub trait Game {
    fn clear(&mut self);
    fn set(&mut self, x: usize, y: usize, colour: Colour) -> Result<(), String>;
    fn merge(&mut self, block: &impl Block, x: usize, y: usize) -> Result<(), String>;
    fn string(&self) -> String;
}

pub struct VecGame {
    w: usize,
    h: usize,
    board: Vec<Colour>,
}

impl VecGame {
    pub fn new(w: usize, h: usize) -> Result<VecGame, String> {
        if w < 4 || h < 4 {
            Err("too small".to_string())
        } else {
            let mut board: Vec<Colour> = vec![];

            board.resize((w * h) as usize, 0);

            Ok(VecGame {
                w: w,
                h: h,
                board: board,
            })
        }
    }
}

impl Game for VecGame {
    fn clear(&mut self) {
        for i in 0..self.w * self.h {
            self.board[i] = 0;
        }
    }
    
    fn set(&mut self, x: usize, y: usize, colour: Colour) -> Result<(), String>{
        if x >= self.w || y >= self.h {
            Err("invalid position".to_string())
        } else {
            let index = (y * self.w + x) as usize;
            self.board[index] = colour;
            Ok(())
        }
    }

    fn merge(&mut self, block: &impl Block, x: usize, y: usize) -> Result<(), String> {
        let (w, h) = block.dims();

        for _y in 0..h {
            for _x in 0..w {
                if block.get(_x, _y) {
                    let x = _x + x;
                    let y = _y + y;
                    if x < 0 || x >= self.w || y < 0 || y >= self.h {
                        return Err("invalid location".to_string());
                    }
                    self.set(_x, _y, block.colour());
                }
            }
        }

        Ok(())
    }

    fn string(&self) -> String {
        format!("{0}x{1}", self.w, self.h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_new() {
        let cases: Vec<(usize, usize, bool)> = vec![
            (3, 3, true),
            (4, 4, false),
            (10, 10, false),
        ];

        for case in cases {
            let (w, h, err) = case;
            let g = VecGame::new(w, h);
            match g {
                Err(_) => assert!(err, "should have returned an error"),
                Ok(_) => assert!(!err, "should not have returned an error"),
            }
        }
    }

    fn game_merge() {
        let cases: Vec<(usize, usize, bool)> = vec![
        ];

        for case in cases {
            let (x, y, err) = case;

            let mut g = VecGame::new(10, 10)
                .ok()
                .expect("could not create new game");

            let b = UBlock::new(BlockType::T);
            let result = g.merge(&b, x, y);
            match result {
                Err(_) => assert!(err, "should have returned an error"),
                Ok(_) => assert!(!err, "should not have returned an error"),
            }
        }
    }
}
