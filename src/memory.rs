use crate::types::Byte;
use crate::types::Double;
use crate::consts;

pub struct Memory {
    memory_space: Vec<Byte>
}

pub fn init_empty_memory() -> Memory {
    Memory{memory_space: vec![0x00.into(), consts::MEMORY_SIZE.into()]}
}

impl Memory {
    pub fn get_value(self: Self, index: Double) -> Byte {
        self.memory_space[index.get_raw_value() as usize]
    }
}

// TODO : Implement indexing