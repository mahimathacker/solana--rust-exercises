use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Lock {
    // Destination to send SOL to
    pub dst: Pubkey,
    // Lock expiration timestamp
    pub exp: u64,
}
