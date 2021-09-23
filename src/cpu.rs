use crate::memory::Memory;
use crate::types::Double;
use crate::types::Byte;
use crate::consts;

pub struct CPU {
    memory_space: Memory,
    program_counter: Double,
    draw_screen_handler: fn(),
}

impl CPU {
    pub fn new(memory: Memory, draw_screen_handler: fn()) -> CPU {
        CPU{memory_space: memory, program_counter: Double::new(consts::PROGRAM_MEMORY_ADDR as u16), draw_screen_handler: draw_screen_handler}
    }
}