extern crate play;

use std::{env, process};
use std::path::PathBuf;

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() != 2 {
        error("extected a filename");
    }
    let path = PathBuf::from(&arguments[1]);
    if let Err(message) = play::play(&path) {
        error(message);
    }
}

fn error(message: &'static str) -> ! {
    println!("Error: {}.", message);
    process::exit(1);
}
