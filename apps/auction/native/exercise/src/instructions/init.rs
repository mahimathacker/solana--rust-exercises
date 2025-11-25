use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{Sysvar, clock::Clock, rent::Rent},
};

use super::lib::{create_ata, get_ata, get_pda, transfer};
use crate::state::Auction;

pub fn init(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    start_price: u64,
    end_price: u64,
    start_time: u64,
    end_time: u64,
    sell_amt: u64,
    // Auction PDA bump
    bump: u8,
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();

    let seller = next_account_info(account_iter)?;
    let mint_sell = next_account_info(account_iter)?;
    let mint_buy = next_account_info(account_iter)?;
    let auction_pda = next_account_info(account_iter)?;
    let auction_sell_ata = next_account_info(account_iter)?;
    let seller_sell_ata = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let ata_program = next_account_info(account_iter)?;
    let sys_program = next_account_info(account_iter)?;
    let rent_sysvar = next_account_info(account_iter)?;

    // Check seller signed
    // Check that auction_pda matches expected PDA
    // Check auction_sell_ata
    // Check seller_sell_ata
    // Check sell token != buy token
    // Check start_price >= end_price
    // Check now <= start_time < end_time
    let clock = Clock::get()?;
    let now: u64 = clock.unix_timestamp.try_into().unwrap();
    // Check sell_amt > 0

    // Create PDA account

    // Create auction_sell_ata

    // Send sell token to auction_sell_ata

    // Store Auction state

    Ok(())
}
