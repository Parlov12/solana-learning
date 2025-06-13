use anchor_lang::prelude::*;

// the program ID (unique identifier for this Solana program) on blockchain
declare_id!("4vgNbvP3WttTGisioWSQPmQtxF6ancU9oYZDCYVApdpE");

// anchor writes an 8-byte discriminator to every account for identification
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program] // turns this Rust module into a Solana program
pub mod favorites {
    use super::*;

    // instruction: lets users set their favorite number, color, and hobbies
    pub fn set_favorites(
        context: Context<SetFavorites>, // context holds the accounts used in this instruction
        number: u64,
        color: String,
        hobbies: Vec<String>
    ) -> Result<()> {
        msg!("Greeting from {}", context.program_id); // logs to the Solana Explorer
        msg!("This is a Solana program that allows users to set their favorite number, color, and hobbies.");

        let user_public_key = context.accounts.user.key();
        msg!("User's public key: {}", user_public_key);
        msg!("User's favorite number: {}", number);
        msg!("User's favorite color: {}", color);
        msg!("User's favorite hobbies: {:?}", hobbies); // debug-print the hobbies

        // write data to the on-chain account
        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        msg!("Favorites account has been set for user: {}", user_public_key);
        Ok(())
    }
}

// defines the structure of the Favorites account stored on-chain
#[account]
#[derive(InitSpace)] // derives logic to calculate required space
pub struct Favorites {
    pub number: u64,

    #[max_len(50)] // max 50 characters for color
    pub color: String,

    #[max_len(5, 50)] // max 5 strings, each up to 50 characters
    pub hobbies: Vec<String>,
}

// defines the accounts required by the set_favorites instruction
#[derive(Accounts)]
pub struct SetFavorites<'info> {

    #[account(mut)] // user must sign and may be charged SOL
    pub user: Signer<'info>,

    #[account(
        init, // creates the account
        payer = user, // user pays for the account creation
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, // compute account space
        seeds = [b"favorites", user.key().as_ref()], // PDA seeds
        bump // bump to avoid collision
    )]
    
    pub favorites: Account<'info, Favorites>, // the actual data account

    pub system_program: Program<'info, System>, // used to create accounts
}
