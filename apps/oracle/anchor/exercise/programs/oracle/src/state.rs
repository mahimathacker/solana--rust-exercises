use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
// InitSpace - automatically calculates the space needed, Oracle::INIT_SPACE
pub struct Oracle {
    pub owner: Pubkey, //Update who can call this instruction
    pub price: u64,   // Update the price field
}
