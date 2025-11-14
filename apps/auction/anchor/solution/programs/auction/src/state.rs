use anchor_lang::prelude::*;

// - state (sell token, buy token, start price, min price, start time, end time, seller)
#[account]
#[derive(InitSpace)]
pub struct Auction {
    pub sell_mint: Pubkey,
    pub buy_mint: Pubkey,
    pub start_price: u64,
    pub end_price: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub seller: Pubkey,
}

impl Auction {
    pub const SEED_PREFIX: &'static [u8; 7] = b"auction";
}
