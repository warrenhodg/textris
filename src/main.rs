mod tris;
mod input;
mod output;
mod gameloop;

extern crate structopt;
extern crate termion;

use structopt::StructOpt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, StructOpt)]
#[structopt(name = "textris", about = "A terminal-based tetris clone", version = VERSION)]
struct Opt {
    /// The width of the board
    #[structopt(short, long, default_value = "10")]
    width: isize,

    /// The height of the board
    #[structopt(short, long, default_value = "20")]
    height: isize,
}

fn main() -> () {
    let opt = Opt::from_args();

    println!("{:?}", opt);

    let i = &mut input::stdin::new();
    let o = &mut output::stdout::new();

    let mut g = gameloop::new(i, o);

    g.run(opt.width, opt.height);
}
