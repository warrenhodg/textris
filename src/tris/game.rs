use super::Block;
use super::Colour;

pub struct Game {
    x: isize,
    y: isize,
    block: Block,
    w: isize,
    h: isize,
    board: Vec<Colour>,
    score: isize,
    last_fall: std::time::Instant,
    fall_rate_nanos: u128,
    game_over: bool,
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
                score: 0,
                last_fall: std::time::Instant::now(),
                fall_rate_nanos: std::time::Duration::from_millis(1000).as_nanos(),
                game_over: false,
            };

            g.new_game();

            Ok(g)
        }
    }

    pub fn new_game(&mut self) {
        for i in 0..(self.w * self.h) as usize {
            self.board[i] = Colour::Empty;
        }
        self.random();
        self.score = 0;
        self.game_over = false;
    }

    pub fn get_score(&self) -> isize {
        self.score
    }

    pub fn random(&mut self) {
        let (dx, dy) = self.block.random();
        self.x = self.w / 2 + dx;
        self.y = dy;
        self.last_fall = std::time::Instant::now();

        if self.collision(self.x, self.y) {
            self.game_over = true;
        }
    }

    pub fn tick(&mut self) -> bool {
        let now = std::time::Instant::now();
        if now.duration_since(self.last_fall).as_nanos() < self.fall_rate_nanos {
            return false;
        }

        if !self.down() {
            self.merge();
            self.random();
        }

        true
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
            return;
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
                        return true;
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
                        return true;
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
            return false;
        }

        true
    }

    pub fn rotate_anticlockwise(&mut self) -> bool {
        self.block.rotate_anticlockwise();
        if self.collision(self.x, self.y) || self.out_of_bounds(self.x, self.y) {
            self.block.rotate_clockwise();
            return false;
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

        self.last_fall = std::time::Instant::now();

        self.y += 1;
        true
    }

    pub fn drop(&mut self) {
        loop {
            if self.collision(self.x, self.y + 1) || self.out_of_bounds(self.x, self.y + 1) {
                break;
            }

            self.y += 1;
        }
    }

    pub fn merge(&mut self) -> isize {
        let (bw, bh) = self.block.dims();

        for by in 0..bh {
            for bx in 0..bw {
                if self.block.get(bx, by) {
                    self.set(self.x + bx, self.y + by, self.block.colour());
                }
            }
        }

        let count = self.remove_lines();
        if count > 0 {
            self.score += 1 << count;
        }

        count
    }

    pub fn remove_lines(&mut self) -> isize {
        let (_, bh) = self.block.dims();

        let mut count = 0 as isize;

        for by in 0..bh {
            let y = by + self.y;
            if self.is_full_line(y) {
                self.move_lines_down(y);
                count += 1;
            }
        }

        count
    }

    pub fn is_full_line(&self, y: isize) -> bool {
        for x in 0..self.w {
            if !self.filled(x, y) {
                return false;
            }
        }

        true
    }

    pub fn move_lines_down(&mut self, y: isize) {
        for x in 0..self.w {
            let mut index = x;
            let mut prev = Colour::Empty;
            for _ in 0..y {
                index += self.w;
                let this = self.board[index as usize];
                self.board[index as usize] = prev;
                prev = this;
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_new() {
        let cases: Vec<(isize, isize, bool)> = vec![(3, 3, true), (4, 4, false), (10, 10, false)];

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
        let mut game = Game::new(10, 20).ok().expect("game could not be created");

        let cases: Vec<(isize, isize, Colour, Vec<(isize, isize, Colour)>)> = vec![(
            5,
            5,
            Colour::Value(0),
            vec![
                (-1, -1, Colour::Empty),
                (0, 0, Colour::Empty),
                (5, 5, Colour::Value(0)),
                (15, 5, Colour::Empty),
            ],
        )];

        for case in cases {
            let (x, y, colour, tests) = case;
            game.set(x, y, colour);

            for test in tests {
                let (x, y, want_colour) = test;

                let colour = game.get(x, y);

                assert!(
                    colour == want_colour,
                    format!("expected {0} to equal {1}", colour, want_colour)
                );
            }
        }
    }

    #[test]
    fn game_merge() {
        let mut g = Game::new(10, 10).ok().expect("could not create new game");

        g.block.test();

        g.merge();
    }
}
