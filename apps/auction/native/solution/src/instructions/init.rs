use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use super::lib::get_pda;
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

    let payer = next_account_info(account_iter)?;
    let mint_sell = next_account_info(account_iter)?;
    let mint_buy = next_account_info(account_iter)?;
    let auction_pda = next_account_info(account_iter)?;
    let auction_sell_ata = next_account_info(account_iter)?;
    let seller_sell_ata = next_account_info(account_iter)?;
    let seller_sell_buy = next_account_info(account_iter)?;
    let token_program = next_account_info(account_iter)?;
    let ata_program = next_account_info(account_iter)?;
    let sys_program = next_account_info(account_iter)?;
    let rent_sysvar = next_account_info(account_iter)?;

    // Check payer signed
    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check that the given account key matches expected PDA
    if *auction_pda.key
        != get_pda(program_id, payer.key, &mint_sell.key, &mint_buy.key, bump)?
    {
        return Err(ProgramError::InvalidSeeds);
    }

    // Create auction sell ata, check calculated matches

    Ok(())
}
