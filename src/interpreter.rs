use crate::{runner::Runner, utils::gather_matching_brackets};
use std::{
    io::{self, Error, ErrorKind, Read, Result},
    time::Instant,
};

pub struct Interpreter<'a> {
    // brainfuck source code
    bytecode: &'a Vec<u8>,

    // brainfuck instruction pointer
    ip: usize,

    // brainfuck memory
    memory: Vec<u8>,

    // brainfuck pointer
    ptr: usize,

    // map for matching [], use a vector since all the keys are idxs
    matching_brackets: Vec<usize>,

    // instructios executed
    instructions_executed: u64,
}

impl <'a> Interpreter<'a> {
    pub fn new(bytecode: &'a Vec<u8>, memory_size: usize) -> Result<Self> {
        Ok(Self {
            bytecode,
            ip: 0,
            memory: vec![0; memory_size],
            ptr: 0,
            matching_brackets: gather_matching_brackets(bytecode)?,
            instructions_executed: 0,
        })
    }

    fn run_one(&mut self) -> Result<usize> {
        // get the opcode to run
        let opcode = self.bytecode[self.ip];
        self.instructions_executed += 1;

        match opcode {
            b'>' => self.ptr = self.ptr.wrapping_add(1) % self.memory.len(),

            b'<' => {
                self.ptr = (self.ptr as isize)
                    .wrapping_sub(1)
                    .rem_euclid(self.memory.len() as isize) as usize
            }

            b'+' => self.memory[self.ptr] = self.memory[self.ptr].wrapping_add(1),

            b'-' => self.memory[self.ptr] = self.memory[self.ptr].wrapping_sub(1),

            b'.' => print!("{}", self.memory[self.ptr] as char),

            b',' => {
                let mut bytes = io::stdin().bytes();
                self.memory[self.ptr] = match bytes.next() {
                    Some(Ok(b)) => b,
                    Some(Err(e)) => return Err(e),
                    None => return Err(Error::new(ErrorKind::UnexpectedEof, "No input available")),
                };
            }

            b'[' => {
                if self.memory[self.ptr] == 0 {
                    return Ok(self.matching_brackets[self.ip] + 1);
                }
            }

            b']' => {
                if self.memory[self.ptr] != 0 {
                    return Ok(self.matching_brackets[self.ip] + 1);
                }
            }

            _ => (),
        }

        Ok(self.ip + 1)
    }

    
}

impl Runner for Interpreter<'_> {
    fn run(&mut self) -> Result<usize> {
        let start = Instant::now();

        while self.ip < self.bytecode.len() {
            self.ip = self.run_one()?;
        }

        let duration = start.elapsed();

        print!("elapsed: {:?}", duration);
        println!(
            " | M instructions/s: {:.04}",
            (self.instructions_executed as f64 / duration.as_secs_f64()) / 1_000_000f64
        );

        Ok(0)
    }
}
