use anchor_lang::prelude::*;

declare_id!("6pd1a8TTiKytoML9juVyMYcXrborBndhSGZLsxAhmH96");

#[program]
pub mod meme_one {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
