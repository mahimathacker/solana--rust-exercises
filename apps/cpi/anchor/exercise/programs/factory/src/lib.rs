use anchor_lang::prelude::*;

declare_id!("Fqe7emgpwa4pfCwVQN54LRTimp8rx8PCBg2n9CZPzrG8");
declare_program!(counter);
use counter::program::Counter;

// Automatically generate module using program idl found in ./idls
// Inport IDL for counter program

#[program]
pub mod factory {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        // Invoke the function init on the counter program
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
    // Add counter_program
    pub counter_program: Program<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Inc<'info> {
    /// CHECK: owned by the counter program
    #[account(mut)]
    pub counter: UncheckedAccount<'info>,
    // Add counter_program
    pub counter_program: Program<'info, Counter>,
}
