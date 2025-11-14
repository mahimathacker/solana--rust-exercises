use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{Burn, MintTo, Transfer},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

pub fn transfer<'info>(
    token_program: &Interface<'info, TokenInterface>,
    src: &InterfaceAccount<'info, TokenAccount>,
    dst: &InterfaceAccount<'info, TokenAccount>,
    auth: &Signer<'info>,
    amt: u64,
) -> Result<()> {
    token::transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: src.to_account_info(),
                to: dst.to_account_info(),
                authority: auth.to_account_info(),
            },
        ),
        amt,
    )
}
