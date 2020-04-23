use super::Colour;

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
const UBLOCK_SPAN: isize = 4;

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
                let m: UBlockValue = 1 << (y * UBLOCK_SPAN + x);
                if self.value & m == m {
                    let nx = h - y - 1;
                    let ny = x;
                    let nm: UBlockValue = 1 << (ny * UBLOCK_SPAN + nx);
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
                let m: UBlockValue = 1 << (y * UBLOCK_SPAN + x);
                if self.value & m == m {
                    let nx = y;
                    let ny = w - x;
                    let nm: UBlockValue = 1 << (ny * UBLOCK_SPAN + nx);
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
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            false
        } else {
            let m = 1 << (y * UBLOCK_SPAN + x);
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
        let cases: Vec<(BlockType, UBlockValue, isize, isize)> = vec![
            (BlockType::T, 0x0072, 3, 2),
        ];

        for case in cases {
            let (block_type, want_value, want_width, want_height) = case;

            let b = UBlock::new(block_type);

            assert_eq!(b.value, want_value);
            assert_eq!(b.w, want_width);
            assert_eq!(b.h, want_height);
        }
    }

    #[test]
    fn block_get() {
        let cases: Vec<(BlockType, UBlockValue, Vec<(isize, isize, bool)>)> = vec![
            (BlockType::T, 0x0072, vec![
              (-1, -1, false),
              (0, -1, false),
              (10, -1, false),
              (-1, 0, false),
              (0, 0, false),
              (1, 0, true),
              (2, 0, false),
              (10, 0, false),
              (0, 1, true),
              (1, 1, true),
              (2, 1, true),
            ]),
        ];

        for case in cases {
            let (block_type, want_value, tests) = case;
            let b = UBlock::new(block_type);
            assert_eq!(b.value, want_value, "value");

            for test in tests {
                let (x, y, want_filled) = test;
                assert_eq!(b.get(x, y), want_filled, "get");
            }
        }
    }

    #[test]
    fn block_rotate_clockwise() {
        let cases: Vec<(BlockType, UBlockValue, isize, isize)> = vec![
            (BlockType::T, 0x0131, 2, 3),
        ];

        for case in cases {
            let (block_type, want_value, want_width, want_height) = case;

            let mut b = UBlock::new(block_type);
            b.rotate_clockwise();
            assert!(b.value == want_value, format!("received value 0x{0:04x?} instead of 0x{1:04x?}", b.value, want_value));
            assert!(b.w == want_width, format!("received w {0} instead of {1}", b.w, want_width));
            assert!(b.h == want_height, format!("received h {0} instead of {1}", b.h, want_height));
        }
    }

    #[test]
    fn block_rotate_anticlockwise() {
        let cases: Vec<(BlockType, UBlockValue, isize, isize)> = vec![
            (BlockType::T, 0x2320, 2, 3),
        ];

        for case in cases {
            let (block_type, want_value, want_width, want_height) = case;

            let mut b = UBlock::new(block_type);
            b.rotate_anticlockwise();
            assert!(b.value == want_value, format!("received value {0:4x} instead of {1:4x}", b.value, want_value));
            assert!(b.w == want_width, format!("received w {0} instead of {1}", b.w, want_width));
            assert!(b.h == want_height, format!("received h {0} instead of {1}", b.h, want_height));
        }
    }
}
