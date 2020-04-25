mod tris;
mod input;
mod output;
mod gameloop;

extern crate termion;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> () {
    let i = &mut input::stdin::new();
    let o = &mut output::stdout::new();

    let mut g = gameloop::new(i, o);

    g.run();
}
