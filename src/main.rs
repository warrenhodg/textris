pub mod tris;

use tris::Block;
use tris::VecGame;
use tris::Game;

fn main() -> () {
    let mut b = tris::UBlock::new(tris::BlockType::T);

    let mut g = tris::VecGame::new(10, 10)
        .ok()
        .expect("invalid board size");

    g.merge(&b, 1, 2);
}
