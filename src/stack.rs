use crate::errors::Chip8Error;

pub struct Stack {
    stack_vec: Vec::<u16>
}

impl Stack {
    pub fn new() -> Stack {
        Stack{stack_vec: Vec::<u16>::new()}
    }

    pub fn push(&mut self, value: u16) {
        self.stack_vec.push(value);
    }

    pub fn pop(&mut self) -> Result<u16, Chip8Error> {
        let value = self.stack_vec.pop();

        if value.is_none() {
            return Err(Chip8Error::TriedPoppingEmptyStack);
        } else {
            return Ok(value.unwrap());
        }
    }

    pub fn get_sp(&self) -> u8 {
        return self.stack_vec.len() as u8;
    }
}