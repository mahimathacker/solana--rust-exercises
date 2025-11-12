use anchor_lang::prelude::*;

use crate::state;

#[derive(Accounts)]
pub struct Lock<'info> {
    // Order of fields in the struct is the order of accounts the client must pass them in
    // mut - Program can update account's data or lamports
    #[account(mut)]
    // Payer signed this transaction
    pub payer: Signer<'info>,
    // Owner signed this transaction
    pub owner: Signer<'info>,
    #[account(
        // Initialize new account
        // Transaction fails if this account is already initialized
        init,
        // 8 = Anchor discriminator
        space = 8 + state::Lock::INIT_SPACE,
        payer = payer
    )]
    pub oracle: Account<'info, state::Lock>,
    // Required to create oracle account
    pub system_program: Program<'info, System>,
}

pub fn lock(ctx: Context<Lock>, price: u64) -> Result<()> {
    Ok(())
}
