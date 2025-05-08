#[allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
mod instructions;
mod state;
use instructions::*;

declare_id!("9eNCujEazpA1EicFUoP8RFf2VfrU5ziWRsaiaJa37k9P");

#[program] 
pub mod carbonpay_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}


