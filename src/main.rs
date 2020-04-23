pub mod tris;

use tris::Game;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> () {
    let b = tris::UBlock::new(tris::BlockType::T);

    let mut g = tris::VecGame::new(10, 10)
        .ok()
        .expect("invalid board size");

    g.merge(&b, 1, 2);

    println!("textris-{}", VERSION);
}
