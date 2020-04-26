use super::Colour;
use super::Block;

pub struct Game {
    x: isize,
    y: isize,
    block: Block,
    w: isize,
    h: isize,
    board: Vec<Colour>,
}

impl Game {
    pub fn new(w: isize, h: isize) -> Result<Self, String> {
        if w < 4 || h < 4 {
            Err("too small".to_string())
        } else {
            let mut board: Vec<Colour> = vec![];

            board.resize((w * h) as usize, Colour::Empty);

            let mut g = Self {
                x: 0,
                y: 0,
                block: super::Block::new(),
                w: w,
                h: h,
                board: board,
            };

            g.clear();
            g.random();

            Ok(g)
        }
    }

    pub fn clear(&mut self) {
        for i in 0..(self.w * self.h) as usize {
            self.board[i] = Colour::Empty;
        }
    }

    pub fn random(&mut self) {
        let (dx, dy) = self.block.random();
        self.x = self.w / 2 + dx;
        self.y = dy;
    }

    pub fn dims(&self) -> (isize, isize) {
        (self.w, self.h)
    }

    pub fn get(&self, x: isize, y: isize) -> Colour {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            Colour::Empty
        } else {
            let index = (y * self.w + x) as usize;
            self.board[index]
        }
    }

    pub fn display_get(&self, x: isize, y: isize) -> Colour {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            Colour::Empty
        } else {
            if self.block.get(x - self.x, y - self.y) {
                self.block.colour()
            } else {
                let index = (y * self.w + x) as usize;
                self.board[index]
            }
        }
    }

    pub fn filled(&self, x: isize, y: isize) -> bool {
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

    fn out_of_bounds(&self, x: isize, y: isize) -> bool {
        let (bw, bh) = self.block.dims();

        for by in 0..bh {
            for bx in 0..bw {
                if self.block.get(bx, by) {
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

    fn collision(&self, x: isize, y: isize) -> bool {
        let (bw, bh) = self.block.dims();

        for by in 0..bh {
            for bx in 0..bw {
                if self.block.get(bx, by) {
                    if self.filled(x + bx, y + by) {
                        return true
                    }
                }
            }
        }

        false
    }

    pub fn rotate_clockwise(&mut self) -> bool {
        self.block.rotate_clockwise();
        if self.collision(self.x, self.y) || self.out_of_bounds(self.x, self.y) {
            self.block.rotate_anticlockwise();
            return false
        }

        true
    }

    pub fn rotate_anticlockwise(&mut self) -> bool {
        self.block.rotate_anticlockwise();
        if self.collision(self.x, self.y) || self.out_of_bounds(self.x, self.y) {
            self.block.rotate_clockwise();
            return false
        }

        true
    }

    pub fn slide(&mut self, dx: isize) -> bool {
        if self.collision(self.x + dx, self.y) || self.out_of_bounds(self.x + dx, self.y) {
            return false;
        }

        self.x += dx;
        true
    }

    pub fn down(&mut self) -> bool {
        if self.collision(self.x, self.y + 1) || self.out_of_bounds(self.x, self.y + 1) {
            return false;
        }

        self.y += 1;
        true
    }

    pub fn drop (&mut self) {
        loop {
            if self.collision(self.x, self.y + 1) || self.out_of_bounds(self.x, self.y + 1) {
                break;
            }

            self.y += 1;
        }
    }

    pub fn merge(&mut self) {
        let (bw, bh) = self.block.dims();

        for by in 0..bh {
            for bx in 0..bw {
                if self.block.get(bx, by) {
                    self.set(self.x + bx, self.y + by, self.block.colour());
                }
            }
        }
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
            let g = Game::new(w, h);
            match g {
                Err(_) => assert!(err, "should have returned an error"),
                Ok(_) => assert!(!err, "should not have returned an error"),
            }
        }
    }

    #[test]
    fn game_get_set() {
        let mut game = Game::new(10, 20)
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

            let mut g = Game::new(10, 10)
                .ok()
                .expect("could not create new game");

            g.block.test();

            g.merge();
        }
    }
}
