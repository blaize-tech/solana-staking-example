use std::convert::TryInto;
use crate::error::CustomError::InvalidInstruction;
use solana_program::program_error::ProgramError;

/// Enum with which you contract
/// determine the desired instruction
/// one instruction = one method
pub enum Instruction {
    InitializePool {
        nonce: u8
    },
    CreateUser {
        nonce: u8,
    },
    Stake {
        amount: u64,
    },
    Unstake {
        amount: u64,
    },
    ClaimRewards,
    ClosePool,
    CloseUser,
}

impl Instruction {
    // method for getting instruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Ok(match input[0] {
            0 => Self::InitializePool { nonce: input[1] },
            1 => Self::CreateUser { nonce: input[1] },
            2 => Self::Stake {
                amount: Self::unpack_to_u64(&input[1..9])?,
            },
            3 => Self::Unstake {
                amount: Self::unpack_to_u64(&input[1..9])?,
            },
            4 => Self::ClaimRewards,
            5 => Self::ClosePool,
            6 => Self::CloseUser,
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_to_u64(input: &[u8]) -> Result<u64, ProgramError> {
        let out_value = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(out_value)
    }
}