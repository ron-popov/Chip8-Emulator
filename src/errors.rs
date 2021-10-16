#[derive(Debug)]
pub enum Chip8Error {
    InvalidInstruction,
    TriedPoppingEmptyStack,
    DisplayError(String)
}