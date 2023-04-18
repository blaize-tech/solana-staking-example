use std::convert::TryInto;
use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::{msg, system_instruction};
use solana_program::sysvar::Sysvar;
use crate::state::{User, USER_STORAGE_TOTAL_BYTES, POOL_STORAGE_TOTAL_BYTES, Pool};
use borsh::{BorshDeserialize, BorshSerialize};

pub fn initialize_pool(
    accounts: &[AccountInfo],
    nonce: u8,
    program_id: &Pubkey,
) -> ProgramResult {
    // get accounts
    let account_info_iter = &mut accounts.iter();
    let pool_owner_wallet_account = next_account_info(account_info_iter)?;
    let pool_storage_account = next_account_info(account_info_iter)?;

    let pool_storage_account_clone = pool_storage_account.clone();
    let mut pool_data_byte_array = pool_storage_account_clone
        .data
        .try_borrow_mut()
        .unwrap();
    let  mut pool_data: Pool =
        Pool::try_from_slice(&pool_data_byte_array[0usize..POOL_STORAGE_TOTAL_BYTES])
            .unwrap();

    pool_data.owner_wallet = *pool_owner_wallet_account.key;
    pool_data.user_stake_count = 0u32;
    pool_data.nonce = nonce;
    pool_data_byte_array[0usize..POOL_STORAGE_TOTAL_BYTES]
        .copy_from_slice(&pool_data.try_to_vec().unwrap());


    pool_data_byte_array[0usize..POOL_STORAGE_TOTAL_BYTES]
        .copy_from_slice(&pool_data.try_to_vec().unwrap());
    Ok(())
}

pub fn create_user(
    accounts: &[AccountInfo],
    nonce: u8,
    program_id: &Pubkey,
) -> ProgramResult {
    // get accounts
    let account_info_iter = &mut accounts.iter();
    let user_wallet_account = next_account_info(account_info_iter)?;
    let user_storage_account = next_account_info(account_info_iter)?;
    let pool_storage_account = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    let (user_storage_address, bump_seed) = get_user_storage_address_and_bump_seed(
        user_wallet_account.key,
        pool_storage_account.key,
        program_id,
    );

    let user_storage_account_signer_seeds: &[&[_]] = &[
        &user_wallet_account.key.to_bytes(),
        &pool_storage_account.key.to_bytes(),
        &[bump_seed],
    ];

    // create user account and allocate memory
    create_and_allocate_account_raw(
        *program_id,
        user_storage_account,
        system_program_info,
        user_wallet_account,
        USER_STORAGE_TOTAL_BYTES,
        user_storage_account_signer_seeds,
    )
        .unwrap();

    let user_storage_data = User {
        user_wallet: *user_wallet_account.key,
        pool: Default::default(),
        nonce: nonce,
    };

    // get user data
    let mut user_data_byte_array = user_storage_account.data.try_borrow_mut().unwrap();
    user_data_byte_array[0usize..USER_STORAGE_TOTAL_BYTES]
        .copy_from_slice(&user_storage_data.try_to_vec().unwrap());

    // get pool data from pool account
    let mut pool_data_byte_array = pool_storage_account.data.try_borrow_mut().unwrap();
    let mut pool_data: Pool =
        Pool::try_from_slice(&pool_data_byte_array[0usize..POOL_STORAGE_TOTAL_BYTES])
            .unwrap();

    // update pool
    pool_data.user_stake_count += 1u32;
    pool_data_byte_array[0usize..POOL_STORAGE_TOTAL_BYTES]
        .copy_from_slice(&pool_data.try_to_vec().unwrap());

    Ok(())
}

pub fn stake(
    accounts: &[AccountInfo],
    amount_to_deposit: u64,
    program_id: &Pubkey,
) -> ProgramResult {

    // 1. get accounts
    // 2. get pool data from pool storage account
    // 3. get user data from user storage account
    // 4. get staking vault from staking account
    // 5. transfer tokens to staking account
    // 6. update user data (user balance)
    // 7. update pool

    Ok(())
}

pub fn unstake(
    accounts: &[AccountInfo],
    amount_to_withdraw: u64,
    program_id: &Pubkey,
) -> ProgramResult {

    // 1. get accounts
    // 2. get pool data from pool storage account
    // 3. get user data from user storage account
    // 4. transfer tokens to user from Staking Vault
    // 5. update user data (user balance)
    // 6. update pool

    Ok(())
}

pub fn claim_rewards(
    accounts: &[AccountInfo],
    program_id: &Pubkey
) -> ProgramResult {

    // 1. get accounts
    // 2. get user data from user storage account
    // 3. get pool data from pool storage account
    // 4. get staking vault data from staking vault account
    // 5. get reward vault data from reward vault account
    // 6. transfer tokens to user from Rewards Vault
    // 7. update pool
    // 8. update user

    Ok(())
}


pub fn close_user(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
) -> ProgramResult {

    // 1. get accounts
    // 2. get user data from user storage account
    // 3. get pool data from pool storage account
    // 4. update pool
    // 5. close account. account data byte array set []

    Ok(())
}

pub fn close_pool(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
) -> ProgramResult {

    // 1. get accounts
    // 2. get pool data from pool storage account
    // 3. get staking vault data from staking vault account
    // 4. transfer tokens to staking refund from staking vault
    // 5. get reward vault data from reward vault account
    // 6. transfer tokens to Rewards Refund from Rewards Vault
    // 7. close Staking Vault
    // 8. close Rewards Vault
    // 9. update pool

    Ok(())
}

pub fn create_and_allocate_account_raw<'a>(
    owner_program_id: Pubkey,
    new_account_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
) -> Result<(), ProgramError> {
    let rent = Rent::get()?;
    let required_lamports = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    // Transfer lamports to the new account
    if required_lamports > 0 {
        invoke(
            &system_instruction::transfer(payer_info.key, new_account_info.key, required_lamports),
            &[
                payer_info.clone(),
                new_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    // Allocate space for the account
    invoke_signed(
        &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
        &[new_account_info.clone(), system_program_info.clone()],
        &[signer_seeds],
    )?;

   //  Assign the account to the owning program
    invoke_signed(
        &system_instruction::assign(new_account_info.key, &owner_program_id),
        &[new_account_info.clone(), system_program_info.clone()],
        &[signer_seeds],
    )?;

    Ok(())
}

pub fn get_user_storage_address_and_bump_seed(
    user_wallet: &Pubkey,
    pool_storage: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[&user_wallet.to_bytes(), &pool_storage.to_bytes()],
        program_id,
    )
}