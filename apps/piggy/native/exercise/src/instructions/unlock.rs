use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{Sysvar, clock::Clock},
};

use super::lib::get_pda;
use crate::state::Lock;

pub fn unlock(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    bump: u8,
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();
    let payer = next_account_info(account_iter)?;
    let pda = next_account_info(account_iter)?;
    let dst = next_account_info(account_iter)?;
    let sys_program = next_account_info(account_iter)?;

    // Check that the given account key matches expected PDA
let (lock_dist, lock_exp) = {
    let data = pda.data.borrow();
    let lock = Lock::try_from_slice(&data)?;
    (lock.dst, lock.exp)
};
    // Load lock state



    // Verify destination matches

    // Verify lock has expired

  

    
    let clock = Clock::get()?;
    let now: u64 = clock.unix_timestamp.try_into().unwrap();

    // Get PDA balance and transfer lamports directly
    // Rent + locked amount
    let pda_lamports = pda.lamports();

      **pda.try_borrow_mut_lamports()? = 0;
    **dst.try_borrow_mut_lamports()? = dst 
        .lamports()
        .checked_add(pda_lamports)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    // Clear out data
    pda.resize(0)?;
    // Assign the account to the System Program
    pda.assign(sys_program.key);
    Ok(())
}
