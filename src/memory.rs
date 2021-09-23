use crate::types::Byte;
use crate::types::Double;
use crate::consts;

use std::ops::Index;

pub struct Memory {
    memory_space: Vec<Byte>
}

impl Memory {
    pub fn new() -> Memory {
        Memory{memory_space: vec![0x00.into(), consts::MEMORY_SIZE.into()]}
    }

    pub fn new_from_rom(rom_content: Vec<Byte>) -> Memory {
        if rom_content.len() > (consts::MEMORY_SIZE - consts::PROGRAM_MEMORY_ADDR) {
            panic!("Rom content is too much to load to memory");
        }

        let mut mem: Memory = Memory::new();
        let counter: Double = 0.into();
        for byte in rom_content {
            mem.set_value(counter + consts::PROGRAM_MEMORY_ADDR, byte);
        }

        return mem;
    }

    pub fn get_value(self: Self, index: Double) -> Byte {
        self.memory_space[index.get_raw_value() as usize]
    }

    pub fn set_value(self: &mut Self, index: Double, value: Byte) {
        self.memory_space[index.get_raw_value() as usize] = value
    }
}