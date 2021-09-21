use crate::types::Double;

#[derive(Copy, Clone, Debug)]
pub struct Byte {
    pub value: u8
}

impl Byte {
    pub fn get_raw_value(self: Self) -> u8 {
        self.value
    }
}

impl Into<usize> for Byte {
    fn into(self: Self) -> usize {
        return self.value.into();
    }
}

impl Into<u16> for Byte {
    fn into(self: Self) -> u16 {
        return self.value.into();
    }
}

impl Into<Double> for Byte {
    fn into(self: Self) -> Double {
        return Double{value: self.get_raw_value().into()};
    }
}

impl From<usize> for Byte {
    fn from(value: usize) -> Self {
        return Byte{value: value as u8}
    }
}
