#![feature(thread_id_value, exit_status_error)]
mod interpreter;
mod runner;
mod x86asm_aot;
mod utils;

use std::fs;

use clap::{Parser, ValueEnum};
use x86asm_aot::x86AsmAOT;

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
            let res = interpreter.run().expect("Failed to run");
            println!("res = 0x{:x}", res);
        },
        Some(BackendType::Asm) => {
            let mut executor = x86AsmAOT::new(&filecontent, 30000).unwrap();
            let res = executor.run().expect("Failed to run");
            println!("res = 0x{:x}", res);
        },
        Some(BackendType::C) => {
            panic!("Not Implemented");
        },
    }
}
