extern crate autopilot;

use autopilot::key::{KeyCodeConvertible, toggle, Character};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Cli {
    Down { code: char },
    Up { code: char }
}

fn main() {
    let args = Cli::from_args();
    println!("args: {:?}", args);
    match args {
        Cli::Up{code} => toggle(&Character(code), false, &[], 0),
        Cli::Down{code} => toggle(&Character(code), true, &[], 0)
    }
}
