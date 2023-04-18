use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};

/// owner_wallet: Pubkey - a Pubkey struct has a size of 32 bytes.
/// user_stake_count: u32 - a u32 has a size of 4 bytes.
/// nonce: u8 - a u8 has a size of 1 byte.
pub const POOL_STORAGE_TOTAL_BYTES: usize = 37; // Should be 2 bytes less than real size of
/// We need two structures to store data about the pool and the user
#[derive(Clone, BorshDeserialize, BorshSerialize, Copy)]
pub struct Pool {
    /** place for pool data such as owner wallet (Pubkey),
          reward rate, user stake count,
          reward duration start/end and more. **/
    pub owner_wallet: Pubkey,
    pub user_stake_count: u32,
    pub nonce: u8
}
pub const USER_STORAGE_TOTAL_BYTES: usize = 65;
#[derive(Clone, BorshDeserialize, BorshSerialize, Copy)]
pub struct User {
    /** place for user data such as user wallet(Pubkey),
          pool, user balance, user id (nonce), some information about rewards
          and more. **/
    pub user_wallet: Pubkey,
    pub pool: Pubkey,
    pub nonce: u8,
}
