use crate::types::Double;

#[derive(Copy, Clone, Debug)]
pub struct Byte {
    value: u8,
}

impl Byte {
    pub fn new(value: u8) -> Byte {
        Byte{value: value}
    }

    pub fn get_raw_value(self: Self) -> u8 {
        self.value
    }
}

impl Into<usize> for Byte {
    fn into(self: Self) -> usize {
        return self.value as usize;
    }
}

impl Into<u16> for Byte {
    fn into(self: Self) -> u16 {
        return self.value as u16;
    }
}

impl Into<Double> for Byte {
    fn into(self: Self) -> Double {
        // Value of byte probably won't be to big to be in a double ...
        return Double::new_u16(self.get_raw_value() as u16);
    }
}

impl From<usize> for Byte {
    fn from(value: usize) -> Self {
        return Byte{value: value as u8}
    }
}
