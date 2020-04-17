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
    fn rotate_clockwise(&mut self);
    fn rotate_anticlockwise(&mut self);
    fn string(&self) -> String;
}

pub struct U16Block {
    value: u16,
    mx: u16,
    my: u16,
}

impl U16Block {
    pub fn new(t: BlockType) -> U16Block {
        match t {
            BlockType::T  => U16Block::setup(0x0002|0x0010|0x0020|0x0040, 2, 2),
            BlockType::LL => U16Block::setup(0x0002|0x0020|0x0100|0x0200, 2, 2),
            BlockType::RL => U16Block::setup(0x0001|0x0010|0x0100|0x0200, 2, 2),
            BlockType::B  => U16Block::setup(0x0001|0x0020|0x0010|0x0020, 1, 1),
            BlockType::LZ => U16Block::setup(0x0001|0x0002|0x0020|0x0040, 2, 2),
            BlockType::RZ => U16Block::setup(0x0002|0x0004|0x0010|0x0020, 2, 2),
            BlockType::I  => U16Block::setup(0x0002|0x0020|0x0040|0x0080, 2, 3),
        }
    }

    fn setup(value: u16, mx: u16, my: u16) -> U16Block {
        U16Block {
            value: value,
            mx: mx,
            my: my,
        }
    }
}

impl Block for U16Block {
    // 0:0 -> w:0
    // w:0 -> h:w
    // w:h -> 0:w
    // 0:h -> 0:0
    fn rotate_clockwise(&mut self) {
        let mut v: u16 = 0;
        let _mx = self.mx;
        let _my = self.my;

        for y in 0.._my+1 {
            for x in 0.._mx+1 {
                let m: u16 = 1 << (y * 4 + x);
                if self.value & m == m {
                    let _x = _my - y;
                    let _y = x;
                    let _m: u16 = 1 << (_y * 4 + _x);
                    v |= _m
                }
            }
        }

        self.value = v;
        self.mx = _mx;
        self.my = _my;
    }

    // 0:0 -> 0:w
    // w:0 -> 0:0
    // w:h -> h:0
    // 0:h -> h:w
    fn rotate_anticlockwise(&mut self) {
        let mut v: u16 = 0;
        let _mx = self.mx;
        let _my = self.my;

        for y in 0.._my+1 {
            for x in 0.._mx+1 {
                let m: u16 = 1 << (y * 4 + x);
                if self.value & m == m {
                    let _x = y;
                    let _y = _mx - x;
                    let _m: u16 = 1 << (_y * 4 + _x);
                    v |= _m
                }
            }
        }

        self.value = v;
        self.mx = _mx;
        self.my = _my;
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
        let b = U16Block::new(BlockType::T);
        assert_eq!(b.value, 0x0072);
        assert_eq!(b.mx, 2);
        assert_eq!(b.my, 2);
    }

    #[test]
    fn block_rotate_clockwise() {
        let mut b = U16Block::new(BlockType::T);
        b.rotate_clockwise();
        assert_eq!(b.value, 0x0262);
        assert_eq!(b.mx, 2);
        assert_eq!(b.my, 2);
    }
}

