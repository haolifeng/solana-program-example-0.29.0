use anchor_lang::prelude::*;

declare_id!("Gj3LJAGE486KkBPRACzcqn9xUeJSQadJvBeNtzUybDYD");

#[program]
pub mod account_data {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
