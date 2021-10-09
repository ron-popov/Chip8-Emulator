use crate::consts;

pub struct Memory {
    memory_space: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory{memory_space: vec![0x00.into(); consts::MEMORY_SIZE]}
    }

    pub fn new_from_rom(rom_content: Vec<u8>) -> Memory {
        if rom_content.len() > (consts::MEMORY_SIZE - consts::PROGRAM_MEMORY_ADDR) {
            panic!("Rom content is too much to load to memory");
        }

        debug!("Loading a rom of length {}", rom_content.len());

        let mut mem: Memory = Memory::new();
        let mut counter: u16 = consts::PROGRAM_MEMORY_ADDR as u16;
        for byte in rom_content {
            mem.set_value(counter, byte);
            counter += 1;
        }

        debug!("Loaded rom to memory in address {} -> {}", consts::PROGRAM_MEMORY_ADDR, counter);

        return mem;
    }

    pub fn get_value(&self, index: u16) -> u8 {
        self.memory_space[index as usize]
    }

    pub fn set_value(&mut self, index: u16, value: u8) {
        self.memory_space[index as usize] = value
    }
}