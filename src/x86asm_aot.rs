use memmap::{Mmap, MmapMut};
use std::{io::Result, process::Command, fs, time::Instant};

use crate::{runner::Runner, utils::gather_matching_brackets};

#[allow(non_camel_case_types)]
pub struct x86AsmAOT {
    // brainfuck memory
    memory: Vec<u8>,

    // brainfuck x86 code
    code: Mmap,
}

impl x86AsmAOT {
    pub fn new(bytecode: &Vec<u8>, memory_size: usize) -> Result<Self> {
        // Allocate RW region
        let mut code = MmapMut::map_anon(
            bytecode.len() * 32, /* twice the max x86 instruction size */
        )?;

        // Compile the code to the region
        Self::compile(bytecode, &mut code)?;

        // Return the compiled code ready to execute
        Ok(Self {
            memory: vec![0; memory_size],
            code: code.make_exec()?,
        })
    }

    fn compile(bytecode: &Vec<u8>, code_mmap: &mut MmapMut) -> Result<()> {

        let matching_brackets = gather_matching_brackets(bytecode)?;
        let mut program = String::new();

        let header = "[BITS 64]\nstart:\n";
        program += header;

        // program called as `func(memory_ptr)`
        // header
        program += "push rdi\n";
        program += "push rsi\n";
        program += "push rdx\n";

        // move the memory pointer to rsi
        program += "mov rsi, rdi\n";

        for (idx, opcode) in bytecode.iter().enumerate() {
            program += &format!("label_{:x}:\n", idx);
            match opcode {
                b'>' => program += "inc rsi\n",

                b'<' => program += "dec rsi\n",

                b'+' => program += "inc BYTE [rsi]\n",

                b'-' => program += "dec BYTE [rsi]\n",

                b'.' => program += r#"
xor eax, eax
inc eax
xor edi, edi
xor edx, edx
inc edx
syscall
"#,

                b',' => program += r#"
xor eax, eax
xor edi, edi
xor edx, edx
inc edx
syscall
"#,

                b'[' => program += &format!(r#"
mov dl, BYTE [rsi]
test dl, dl
je label_{:x}
"#, matching_brackets[idx] + 1),

                b']' => program += &format!(
r#"
mov dl, BYTE [rsi]
test dl, dl
jne label_{:x}
"#, matching_brackets[idx] + 1),

                _ => (),
            }
        }

        // trailer
        program += &format!("label_{:x}:\n", bytecode.len());
        program += "xor eax, eax\n";
        program += "pop rdx\n";
        program += "pop rsi\n";
        program += "pop rdi\n";
        program += "ret\n";

        let progfn =
            std::env::temp_dir().join(format!("tmp_{:?}.s", std::thread::current().id().as_u64()));
        let binfn = std::env::temp_dir().join(format!(
            "tmp_{:?}.bin",
            std::thread::current().id().as_u64()
        ));
        std::fs::write(&progfn, program).expect("Failed to write program to file");

        let _cmd = Command::new("nasm")
            .args(&[
                "-fbin",
                "-o",
                binfn.to_str().unwrap(),
                progfn.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to execute nasm")
            .exit_ok()
            .expect("nasm returned an error");

        let bin = fs::read(binfn).unwrap();
        code_mmap.fill(0xc3);
        code_mmap[..bin.len()].copy_from_slice(&bin);

        Ok(())
    }
}

impl Runner for x86AsmAOT {
    fn run(&mut self) -> Result<usize> {
        let func = &(self.code.as_ptr() as usize) as *const usize
            as *const unsafe extern "C" fn(*mut u8) -> usize;

        let start = Instant::now();

        let res = unsafe { (*func)(self.memory.as_mut_ptr()) };

        let duration = start.elapsed();

        println!("elapsed: {:?}", duration);

        Ok(res)
    }
}
