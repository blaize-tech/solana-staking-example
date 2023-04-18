use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum CustomError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    // more errors which can be in solana contract
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
