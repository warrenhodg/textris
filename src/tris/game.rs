use super::Colour;
use super::Block;
use super::UBlock;
use super::BlockType;

pub trait Game {
    // Clear the game board - setting 
    fn clear(&mut self);
    fn get(&self, x: isize, y: isize) -> Colour;
    fn filled(&self, x: isize, y: isize) -> bool;
    fn set(&mut self, x: isize, y: isize, colour: Colour);
    fn out_of_bounds(&self, block: &impl Block, x: isize, y: isize) -> bool;
    fn collision(&self, block: &impl Block, x: isize, y: isize) -> bool;
    fn merge(&mut self, block: &impl Block, x: isize, y: isize);
    fn string(&self) -> String;
}

pub struct VecGame {
    w: isize,
    h: isize,
    board: Vec<Colour>,
}

impl VecGame {
    pub fn new(w: isize, h: isize) -> Result<VecGame, String> {
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
        for i in 0..(self.w * self.h) as usize {
            self.board[i] = 0;
        }
    }
    
    fn get(&self, x: isize, y: isize) -> Colour {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            0
        } else {
            let index = (y * self.w + x) as usize;
            self.board[index]
        }
    }

    fn filled(&self, x: isize, y: isize) -> bool {
        self.get(x, y) != 0
    }

    fn set(&mut self, x: isize, y: isize, colour: Colour) {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            return
        } 
        
        let index = (y * self.w + x) as usize;
        self.board[index] = colour;
    }

    fn out_of_bounds(&self, block: &impl Block, x: isize, y: isize) -> bool {
        let (bw, bh) = block.dims();

        for by in 0..bh {
            for bx in 0..bw {
                if block.get(bw, by) {
                    let x = x + bx;
                    let y = y + by;
                    if x < 0 || x >= self.w || y < 0 || y >= self.h {
                        return true
                    }
                }
            }
        }
        false
    }

    fn collision(&self, block: &impl Block, x: isize, y: isize) -> bool {
        let (bw, bh) = block.dims();

        for by in 0..bh {
            for bx in 0..bw {
                if block.get(bw, by) {
                    if self.filled(x + bx, y + by) {
                        return true
                    }
                }
            }
        }

        false
    }

    fn merge(&mut self, block: &impl Block, x: isize, y: isize) {
        let (bw, bh) = block.dims();

        for by in 0..bh {
            for bx in 0..bw {
                if block.get(bx, by) {
                    self.set(x + bx, y + by, block.colour());
                }
            }
        }
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
        let cases: Vec<(isize, isize, bool)> = vec![
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

    fn game_get_set() {
        let mut game = VecGame::new(10, 20);

        let cases: Vec<(isize, isize, Vec(isize, isize, bool))> = vec![
        ];

    }

    fn game_merge() {
        let cases: Vec<(isize, isize, bool)> = vec![
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
