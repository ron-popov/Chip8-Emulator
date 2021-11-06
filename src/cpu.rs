use crate::memory::Memory;
use crate::errors::Chip8Error;
use crate::consts;
use crate::stack::Stack;
use crate::delay_timer::DelayTimer;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Scancode;

use rand::Rng;
use rand::rngs::ThreadRng;

use std::collections::HashMap;
use std::num::Wrapping;

pub struct CPU {
    memory_space: Memory,
    program_counter: u16,
    canvas: Canvas<Window>,
    stack: Stack,
    registers: [u8; 16],
    index_register: u16,
    rng: ThreadRng,
    chip_to_real_key_map: HashMap::<u8, &'static str>,
    last_real_keys: Option::<(u8, Vec::<Scancode>)>,
    delay_timer: DelayTimer,
    display_state: Vec<Vec<bool>>,
}

impl CPU {
    pub fn new(memory: Memory, canvas: Canvas<Window>) -> CPU {
        let rng = rand::thread_rng();
        CPU{memory_space: memory, program_counter: consts::PROGRAM_MEMORY_ADDR as u16, canvas: canvas, stack: Stack::new(), 
            registers: [0x00; 16], index_register: 0x00, rng: rng, chip_to_real_key_map : consts::get_chip_to_real_key_map(), 
            last_real_keys: None, delay_timer: DelayTimer::init_timer(), display_state: vec![vec![false; consts::DISPLAY_HEIGHT]; consts::DISPLAY_WIDTH]}
    }

    pub fn draw_sprite(&mut self, sprite_content: Vec<u8>, x_coord: u8, y_coord: u8) -> Result<(), String> {
        debug!("DRAW_ACTION | Displaying sprite");
        debug!("DRAW_ACTION | Sprite content : {:?}", sprite_content);
        debug!("DRAW_ACTION | Sprite coords : ({},{})", x_coord, y_coord);
    
        let mut y = (y_coord as i32) % consts::DISPLAY_HEIGHT as i32;
        for sprite in sprite_content {
            let mut value = sprite;
            
            for i in 0..8 {
                let x = (x_coord as i32 + (7 - i)) % consts::DISPLAY_WIDTH as i32;

                let is_pixel_on = self.display_state[x as usize][y as usize] as u8;
                if ((value & 0b1) ^ is_pixel_on) == 1 {
                    self.canvas.set_draw_color(Color::WHITE);
                    self.display_state[x as usize][y as usize] = true;
                } else {
                    self.canvas.set_draw_color(Color::BLACK);
                    self.display_state[x as usize][y as usize] = false;
                }
                
                value = value >> 1;
                self.canvas.fill_rect(Rect::new(x, y, 1, 1))?;
            }
            y += 1;
            y %= consts::DISPLAY_HEIGHT as i32;
        }
    
        self.canvas.present();
    
        return Ok(());
    }

    pub fn execute_instruction(&mut self, real_keys: Vec::<Scancode>) -> Result<(),Chip8Error> {
        // Check if wait for keyboard
        if self.last_real_keys.is_some() {
            // Wait for enter press
            let mut found_key = false;
            'find_key: for key in &real_keys {
                let key_name = key.name();
                trace!("KEYPAD_ACTION | Pressed key name : {}", key_name);
                if key_name == "Return" {
                    let (x_register, _) = self.last_real_keys.as_ref().unwrap();
                    self.registers[*x_register as usize] = 1;
                    self.last_real_keys = None;
                    
                    debug!("KEYPAD_ACTION | Leaving wait for keypress mode");
                    found_key = true;

                    break 'find_key;
                }
            }

            if !found_key {
                return Ok(());
            }
        }

        // Parse instruction
        let instruction_double: u16 = ((self.memory_space.get_value(self.program_counter) as u16) << 8) + self.memory_space.get_value(self.program_counter + 1) as u16;
        trace!("CURRENT_OPCODE | {:#06x} -> {:#06x}", self.program_counter, instruction_double);
        
        // Execute simple instructions
        match instruction_double {
            0x00E0 => { //CLS - Clear screen
                self.canvas.clear();
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
                        return Ok(());
                    },
                    3 => { //SE - Skip if equal
                        let register_index = instruction_nibbles[1];
                        let comp_value = (instruction_nibbles[2] << 4) + instruction_nibbles[3];

                        if self.registers[register_index as usize] == comp_value {
                            debug!("Register {:X} has value {}, skipping next instruction", register_index, comp_value);
                            self.program_counter += 2;
                        } else {
                            debug!("Register {:X} has value {} instead of {}, not skipping next instruction", 
                                register_index, self.registers[register_index as usize], comp_value);
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

                        self.registers[register_index as usize] = new_value;
                        trace!("Loading value {} to register {:#x}", new_value, register_index);
                    },
                    7 => { // ADD - Add to register
                        let add_value = (instruction_nibbles[2] << 4) + instruction_nibbles[3];
                        self.registers[instruction_nibbles[1] as usize] = (Wrapping(self.registers[instruction_nibbles[1] as usize] as u8) + Wrapping(add_value as u8)).0;
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

                                self.registers[x_register] = (Wrapping(self.registers[x_register]) - Wrapping(self.registers[y_register])).0;
                            },
                            6 => { //Shift Right
                                let before_value = self.registers[x_register];

                                self.registers[0x0F] = self.registers[x_register] & 0b00000001;
                                self.registers[x_register] = self.registers[x_register] >> 1;

                                trace!("Register {} shifted right from {} to {}", x_register, before_value, self.registers[x_register]);
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
                                let before_value = self.registers[x_register];
                                self.registers[0x0F] = self.registers[x_register] & 0b10000000;
                                self.registers[x_register] = self.registers[x_register] << 1;

                                trace!("Register {} shifted left from {} to {}", x_register, before_value, self.registers[x_register]);
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
                        let new_value: u16 = instruction_double % 0x1000;
                        self.index_register = new_value;
                    },
                    0xB => { //Jump V0
                        let mut target_addr: u16 = instruction_double % 0x1000;
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
                        trace!("Reading sprite content from address {:#06x}", sprite_memory_addr);

                        let mut sprite_content = Vec::<u8>::new();

                        for i in 0..sprite_length {
                            sprite_content.push(self.memory_space.get_value(sprite_memory_addr + i as u16));
                        }

                        let x_coord = self.registers[instruction_nibbles[1] as usize];
                        let y_coord = self.registers[instruction_nibbles[2] as usize];

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
                        let target_key_real_name = self.chip_to_real_key_map[&keycode];

                        if instruction_nibbles[2] == 9 && instruction_nibbles[3] == 0xE { //Skip if pressed
                            'find_key_exists: for key in &real_keys {
                                if target_key_real_name == key.name() {
                                    self.program_counter += 2;
                                    break 'find_key_exists;
                                }
                            }
                        } else if instruction_nibbles[2] == 0xA && instruction_nibbles[3] == 1 { //Skip if not pressed
                            // if !real_keys.contains(&scancode) {
                            //     self.program_counter += 2;
                            // }

                            let mut found_key = false;
                            'find_key_notexists: for key in &real_keys {
                                if target_key_real_name == key.name() {
                                    found_key = true;
                                    break 'find_key_notexists;
                                }
                            }

                            if !found_key {
                                self.program_counter += 2;
                            }
                        }
                    },
                    0xF => {
                        let last_byte = ((instruction_nibbles[2]) << 4) + instruction_nibbles[3];
                        let x_register = instruction_nibbles[1] as usize;


                        match last_byte {
                            0x07 => { // Get delay timer
                                self.registers[x_register] = self.delay_timer.get_value();
                            },
                            0x0A => { // Wait for keypress
                                debug!("KEYPAD_ACTION | Entering wait for keypress mode");
                                if self.last_real_keys.is_some() {
                                    return Err(Chip8Error::WaitForKeypressDuringWaitMode);
                                }

                                self.last_real_keys = Some((x_register as u8, real_keys));
                            },
                            0x15 => { // Set delay timer
                                self.delay_timer.set_value(self.registers[x_register]);
                            },
                            0x18 => { // Set sound timer
                                error!("Unimplemented instruction : {:#06x}", instruction_double);
                                return Err(Chip8Error::UnimplementedInstruction);
                                // TODO
                            },
                            0x1E => { // ADD Index,Vx
                                self.index_register = (Wrapping(self.index_register) + Wrapping(self.registers[x_register] as u16)).0;
                                trace!("Index register has value : {:#06x}", self.index_register);
                            },
                            0x29 => { // Get digit font addr
                                self.index_register = self.memory_space.get_font_addr(self.registers[x_register])?;
                            },
                            0x33 => { // Store Decimal representation of register
                                let x_value = self.registers[x_register];
                                trace!("Storing decimal representation value of {}", x_value);

                                let ones_digit: u8 = (x_value % 10) as u8;
                                let tens_digit: u8 = (x_value % 100) / 10 as u8;
                                let hunderds_digit: u8 = (x_value) / 100 as u8;

                                self.memory_space.set_value(self.index_register + 1, hunderds_digit);
                                self.memory_space.set_value(self.index_register + 2, tens_digit);
                                self.memory_space.set_value(self.index_register + 3, ones_digit);
                            },
                            0x55 => { // Store registers to memory
                                for i in 0..x_register+1 {
                                    self.memory_space.set_value(self.index_register, self.registers[i]);
                                    self.index_register += 1;
                                }
                            },
                            0x65 => { // Read register from memory
                                for i in 0..x_register+1 {
                                    self.registers[i] = self.memory_space.get_value(self.index_register);
                                    trace!("Register {}(#{:#6x}) = {}", i, self.index_register, self.registers[i]);
                                    self.index_register += 1;
                                }
                            }
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
