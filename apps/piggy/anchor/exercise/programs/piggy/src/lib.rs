use anchor_lang::prelude::*;


pub mod error;        // ← Add this
pub mod state;        // ← Add this
pub mod instructions;

declare_id!("7NUrHk28BSWHor7Jt7fm5iwxyq3pujmnUYqeUoKpns5c");

#[program]
pub mod piggy {
    pub use super::instructions::*;
    use super::*;

    pub fn lock(ctx: Context<Lock>, amt: u64, exp: u64) -> Result<()> {
        instructions::lock(ctx, amt, exp);
        Ok(())
    }

    pub fn unlock(ctx: Context<Unlock>) -> Result<()> {
        instructions::unlock(ctx);
        Ok(())
    }
}
