use std::collections::HashMap;

pub const MEMORY_SIZE: usize = 4096;
pub const PROGRAM_MEMORY_ADDR: usize = 0x200;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const SCALE_FACTOR: usize = 8;
pub const DELAY_TIMER_TICK_MILLIS: f32 = 1000 as f32 / 0xff as f32;

pub const FONT_START_ADDR: usize = 0x00;
pub const FONT_CONTENT: [[u8; 5]; 16] = [
    [0b11110000, 0b10010000, 0b10010000, 0b10010000, 0b11110000], 
    [0b00100000, 0b01100000, 0b00100000, 0b00100000, 0b01110000],
    [0b11110000, 0b00010000, 0b11110000, 0b10000000, 0b11110000],
    [0b11110000, 0b00010000, 0b11110000, 0b00010000, 0b11110000],
    [0b10010000, 0b10010000, 0b11110000, 0b00010000, 0b00010000],
    [0b11110000, 0b10000000, 0b11110000, 0b00010000, 0b11110000],
    [0b11110000, 0b10000000, 0b11110000, 0b10010000, 0b11110000],
    [0b11110000, 0b00010000, 0b00100000, 0b01000000, 0b01000000],
    [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b11110000],
    [0b11110000, 0b10010000, 0b11110000, 0b00010000, 0b11110000],
    [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b10010000],
    [0b11100000, 0b10010000, 0b11100000, 0b10010000, 0b11100000],
    [0b11110000, 0b10000000, 0b10000000, 0b10000000, 0b11110000],
    [0b11100000, 0b10010000, 0b10010000, 0b10010000, 0b11100000],
    [0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b11110000],
    [0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b10000000]
];

pub fn get_chip_to_real_key_map() -> HashMap<u8, &'static str> {
    let mut map = HashMap::<u8, &str>::new();

    //// Chip 8 Keyboard
    // 1 | 2 | 3 | C
    // -------------
    // 4 | 5 | 6 | D
    // -------------
    // 7 | 8 | 9 | E
    // -------------
    // A | 0 | B | F

    map.insert(1  , "1"); // 1
    map.insert(2  , "2"); // 2
    map.insert(3  , "3"); // 3
    map.insert(0xC, "4"); // 4
    
    map.insert(4  , "Q"); // Q
    map.insert(5  , "W"); // W
    map.insert(6  , "E"); // E
    map.insert(0xD, "R"); // R
    
    map.insert(7  , "A"); // A
    map.insert(8  , "S"); // S
    map.insert(9  , "D"); // D
    map.insert(0xE, "F"); // F

    map.insert(0xA, "Z"); // Z
    map.insert(0  , "X"); // X
    map.insert(0xB, "C"); // C
    map.insert(0xF, "V"); // V

    return map;
}