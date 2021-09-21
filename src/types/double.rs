use crate::types::Byte;

#[derive(Copy, Clone, Debug)]
pub struct Double {
    pub value: u16
}

impl Double {
    pub fn get_raw_value(self: Self) -> u16 {
        self.value
    }
}

