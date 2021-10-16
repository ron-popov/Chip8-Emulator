use crate::memory::Memory;
use crate::errors::Chip8Error;
use crate::consts;
use crate::stack::Stack;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Scancode;

use rand::Rng;
use rand::rngs::ThreadRng;

use std::collections::HashMap;

pub struct CPU {
    memory_space: Memory,
    program_counter: u16,
    canvas: Canvas<Window>,
    stack: Stack,
    registers: [u8; 16],
    index_register: u16,
    rng: ThreadRng,
    layout_to_scancode_map: HashMap::<u8, u16>
}

impl CPU {
    pub fn new(memory: Memory, canvas: Canvas<Window>) -> CPU {
        let rng = rand::thread_rng();
        CPU{memory_space: memory, program_counter: consts::PROGRAM_MEMORY_ADDR as u16, canvas: canvas, stack: Stack::new(), 
            registers: [0x00; 16], index_register: 0x00, rng: rng, layout_to_scancode_map : consts::get_layout_to_scancode_map()}
    }

    pub fn draw_sprite(&mut self, sprite_content: Vec<u8>, x_coord: u8, y_coord: u8) -> Result<(), String> {
        debug!("Displaying sprite");
        trace!("Sprite content : {:?}", sprite_content);
    
        let mut y = (y_coord as i32) % consts::DISPLAY_HEIGHT as i32;
        for sprite in sprite_content {
            let mut value = sprite;
            
            
            for i in 0..8 {
                let x = (x_coord as i32 + (7 - i)) % consts::DISPLAY_WIDTH as i32;
    
                if (value & 0b1) == 1 {
                    // self.canvas.set_draw_color(Color::BLACK);
                    self.canvas.set_draw_color(Color::WHITE);
                    trace!("Drawing black pixel at ({}, {})", x, y);
                } else {
                    self.canvas.set_draw_color(Color::BLACK);
                    trace!("Drawing white pixel at ({}, {})", x, y);
                }
    
                value = value >> 1;
    
                self.canvas.fill_rect(Rect::new(x, y, 1, 1))?;
                
            }
            y += 1;
        }
    
        self.canvas.present();
    
        return Ok(());
    }

    pub fn execute_instruction(&mut self, keys: Vec::<Scancode>) -> Result<(),Chip8Error> {
        let instruction_double: u16 = ((self.memory_space.get_value(self.program_counter) as u16) << 8) + self.memory_space.get_value(self.program_counter + 1) as u16;
        debug!("Current Instruction : {:#06x}", instruction_double);
        
        // Execute simple instructions
        match instruction_double {
            0x00E0 => { //CLS - Clear screen
                //TODO
            },
            0x00EE => { // RETURN
                self.program_counter = self.stack.pop()?;
            },
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
                    0 => {}, //SYS Addr, ignored by modern interpreters
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
                    3 => { //SE - Skip if equal
                        let register_index = instruction_nibbles[1];
                        let comp_value = (instruction_nibbles[2] << 4) + instruction_nibbles[3];

                        if self.registers[register_index as usize] == comp_value {
                            self.program_counter += 2;
                        }
                    },
                    4 => { //SNE - Skip if not equal
                        let register_index = instruction_nibbles[1];
                        let comp_value = (instruction_nibbles[2] << 4) + instruction_nibbles[3];

                        if self.registers[register_index as usize] != comp_value {
                            self.program_counter += 2;
                        }
                    },
                    5 => { //SE - Skip if registers equals
                        if instruction_nibbles[3] != 0 {
                            error!("Invalid instruction : {:#06x}", instruction_double);
                            return Err(Chip8Error::InvalidInstruction);
                        }

                        let first_register_index = instruction_nibbles[1];
                        let second_register_index = instruction_nibbles[2];
                        
                        if self.registers[first_register_index as usize] == self.registers[second_register_index as usize] {
                            self.program_counter += 2;
                        }
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
                    },
                    8 =>{ //LD - Registers
                        let x_register = instruction_nibbles[1] as usize;
                        let y_register = instruction_nibbles[2] as usize;

                        match instruction_nibbles[3] {
                            0 => { //Assign
                                self.registers[x_register] = self.registers[y_register];
                            },
                            1 => { //Bitwise or
                                self.registers[x_register] = self.registers[x_register] | self.registers[y_register];
                            },
                            2 => { //Bitwise and
                                self.registers[x_register] = self.registers[x_register] & self.registers[y_register];
                            },
                            3 => { //Bitwise xor
                                self.registers[x_register] = self.registers[x_register] ^ self.registers[y_register];
                            },
                            4 => { //Add
                                let add_result: u16 = self.registers[x_register] as u16 + self.registers[y_register] as u16;
                                if add_result > u8::MAX as u16 {
                                    self.registers[0x0F] = 1;
                                } else {
                                    self.registers[0x0F] = 0;
                                }

                                self.registers[x_register] = add_result as u8;
                            },
                            5 => { //Sub
                                if self.registers[x_register] > self.registers[y_register] {
                                    self.registers[0x0F] = 1;
                                } else {
                                    self.registers[0x0F] = 0;
                                }

                                self.registers[x_register] = self.registers[x_register] - self.registers[y_register];
                            },
                            6 => { //Shift Right
                                self.registers[0x0F] = self.registers[x_register] & 0b00000001;
                                self.registers[x_register] = self.registers[x_register] >> 1;
                            },
                            7 => { //SubN
                                if self.registers[y_register] > self.registers[x_register] {
                                    self.registers[0x0F] = 1;
                                } else {
                                    self.registers[0x0F] = 0;
                                }

                                self.registers[x_register] = self.registers[y_register] - self.registers[x_register];
                            },
                            0xE => { //Shift Left
                                self.registers[0x0F] = self.registers[x_register] & 0b10000000;
                                self.registers[x_register] = self.registers[x_register] << 1;
                            },
                            _ => {
                                error!("Invalid instruction : {:#06x}", instruction_double);
                                return Err(Chip8Error::InvalidInstruction);
                            }
                        }
                    },
                    9 => { //SNE
                        if instruction_nibbles[3] != 0 {
                            error!("Invalid instruction : {:#06x}", instruction_double);
                            return Err(Chip8Error::InvalidInstruction);
                        }

                        let x_register = instruction_nibbles[1] as usize;
                        let y_register = instruction_nibbles[2] as usize;

                        if self.registers[x_register] != self.registers[y_register] {
                            self.program_counter += 2;
                        }
                    }
                    0xA => { // LD I - Set Index register
                        let new_value: u16 = ((instruction_nibbles[1] as u16) << 8) + ((instruction_nibbles[2] as u16) << 4) + instruction_nibbles[3] as u16;
                        self.index_register = new_value;
                    },
                    0xB => { //Jump V0
                        let mut target_addr: u16 = ((instruction_nibbles[1] as u16) << 8) + ((instruction_nibbles[2] as u16) << 4) + instruction_nibbles[3] as u16;
                        target_addr += self.registers[0x00] as u16;

                        self.program_counter = target_addr;
                    },
                    0xC => { //RND
                        let x_register = instruction_nibbles[1] as usize;
                        let and_mask = ((instruction_nibbles[2]) << 4) + instruction_nibbles[3];

                        let rand_value = self.rng.gen::<u8>() & and_mask;
                        self.registers[x_register] = rand_value;
                    }
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
                        let draw_return = self.draw_sprite(sprite_content, x_coord, y_coord);
                        if draw_return.is_err() {
                            let error_msg = draw_return.unwrap_err();
                            error!("Display error : {}", error_msg);
                            return Err(Chip8Error::DisplayError(error_msg));
                        }

                    },
                    0xE => { // SKP - If key pressed / not pressed
                        let x_register = instruction_nibbles[1] as usize;
                        let keycode = self.registers[x_register];
                        let scancode_option = Scancode::from_i32(self.layout_to_scancode_map[&keycode] as i32);

                        if scancode_option.is_none() {
                            return Err(Chip8Error::InvalidKeycode(keycode));
                        }

                        let scancode = scancode_option.unwrap();

                        if instruction_nibbles[2] == 9 && instruction_nibbles[3] == 0xE { //Skip if pressed
                            if keys.contains(&scancode) {
                                self.program_counter += 2;
                            }
                        } else if instruction_nibbles[2] == 0xA && instruction_nibbles[3] == 1 { //Skip if not pressed
                            if !keys.contains(&scancode) {
                                self.program_counter += 2;
                            }
                        }
                    },
                    0xF => {
                        let last_byte = ((instruction_nibbles[2]) << 4) + instruction_nibbles[3];
                        let x_register = instruction_nibbles[1] as usize;

                        match last_byte {
                            0x07 => { // Get delay timer
                                // TODO
                            },
                            0x0A => { // Wait for keypress
                                // TODO
                            },
                            0x15 => { // Set delay timer
                                // TODO
                            },
                            0x18 => { // Set sound timer
                                // TODO
                            },
                            0x1E => { // ADD Index,Vx
                                self.program_counter = ((self.program_counter as u32 + self.registers[x_register] as u32) % u32::pow(2,12)) as u16
                            },
                            _ => {
                                error!("Invalid instruction : {:#06x}", instruction_double);
                                return Err(Chip8Error::InvalidInstruction);   
                            }
                        }
                    },
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