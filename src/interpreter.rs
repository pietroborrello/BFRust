use crate::runner::Runner;
use std::{
    collections::BTreeMap,
    io::{self, Error, ErrorKind, Read, Result},
};

pub struct Interpreter {
    // brainfuck source code
    bytecode: Vec<u8>,

    // brainfuck instruction pointer
    ip: usize,

    // brainfuck memory
    memory: Vec<u8>,

    // brainfuck pointer
    ptr: usize,

    // map for matching []
    matching_brackets: BTreeMap<usize, usize>,
}

impl Interpreter {
    pub fn new(bytecode: Vec<u8>, memory_size: usize) -> Self {
        Self {
            bytecode,
            ip: 0,
            memory: vec![0; memory_size],
            ptr: 0,
            matching_brackets: BTreeMap::new(),
        }
    }

    fn run_one(&mut self) -> Result<usize> {
        // get the opcode to run
        let opcode = self.bytecode[self.ip];

        match opcode {
            b'>' => self.ptr = self.ptr.wrapping_add(1) % self.memory.len(),

            b'<' => self.ptr = (self.ptr as isize).wrapping_sub(1).rem_euclid(self.memory.len() as isize) as usize,

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

            b'[' => if self.memory[self.ptr] == 0 { 
                return Ok(*self.matching_brackets.get(&self.ip).unwrap() + 1);
            },

            b']' => if self.memory[self.ptr] != 0 { 
                return Ok(*self.matching_brackets.get(&self.ip).unwrap() + 1);
            }

            _ => ()
        }

        Ok(self.ip + 1)
    }

    fn gather_matching_brackets(&mut self) -> Result<()> {
        // keep track of where we met the [ brackets to match them
        let mut brackets_queue: Vec<usize> = Vec::with_capacity(self.bytecode.len() / 2);

        for (idx, opcode) in self.bytecode.iter().enumerate() {
            match opcode {
                b'[' => brackets_queue.push(idx),

                b']' => {
                    let last_bracket = brackets_queue.pop().ok_or_else(|| {
                        Error::new(ErrorKind::InvalidData, "Mismatched bracket ]")
                    })?;

                    // keep track of both directions
                    self.matching_brackets.insert(last_bracket, idx);
                    self.matching_brackets.insert(idx, last_bracket);
                }

                _ => (),
            }
        }
        Ok(())
    }
}

impl Runner for Interpreter {
    fn run(&mut self) -> Result<()> {
        self.gather_matching_brackets()?;

        while self.ip < self.bytecode.len() {
            self.ip = self.run_one()?;
        }
        Ok(())
    }
}
