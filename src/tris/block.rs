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
    fn dims(&self) -> (isize, isize);
    fn get(&self, x: isize, y: isize) -> bool;
    fn string(&self) -> String;
}

type UBlockValue = u16;
const UBLOCKSPAN: isize = 4;

pub struct UBlock {
    value: UBlockValue,
    w: isize,
    h: isize,
    colour: Colour,
}

impl UBlock {
    pub fn new(t: BlockType) -> UBlock {
        match t {
            BlockType::T  => UBlock::setup(0x0002|0x0010|0x0020|0x0040, 3, 2, Colour::Value(0)),
            BlockType::LL => UBlock::setup(0x0002|0x0020|0x0100|0x0200, 3, 3, Colour::Value(1)),
            BlockType::RL => UBlock::setup(0x0001|0x0010|0x0100|0x0200, 3, 3, Colour::Value(2)),
            BlockType::B  => UBlock::setup(0x0001|0x0020|0x0010|0x0020, 2, 2, Colour::Value(3)),
            BlockType::LZ => UBlock::setup(0x0001|0x0002|0x0020|0x0040, 3, 3, Colour::Value(4)),
            BlockType::RZ => UBlock::setup(0x0002|0x0004|0x0010|0x0020, 3, 3, Colour::Value(5)),
            BlockType::I  => UBlock::setup(0x0002|0x0020|0x0040|0x0080, 3, 4, Colour::Value(6)),
        }
    }

    fn setup(value: UBlockValue, w: isize, h: isize, colour: Colour) -> UBlock {
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
        let mut v: UBlockValue = 0;
        let w = self.w;
        let h = self.h;

        for y in 0..h {
            for x in 0..w {
                let m: UBlockValue = 1 << (y * UBLOCKSPAN + x);
                if self.value & m == m {
                    let nx = h - y - 1;
                    let ny = x;
                    let nm: UBlockValue = 1 << (ny * UBLOCKSPAN + nx);
                    v |= nm
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
        let mut v: UBlockValue = 0;
        let w = self.w;
        let h = self.h;

        for y in 0..h {
            for x in 0..w {
                let m: UBlockValue = 1 << (y * UBLOCKSPAN + x);
                if self.value & m == m {
                    let nx = y;
                    let ny = w - x;
                    let nm: UBlockValue = 1 << (ny * UBLOCKSPAN + nx);
                    v |= nm
                }
            }
        }

        self.value = v;
        self.w = h;
        self.h = w;
    }

    fn dims(&self) -> (isize, isize) {
        (self.w, self.h)
    }

    fn get(&self, x: isize, y: isize) -> bool {
        if x >= self.w || y >= self.h {
            false
        } else {
            let m = 1 << (y * UBLOCKSPAN + x);
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

