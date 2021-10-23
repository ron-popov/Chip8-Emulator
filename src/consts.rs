use std::collections::HashMap;

pub const MEMORY_SIZE: usize = 4096;
pub const PROGRAM_MEMORY_ADDR: usize = 0x200;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub fn get_layout_to_scancode_map() -> HashMap<u8, u16> {
    let mut map = HashMap::<u8, u16>::new();

    //// Chip 8 Keyboard
    // 1 | 2 | 3 | C
    // -------------
    // 4 | 5 | 6 | D
    // -------------
    // 7 | 8 | 9 | E
    // -------------
    // A | 0 | B | F

    map.insert(1  , 0x31); // 1
    map.insert(2  , 0x32); // 2
    map.insert(3  , 0x33); // 3
    map.insert(0xC, 0x34); // 4
    
    map.insert(4  , 0x51); // Q
    map.insert(5  , 0x57); // W
    map.insert(6  , 0x45); // E
    map.insert(0xD, 0x52); // R
    
    map.insert(7  , 0x41); // A
    map.insert(8  , 0x53); // S
    map.insert(9  , 0x44); // D
    map.insert(0xE, 0x46); // F

    map.insert(0xA, 0x5A); // Z
    map.insert(0  , 0x58); // X
    map.insert(0xB, 0x43); // C
    map.insert(0xF, 0x56); // V

    return map;
}