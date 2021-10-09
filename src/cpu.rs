use crate::memory::Memory;
use crate::errors::Chip8Error;
use crate::consts;
use crate::stack::Stack;
use crate::display::Display;

pub struct CPU {
    memory_space: Memory,
    program_counter: u16,
    display: Display,
    stack: Stack,
    registers: [u8; 16],
    index_register: u16
}

impl CPU {
    pub fn new(memory: Memory, display: Display) -> CPU {
        CPU{memory_space: memory, program_counter: consts::PROGRAM_MEMORY_ADDR as u16, display: display, stack: Stack::new(), registers: [0x00; 16], index_register: 0x00}
    }

    pub fn execute_instruction(&mut self) -> Result<(),Chip8Error> {
        let instruction_double: u16 = ((self.memory_space.get_value(self.program_counter) as u16) << 8) + self.memory_space.get_value(self.program_counter + 1) as u16;
        debug!("Current Instruction : {:#06x}", instruction_double);
        
        // if instruction_double == 0x1228 {
        //     return Err(Chip8Error::UnknownError);
        // }
        

        // Execute simple instructions
        match instruction_double {
            0x00E0 => { //CLS - Clear screen
                //TODO
            },
            0x00EE => { // RETURN
                self.program_counter = self.stack.pop()?;
            }
            _ => { // If the instruction requires parsing the opcode nibbles
                // Parse nibbles
                let instruction_nibbles: [u8; 4] = [
                    ((instruction_double >> 12) & 0b00001111).try_into().unwrap(),
                    ((instruction_double >> 8) & 0b00001111).try_into().unwrap(),
                    ((instruction_double >> 4) & 0b00001111).try_into().unwrap(),
                    (instruction_double & 0b0001111).try_into().unwrap()
                ];

                // trace!("Instruction nibbles : {:?}", instruction_nibbles);

                // Decode and execute instruction
                match instruction_nibbles[0] {
                    1 => { //JUMP
                        let new_addr = instruction_double & 0b0000111111111111;
                        self.program_counter = new_addr;
                        return Ok(());
                    },
                    2 => { //CALL
                        let new_addr = instruction_double & 0b0000111111111111;
                        self.stack.push(self.program_counter);

                        self.program_counter = new_addr;
                    },
                    6 => { //LD - Set register value
                        let register_index = instruction_nibbles[1];
                        let new_value = (instruction_nibbles[2] << 4) + instruction_nibbles[3];

                        trace!("Setting register #{} to {:#04x}", register_index, new_value);
                        self.registers[register_index as usize] = new_value;
                    },
                    7 => { // ADD - Add to register
                        let add_value = (instruction_nibbles[2] << 4) + instruction_nibbles[3];
                        self.registers[instruction_nibbles[1] as usize] += add_value;
                    }
                    0xA => { // LD I - Set Index register
                        
                        let new_value: u16 = ((instruction_nibbles[1] as u16) << 8) + ((instruction_nibbles[2] as u16) << 4) + instruction_nibbles[3] as u16;
                        self.index_register = new_value;
                    },
                    0xD => { // DRW - Draw sprite on screen
                        let sprite_length = instruction_nibbles[3];
                        let sprite_memory_addr = self.index_register;

                        let mut sprite_content = Vec::<u8>::new();

                        for i in 0..sprite_length {
                            sprite_content.push(self.memory_space.get_value(sprite_memory_addr + i as u16));
                        }

                        let x_coord = self.registers[instruction_nibbles[1] as usize];
                        let y_coord = self.registers[instruction_nibbles[2] as usize];

                        // self.draw_screen_handler(sprite_content, x_coord, y_coord);
                        // (self.draw_screen_handler)(sprite_content, x_coord, y_coord);
                        let draw_return = self.display.draw_sprite(sprite_content, x_coord, y_coord);
                        if draw_return.is_err() {
                            let error_msg = draw_return.unwrap_err();
                            error!("Display error : {}", error_msg);
                            return Err(Chip8Error::DisplayError(error_msg));
                        }

                    }
                    _ => {
                        error!("Invalid instruction : {:#06x}", instruction_double);
                        return Err(Chip8Error::InvalidInstruction);
                    }
                }
            }
        }

        self.program_counter += 2;

        // return Err(Chip8Error::InvalidInstruction);
        return Ok(())
    }
}