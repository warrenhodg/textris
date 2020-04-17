use super::Colour;
use super::Game;

pub enum BlockType {
    T,
    LL,
    RL,
    B,
    LZ,
    RZ,
    I,
}

pub trait Block {
    fn colour(&self) -> Colour;
    fn rotate_clockwise(&mut self);
    fn rotate_anticlockwise(&mut self);
    fn dims(&self) -> (usize, usize);
    fn get(&self, x: usize, y: usize) -> bool;
    fn string(&self) -> String;
}

pub struct UBlock {
    value: usize,
    w: usize,
    h: usize,
    colour: Colour,
}

impl UBlock {
    pub fn new(t: BlockType) -> UBlock {
        match t {
            BlockType::T  => UBlock::setup(0x0002|0x0010|0x0020|0x0040, 3, 3, 1),
            BlockType::LL => UBlock::setup(0x0002|0x0020|0x0100|0x0200, 3, 3, 2),
            BlockType::RL => UBlock::setup(0x0001|0x0010|0x0100|0x0200, 3, 3, 3),
            BlockType::B  => UBlock::setup(0x0001|0x0020|0x0010|0x0020, 2, 2, 4),
            BlockType::LZ => UBlock::setup(0x0001|0x0002|0x0020|0x0040, 3, 3, 5),
            BlockType::RZ => UBlock::setup(0x0002|0x0004|0x0010|0x0020, 3, 3, 6),
            BlockType::I  => UBlock::setup(0x0002|0x0020|0x0040|0x0080, 3, 4, 7),
        }
    }

    fn setup(value: usize, w: usize, h: usize, colour: Colour) -> UBlock {
        UBlock {
            value: value,
            w: w,
            h: h,
            colour: colour,
        }
    }
}

impl Block for UBlock {
    fn colour(&self) -> Colour {
        self.colour
    }

    // 0:0 -> w:0
    // w:0 -> h:w
    // w:h -> 0:w
    // 0:h -> 0:0
    fn rotate_clockwise(&mut self) {
        let mut v: usize = 0;
        let w = self.w;
        let h = self.h;

        for y in 0..h {
            for x in 0..w {
                let m: usize = 1 << (y * 4 + x);
                if self.value & m == m {
                    let _x = h - y - 1;
                    let _y = x;
                    let _m: usize = 1 << (_y * 4 + _x);
                    v |= _m
                }
            }
        }

        self.value = v;
        self.w = h;
        self.h = w;
    }

    // 0:0 -> 0:w
    // w:0 -> 0:0
    // w:h -> h:0
    // 0:h -> h:w
    fn rotate_anticlockwise(&mut self) {
        let mut v: usize = 0;
        let w = self.w;
        let h = self.h;

        for y in 0..h {
            for x in 0..w {
                let m: usize = 1 << (y * 4 + x);
                if self.value & m == m {
                    let _x = y;
                    let _y = w - x;
                    let _m: usize = 1 << (_y * 4 + _x);
                    v |= _m
                }
            }
        }

        self.value = v;
        self.w = h;
        self.h = w;
    }

    fn dims(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if x >= self.w || y >= self.h {
            false
        } else {
            let m = 1 << (y * 4 + x);
            self.value & m == m
        }
    }

    fn string(&self) -> String {
        format!("0x{0:04x}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_new() {
        let b = UBlock::new(BlockType::T);
        assert_eq!(b.value, 0x0072);
        assert_eq!(b.w, 3);
        assert_eq!(b.h, 3);
    }

    #[test]
    fn block_rotate_clockwise() {
        let mut b = UBlock::new(BlockType::T);
        b.rotate_clockwise();
        assert_eq!(b.value, 0x0262);
        assert_eq!(b.w, 3);
        assert_eq!(b.h, 3);
        assert_eq!(b.dims(), (3, 3));
        assert_eq!(b.get(0, 0), false);
        assert_eq!(b.get(1, 0), true);
        assert_eq!(b.get(2, 0), false);
        assert_eq!(b.get(1000, 0), false);
    }

    #[test]
    fn block_rotate_anticlockwise() {
        let mut b = UBlock::new(BlockType::T);
        b.rotate_anticlockwise();
        assert_eq!(b.value, 0x2320);
        assert_eq!(b.w, 3);
        assert_eq!(b.h, 3);
    }
}

