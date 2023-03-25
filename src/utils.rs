use std::io::{Result, Error, ErrorKind};

pub fn gather_matching_brackets(bytecode: &Vec<u8>) -> Result<Vec<usize>> {
    // keep track of where we met the [ brackets to match them
    let mut brackets_queue: Vec<usize> = Vec::with_capacity(bytecode.len() / 2);

    let mut matching_brackets = vec![0usize; bytecode.len()];

    for (idx, opcode) in bytecode.iter().enumerate() {
        match opcode {
            b'[' => brackets_queue.push(idx),

            b']' => {
                let last_bracket = brackets_queue.pop().ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "Mismatched bracket ]")
                })?;

                // keep track of both directions
                matching_brackets[last_bracket] = idx;
                matching_brackets[idx] = last_bracket;
            }

            _ => (),
        }
    }
    Ok(matching_brackets)
}