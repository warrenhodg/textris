pub mod tris;

use tris::Block;

fn main() -> () {
    let mut b = tris::U16Block::new(tris::BlockType::T);

    for _i in 0..9 {
        println!("{0}", b.string());
        b.rotate_clockwise();
    }
}
