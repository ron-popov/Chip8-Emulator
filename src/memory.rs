use crate::consts;
use crate::errors::Chip8Error;

pub struct Memory {
    memory_space: Vec<u8>,
    font_addresses: [u16; 16]
}

impl Memory {
    pub fn new() -> Memory {
        Memory{memory_space: vec![0x00.into(); consts::MEMORY_SIZE], font_addresses: [0; 16]}
    }

    pub fn load_font(&mut self) {
        let mut font_index: u16 = consts::FONT_START_ADDR as u16;
        for (i,digit_font_content) in consts::FONT_CONTENT.iter().enumerate() {
            self.font_addresses[i] = font_index;
            for b in digit_font_content {
                self.set_value(font_index, *b);
                font_index += 1;
            }
        }
    }

    pub fn get_font_addr(&self, digit: u8) -> Result<u16, Chip8Error> {
        if (digit as usize) > self.font_addresses.len() {
            return Err(Chip8Error::InvalidKeycode(digit));
        }

        return Ok(self.font_addresses[digit as usize]);
    }

    pub fn new_from_rom(rom_content: Vec<u8>) -> Memory {
        if rom_content.len() > (consts::MEMORY_SIZE - consts::PROGRAM_MEMORY_ADDR) {
            panic!("Rom content is too much to load to memory");
        }

        debug!("ROM_LOAD | Loading a rom of length {}", rom_content.len());

        let mut mem: Memory = Memory::new();
        let mut counter: u16 = consts::PROGRAM_MEMORY_ADDR as u16;
        for byte in rom_content {
            mem.set_value(counter, byte);
            counter += 1;
        }

        debug!("ROM_LOAD | Loaded rom to memory in address {} -> {}", consts::PROGRAM_MEMORY_ADDR, counter);

        mem.load_font();
        return mem;
    }

    pub fn get_value(&self, index: u16) -> u8 {
        self.memory_space[index as usize]
    }

    pub fn set_value(&mut self, index: u16, value: u8) {
        self.memory_space[index as usize] = value
    }
}