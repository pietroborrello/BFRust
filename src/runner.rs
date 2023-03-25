use std::io::Result;

pub trait Runner {
    fn run(&mut self) -> Result<usize>;
}
