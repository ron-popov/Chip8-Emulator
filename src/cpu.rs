use crate::memory::Memory;
use crate::types::Double;
use crate::types::Byte;
use crate::errors::Chip8Error;
use crate::consts;

pub struct CPU {
    memory_space: Memory,
    program_counter: Double,
    draw_screen_handler: fn(&Memory),
}

impl CPU {
    pub fn new(memory: Memory, draw_screen_handler: fn(&Memory)) -> CPU {
        CPU{memory_space: memory, program_counter: Double::new_usize(consts::PROGRAM_MEMORY_ADDR), draw_screen_handler: draw_screen_handler}
    }

    pub fn execute_instruction(&mut self) -> Result<(),Chip8Error> {
        let lower_byte = self.memory_space.get_value(self.program_counter);
        let upper_byte = self.memory_space.get_value(self.program_counter + 1);

        self.program_counter += 2;

        return Err(Chip8Error::UnknownError)
    }
}