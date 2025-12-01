use anchor_lang::prelude::*;

declare_id!("9WUK6RjaUJ2tY5ymQiKHtoXHX1B63F2XfcHbm6FShBGc");

// Automatically generate module using program idl found in ./idls
declare_program!(counter);
use counter::program::Counter;

#[program]
pub mod factory {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        // Initialize counter state
        let cpi_accounts = counter::cpi::accounts::Init {
            payer: ctx.accounts.payer.to_account_info(),
            counter: ctx.accounts.counter.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.counter_program.to_account_info(),
            cpi_accounts,
        );
        counter::cpi::init(cpi_ctx)?;

        Ok(())
    }

    pub fn inc(ctx: Context<Inc>) -> Result<()> {
        let cpi_accounts = counter::cpi::accounts::Inc {
            counter: ctx.accounts.counter.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.counter_program.to_account_info(),
            cpi_accounts,
        );
        counter::cpi::inc(cpi_ctx)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // Need to forward signature for account creation
    #[account(mut)]
    pub counter: Signer<'info>,
    pub counter_program: Program<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Inc<'info> {
    /// CHECK: owned by the counter program
    #[account(mut)]
    pub counter: UncheckedAccount<'info>,
    pub counter_program: Program<'info, Counter>,
}
