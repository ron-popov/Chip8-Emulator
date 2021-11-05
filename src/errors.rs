#[derive(Debug)]
pub enum Chip8Error {
    InvalidInstruction,
    TriedPoppingEmptyStack,
    DisplayError(String),
    InvalidKeycode(u8),
    UnimplementedInstruction,
    WaitForKeypressDuringWaitMode
}