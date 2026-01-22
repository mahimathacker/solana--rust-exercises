use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

declare_id!("9QUtGjgmcPE3G5wLyF6QRgocAtG7dZZ7madGaGYmXVeP");

#[program]
pub mod auction {
    pub use super::instructions::*;
    use super::*;

    pub fn init(
        ctx: Context<Init>,
        start_price: u64,
        end_price: u64,
        start_time: u64,
        end_time: u64,
        sell_amt: u64,
    ) -> Result<()> {
 instructions::init(
            ctx,
            start_price,
            end_price,
            start_time,
            end_time,
            sell_amt,
        )?;
        Ok(())
    }

    pub fn buy(ctx: Context<Buy>, max_price: u64) -> Result<()> {
        instructions::buy::buy(ctx, max_price)?;
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        instructions::cancel::cancel(ctx)?;
        Ok(())
    }
}
