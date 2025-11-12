use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

declare_id!("AZbizcSksZLiaKoFMB5hTJGtKUf6BxhyKVd2Ffdd41Ym");

#[program]
pub mod piggy {
    pub use super::instructions::*;
    use super::*;

    pub fn lock(ctx: Context<Lock>, price: u64) -> Result<()> {
        instructions::lock(ctx, price)?;
        Ok(())
    }

    pub fn unlock(ctx: Context<Unlock>, price: u64) -> Result<()> {
        instructions::unlock(ctx, price)?;
        Ok(())
    }
}
