use crate::types::Byte;
use crate::types::Double;
use crate::consts;

pub struct Memory {
    memory_space: Vec<Byte>
}

impl Memory {
    pub fn new() -> Memory {
        Memory{memory_space: vec![0x00.into(); consts::MEMORY_SIZE]}
    }

    pub fn new_from_rom(rom_content: Vec<Byte>) -> Memory {
        if rom_content.len() > (consts::MEMORY_SIZE - consts::PROGRAM_MEMORY_ADDR) {
            panic!("Rom content is too much to load to memory");
        }

        debug!("Loading a rom of length {}", Double::new_usize(rom_content.len()));

        let mut mem: Memory = Memory::new();
        let mut counter: Double = Double::new_usize(consts::PROGRAM_MEMORY_ADDR);
        for byte in rom_content {
            mem.set_value(counter, byte);
            counter += 1;
        }

        debug!("Loaded rom to memory in address {} -> {}", Double::new_usize(consts::PROGRAM_MEMORY_ADDR), counter);

        return mem;
    }

    pub fn get_value(self: Self, index: Double) -> Byte {
        self.memory_space[index.get_raw_value() as usize]
    }

    pub fn set_value(self: &mut Self, index: Double, value: Byte) {
        self.memory_space[index.get_raw_value() as usize] = value
    }
}