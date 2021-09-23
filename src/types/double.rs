use crate::types::Byte;

use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct Double {
    value: u16
}

impl Double {
    pub fn new(value: u16) -> Double {
        Double{value: value}
    }

    pub fn get_raw_value(self: Self) -> u16 {
        self.value
    }
}

impl Into<Byte> for Double {
    fn into(self: Self) -> Byte {
        Byte::new(self.value as u8)
    }
}

impl From<usize> for Double {
    fn from(value: usize) -> Double {
        Double{value: value as u16}
    }
}

impl Add<usize> for Double {
    type Output = Double;
    fn add(self: Self, second: usize) -> Double {
        Double::new(self.get_raw_value() + second as u16)
    }
}