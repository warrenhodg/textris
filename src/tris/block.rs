use super::Colour;

pub trait Block {
    fn random(&mut self);
    fn colour(&self) -> Colour;
    fn rotate_clockwise(&mut self);
    fn rotate_anticlockwise(&mut self);
    fn dims(&self) -> (isize, isize);
    fn get(&self, x: isize, y: isize) -> bool;
    fn string(&self) -> String;
}

type UBlockValue = u16;
const UBLOCK_SPAN: isize = 4;
const BLOCK_TYPE_COUNT: usize = 7;

pub struct UBlock {
    value: UBlockValue,
    w: isize,
    h: isize,
    colour: Colour,
}

pub fn new() -> UBlock {
    UBlock {
        value: 0x0000,
        w: 0,
        h: 0,
        colour: Colour::Empty,
    }
}

#[cfg(test)]
pub const TEST_BLOCK: UBlock = UBlock {
    value: 0x0072,
    w: 3,
    h: 2,
    colour: Colour::Value(0),
};

impl UBlock {
    fn setup_block(&mut self, block_type: usize) {
        let block_type = block_type % BLOCK_TYPE_COUNT;

        match block_type {
            0 => self.setup(0x0002|0x0010|0x0020|0x0040, 3, 2, Colour::Value(block_type)), //T
            1 => self.setup(0x0002|0x0020|0x0100|0x0200, 3, 3, Colour::Value(block_type)), //LL
            2 => self.setup(0x0001|0x0010|0x0100|0x0200, 3, 3, Colour::Value(block_type)), //RL
            3 => self.setup(0x0001|0x0020|0x0010|0x0020, 2, 2, Colour::Value(block_type)), //B
            4 => self.setup(0x0001|0x0002|0x0020|0x0040, 3, 3, Colour::Value(block_type)), //LZ
            5 => self.setup(0x0002|0x0004|0x0010|0x0020, 3, 3, Colour::Value(block_type)), //RZ
            6 => self.setup(0x0002|0x0020|0x0040|0x0080, 3, 4, Colour::Value(block_type)), //I
            _ => (),
        }
    }

    fn setup(&mut self, value: UBlockValue, w: isize, h: isize, colour: Colour) {
        self.value = value;
        self.w = w;
        self.h = h;
        self.colour = colour;
    }
}

impl Block for UBlock {
    fn random(&mut self) {
        self.setup_block(rand::random::<usize>());
    }

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
        let b = new();

        assert_eq!(b.value, 0);
        assert_eq!(b.w, 0);
        assert_eq!(b.h, 0);
        assert_eq!(b.colour, Colour::Empty);
    }

    #[test]
    fn block_setup() {
        let mut b = new();

        let cases: Vec<(UBlockValue, isize, isize, Colour)> = vec![
            (0x0072, 3, 2, Colour::Value(0)),
        ];

        for case in cases {
            let (value, width, height, colour) = case;

            b.setup(value, width, height, colour);

            assert_eq!(b.value, value);
            assert_eq!(b.w, width);
            assert_eq!(b.h, height);
            assert_eq!(b.colour, colour);
        }
    }

    #[test]
    fn block_get() {
        let mut b = new();

        b.setup(0xffff, 2, 2, Colour::Value(0));
        assert_eq!(b.get(-1, -1), false);
        assert_eq!(b.get(0, 0), true);
        assert_eq!(b.get(1, 0), true);
        assert_eq!(b.get(0, 1), true);
        assert_eq!(b.get(1, 1), true);
        assert_eq!(b.get(2, 2), false);
    }

    #[test]
    fn block_rotate_clockwise() {
        let mut b = new();

        let cases: Vec<(UBlockValue, isize, isize, UBlockValue, isize, isize)> = vec![
            (0x0072, 3, 2, 0x0131, 2, 3),
        ];

        for case in cases {
            let (value, w, h, want_value, want_w, want_h) = case;

            b.setup(value, w, h, Colour::Value(0));
            b.rotate_clockwise();

            assert!(b.value == want_value, format!("received value 0x{0:04x?} instead of 0x{1:04x?}", b.value, want_value));
            assert!(b.w == want_w, format!("received w {0} instead of {1}", b.w, want_w));
            assert!(b.h == want_h, format!("received h {0} instead of {1}", b.h, want_h));
        }
    }

    #[test]
    fn block_rotate_anticlockwise() {
        let mut b = new();

        let cases: Vec<(UBlockValue, isize, isize, UBlockValue, isize, isize)> = vec![
            (0x0072, 3, 2, 0x2320, 2, 3),
        ];

        for case in cases {
            let (value, w, h, want_value, want_w, want_h) = case;

            b.setup(value, w, h, Colour::Value(0));
            b.rotate_anticlockwise();

            assert!(b.value == want_value, format!("received value 0x{0:04x?} instead of 0x{1:04x?}", b.value, want_value));
            assert!(b.w == want_w, format!("received w {0} instead of {1}", b.w, want_w));
            assert!(b.h == want_h, format!("received h {0} instead of {1}", b.h, want_h));
        }
    }
}
