use anchor_lang::prelude::*;

#[account]
pub struct MintConfigAccount {
  pub bump: u8,
  pub authority: Pubkey,
  pub mint: Pubkey,
  pub commission: u64,
}

impl Space for MintConfigAccount {
    const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
        + 1 // bump
        + 32 // authority
        + 32 // mint
        + 8; // creation_fee_option_sol
}