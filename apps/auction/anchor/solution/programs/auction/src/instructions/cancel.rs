use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::error;
use crate::state;

// buy
// - close PDA
// - refund seller (buy token + PDA rent)
// - send sell token to buyer
// cancel
// - close PDA
// - refund seller (token + PDA rent)

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub sell_mint: InterfaceAccount<'info, Mint>,
    pub buy_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [
            state::Auction::SEED_PREFIX, payer.key().as_ref(), sell_mint.key().as_ref(), buy_mint.key().as_ref()
        ],
        bump,
        close = payer,
        constraint = auction.seller == payer.key() @ error::Error::Unauthorized
    )]
    pub auction: Account<'info, state::Auction>,

    /*
    #[account(
        mut,
        associated_token::mint = sell_mint,
        associated_token::authority = auction,
        close = payer,
    )]
    pub auction_sell_ata: InterfaceAccount<'info, TokenAccount>,
    */
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
    Ok(())
}
