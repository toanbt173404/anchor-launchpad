use anchor_lang::prelude::*;

#[account]
pub struct UserConfigAccount {
  pub bump: u8,
  pub authority: Pubkey,
  pub balances: u64,
  pub commission: u64,
  pub claimed : bool,
  pub claimed_commission: bool,
  pub amount_ref: u64,
}

impl Space for UserConfigAccount {
    const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
        + 1 // bump
        + 32
        + 8
        + 8 // balances, commission, history_ref (u64 each)
        + 1 * 2 // claimed, claimed_commission (bool each)
        + 8;
}
