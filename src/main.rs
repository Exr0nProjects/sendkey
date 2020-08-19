extern crate autopilot;

use autopilot::key::{type_string, toggle, Character};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Cli {
    // down?
    Down { code: u32 },
    Up { code: u32 }
    //#[structopt(long, parse(try_from_str = parse_bool))]
    //down: bool,
    //code: u32
}

fn main() {
    let args = Cli::from_args();
    match args {
        Cli::Up{code} => toggle(&Character('a'), false, &[], 0),
        Cli::Down{code} => toggle(&Character('a'), true, &[], 0)
    }
}
