use crate::instruction::Instruction;

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};
use crate::processor_methods::{claim_rewards, close_pool, close_user, create_user, initialize_pool, stake, unstake};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = Instruction::unpack(instruction_data)?;
        match instruction {
            Instruction::InitializePool {nonce}=> {
                msg!("Instruction::InitializePool");
                initialize_pool(accounts,
                                nonce,
                                program_id)
            }
            Instruction::CreateUser {nonce}=> {
                msg!("Instruction::CreateUser");
                create_user(accounts, nonce, program_id)
            }

            Instruction::Stake {amount}  => {
                msg!("Instruction::Stake");
                stake(accounts, amount, program_id)
            }

            Instruction::Unstake  {amount} => {
                msg!("Instruction::Unstake");
                unstake(accounts, amount, program_id)
            }

            Instruction::ClaimRewards => {
                msg!("Instruction::ClaimRewards");
                claim_rewards(accounts, program_id)
            }

            Instruction::ClosePool => {
                msg!("Instruction::ClosePool");
                close_pool(accounts, program_id)
            }

            Instruction::CloseUser => {
                msg!("Instruction::CloseUser");
                close_user(accounts, program_id)
            }
        }
    }
}
