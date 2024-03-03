use anchor_lang::prelude::*;

#[account]
pub struct ConfigAccount {
  pub bump: u8,
  pub authority: Pubkey,
  pub creation_fee: u64,
  pub add_fee_un_con: Pubkey,
  pub creation_fee_option_sol: u8,
}

impl Space for ConfigAccount {
    const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
        + 1 // bump
        + 32 // authority
        + 8 // creation_fee
        + 32 // add_fee_un_con
        + 1; // creation_fee_option_sol
}