use super::Colour;
use super::Block;

#[cfg(test)]
use super::TEST_BLOCK;

pub trait Game {
    // Clear the game board - setting 
    fn clear(&mut self);
    fn dims(&self) -> (isize, isize);
    fn get(&self, x: isize, y: isize) -> Colour;
    fn filled(&self, x: isize, y: isize) -> bool;
    fn set(&mut self, x: isize, y: isize, colour: Colour);
    fn out_of_bounds(&self, block: &mut dyn Block, x: isize, y: isize) -> bool;
    fn collision(&self, block: &mut dyn Block, x: isize, y: isize) -> bool;
    fn merge(&mut self, block: &mut dyn Block, x: isize, y: isize);
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

            board.resize((w * h) as usize, Colour::Empty);

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
            self.board[i] = Colour::Empty;
        }
    }
    
    fn dims(&self) -> (isize, isize) {
        (self.w, self.h)
    }

    fn get(&self, x: isize, y: isize) -> Colour {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            Colour::Empty
        } else {
            let index = (y * self.w + x) as usize;
            self.board[index]
        }
    }

    fn filled(&self, x: isize, y: isize) -> bool {
        match self.get(x, y) {
            Colour::Empty => false,
            _ => true,
        }
    }

    fn set(&mut self, x: isize, y: isize, colour: Colour) {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            return
        } 
        
        let index = (y * self.w + x) as usize;
        self.board[index] = colour;
    }

    fn out_of_bounds(&self, block: &mut dyn Block, x: isize, y: isize) -> bool {
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

    fn collision(&self, block: &mut dyn Block, x: isize, y: isize) -> bool {
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

    fn merge(&mut self, block: &mut dyn Block, x: isize, y: isize) {
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

    #[test]
    fn game_get_set() {
        let mut game = VecGame::new(10, 20)
            .ok()
            .expect("game could not be created");

        let cases: Vec<(isize, isize, Colour, Vec<(isize, isize, Colour)>)> = vec![
            (5, 5, Colour::Value(0), vec![
               (-1, -1, Colour::Empty),
               (0, 0, Colour::Empty),
               (5, 5, Colour::Value(0)),
               (15, 5, Colour::Empty),
            ]),
        ];

        for case in cases {
            let (x, y, colour, tests) = case;
            game.set(x, y, colour);

            for test in tests {
                let (x, y, want_colour) = test;

                let colour = game.get(x, y);

                assert!(colour == want_colour, format!("expected {0} to equal {1}", colour, want_colour));
            }
        }
    }

    #[test]
    fn game_merge() {
        let cases: Vec<(isize, isize)> = vec![
        ];

        for case in cases {
            let (x, y) = case;

            let mut g = VecGame::new(10, 10)
                .ok()
                .expect("could not create new game");

            let b = &mut TEST_BLOCK;
            g.merge(b, x, y);
        }
    }
}
