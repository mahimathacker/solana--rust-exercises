use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::error;
use crate::state;

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: Receiver of buy token
    pub seller: UncheckedAccount<'info>,

    pub sell_mint: InterfaceAccount<'info, Mint>,
    pub buy_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [
            state::Auction::SEED_PREFIX, seller.key().as_ref(), sell_mint.key().as_ref(), buy_mint.key().as_ref()
        ],
        bump,
        close = seller,
    )]
    pub auction: Account<'info, state::Auction>,

    /*
    #[account(
        mut,
        associated_token::mint = sell_mint,
        associated_token::authority = auction,
        close = seller,
    )]
    pub auction_sell_ata: InterfaceAccount<'info, TokenAccount>,
    */
    #[account(
        mut,
        associated_token::mint = buy_mint,
        associated_token::authority = seller,
    )]
    pub seller_buy_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn buy(ctx: Context<Buy>) -> Result<()> {
    Ok(())
}
