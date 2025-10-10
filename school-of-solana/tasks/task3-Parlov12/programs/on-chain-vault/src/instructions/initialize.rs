//-------------------------------------------------------------------------------
///
/// REFERENCE IMPLEMENTATION: Initialize Vault Instruction
/// 
/// This instruction is fully implemented and serves as an example for you to study
/// when implementing the other instructions (deposit, withdraw, toggle_lock).
/// 
/// Key Concepts Demonstrated:
/// - Account constraints and validation using #[account(...)] attributes
/// - PDA (Program Derived Address) creation with seeds and bump
/// - Account initialization with proper space allocation
/// - Event emission after successful operations
/// 
/// Use this as your reference when implementing the TODO instructions!
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::events::InitializeVaultEvent;

#[derive(Accounts)]
pub struct InitializeVault<'info> {

    // signer account 
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    
    // storage - vault account
    #[account(
        init, 
        payer = vault_authority, 
        // space = discriminant + account size
        space = 8 + Vault::INIT_SPACE, 
        seeds = [b"vault", vault_authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    
    pub system_program: Program<'info, System>,
}

pub fn _init_vault(ctx: Context<InitializeVault>, locked: bool) -> Result<()> {
  let vault = &mut ctx.accounts.vault; // it creates 

  vault.vault_authority = ctx.accounts.vault_authority.key(); // assigning vault_authority's key to vault
  vault.locked = locked;

  emit!(InitializeVaultEvent { // emits and event to Solana's transaction logs
    // Events are like structured log messages — they don’t store data on-chain, 
    // but they can be read by clients or indexers (like Solana Explorer or custom APIs).
    vault: vault.key(),
    vault_authority: vault.vault_authority,
    locked,
  });

  Ok(())
}