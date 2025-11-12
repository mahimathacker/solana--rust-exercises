use anchor_lang::prelude::*;

use crate::state;

#[derive(Accounts)]
pub struct Unlock<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    // Check oracle.owner == owner
    // oracle is deserialized to Oracle struct
    pub oracle: Account<'info, state::Lock>,
}

pub fn unlock(ctx: Context<Unlock>, price: u64) -> Result<()> {
    Ok(())
}
