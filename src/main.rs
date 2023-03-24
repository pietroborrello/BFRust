mod interpreter;
mod runner;

use std::{env, fs};

use crate::interpreter::Interpreter;
use crate::runner::Runner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: bfrust <file.bf>");
        std::process::exit(1);
    }

    let filename = &args[1];

    let filecontent = fs::read(filename).expect(&format!("Failed to read file \"{}\"", &filename));

    let mut interpreter = Interpreter::new(filecontent, 30000);
    interpreter.run().expect("Failed to run");
}
