use anchor_lang::prelude::*;

declare_id!("6YtvoySana42x5pEmCA6AubK3K1bTYQNoMJFv5Fv4E5t");


// main logic of program
#[program] // macro
pub mod lesson_3 { // module name
    use super::*;

    // functions that program supports
    pub fn initialize(ctx: Context<Initialize>, hello: String) -> Result<()> { // Initialize - structure that holds accounts
        msg!("Greetings from: {:?}", ctx.program_id);

        let data_account = &mut ctx.accounts.data_account; // mutable reference - so we can update account

        data_account.hello = hello;

        Ok(())
    }

    // pub fn update(ctx: Context<Update>) -> Result<()> { 
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    // pub fn transfer(ctx: Context<Update>) -> Result<()> {
    //     //TO-DO
    // }
}

#[derive(Accounts)] 
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, // signer is the one paying rent
    
    #[account( // anchor constraints
        init, // this means when program is initializes, we want this data account to be initialized as well
        payer = signer, // who is paying rent
        space = 200, // 200 bytes
    )]
    pub data_account: Account<'info, DataAccountWhatever>, // it expects data account of type DataAccountWhatever // 'info is lifetime
    pub system_program: Program<'info, System>, // it is neccessary to include SystemProgram since by deafult, SystemProgram is owner of account - 
    // before it is fully initialized
}

#[account] // macro - account 
pub struct DataAccountWhatever { // can be named whatever
    // in data account we can store for example any kind of strng
    pub hello: String,
}


// defines what account instructions within program expect in input
// #[derive(Accounts)] 
// pub struct Update {}

