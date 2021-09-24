use crate::types::Byte;

use std::ops::{Add, AddAssign};
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Debug)]
pub struct Double {
    value: u16
}

impl Double {
    pub fn new_u16(value: u16) -> Double {
        Double{value: value}
    }

    pub fn new_usize(value: usize) -> Double {
        if value > std::u16::MAX as usize {
            panic!("Value {} is too big to be in a Double", value);
        }

        Double{value: value as u16}
    }

    pub fn get_raw_value(self) -> u16 {
        self.value
    }
}

impl Into<Byte> for Double {
    fn into(self) -> Byte {
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
    fn add(self, second: usize) -> Double {
        Double::new_usize(self.get_raw_value() as usize + second)
    }
}

impl AddAssign<usize> for Double {
    fn add_assign(&mut self, value: usize) {
        self.value += value as u16;
    }
}

impl Display for Double {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:#06x}", self.value)
    }
}