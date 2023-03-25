mod interpreter;
mod runner;
mod utils;

use std::fs;

use clap::{Parser, ValueEnum};

use crate::interpreter::Interpreter;
use crate::runner::Runner;

#[derive(ValueEnum, Clone, Debug)]
enum BackendType {
    C,
    Asm,
    Emu
}

#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    #[arg(value_enum, short, long)]
    backend: Option<BackendType>,
    filename: String,
}

fn main() {
    let cli = Cli::parse();

    let filename = &cli.filename;

    let filecontent = fs::read(filename).expect(&format!("Failed to read file \"{}\"", &filename));

    match cli.backend {
        None | Some(BackendType::Emu) => {
            let mut interpreter = Interpreter::new(&filecontent, 30000).unwrap();
            interpreter.run().expect("Failed to run");
        },
        Some(BackendType::Asm) => {
            panic!("Not Implemented");
        },
        Some(BackendType::C) => {
            panic!("Not Implemented");
        },
    }
}
