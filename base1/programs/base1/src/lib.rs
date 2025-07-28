use anchor_lang::prelude::*;

declare_id!("EnYjwQgVud8ZJw3V4zRaVYFc8uT6C7Z1yUsPWeHj6WsJ");

#[program]
pub mod base1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
