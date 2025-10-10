use anchor_lang::prelude::*;

declare_id!("7BwAjFvcUHHo2NWWSHQ3tX3wcrptM3TZbheTyvWNhpKd");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>, _poll_id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {

    #[account(mut)]
    pub signer: Signer<'info> // signer account

    #[account()]
    pub poll: Account<'info, Poll>

}

#[account]
pub struct Poll {
    pub poll_id: u64,
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub poll_index
}