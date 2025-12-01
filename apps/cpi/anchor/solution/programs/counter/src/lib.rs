use anchor_lang::prelude::*;

declare_id!("6n5mmdpDHxuyZBkju2cQvrhnfhmQxMemXAss45oJYqW9");

#[account]
#[derive(InitSpace, Debug)]
pub struct Counter {
    pub count: u64,
}

#[program]
pub mod counter {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        Ok(())
    }

    pub fn inc(ctx: Context<Inc>) -> Result<()> {
        ctx.accounts.counter.count =
            ctx.accounts.counter.count.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        space = 8 + Counter::INIT_SPACE,
        payer = payer
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Inc<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
