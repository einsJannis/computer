use crate::{Instruction};

pub trait Generable {
    fn generate(&self) -> &[u8];
}

impl<T> Generable for T where T: Instruction {
    fn generate(&self) -> &[u8] {
        todo!()
    }
}
