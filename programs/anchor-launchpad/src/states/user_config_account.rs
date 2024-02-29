use anchor_lang::prelude::*;
pub const USER_HISTORY_REF_SIZE_USIZE: usize = 50;

#[account]
pub struct UserConfigAccount {
  pub bump: u8,
  pub authority: Pubkey,
  pub balances: u64,
  pub commission: u64,
  pub ref_pubkey: Pubkey,
  pub claimed : bool,
  pub claimed_commission: bool,
  pub history_ref: Vec<HistoryRef>,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct HistoryRef {
  pub amount: u64,
  pub token_ref: Pubkey
}

impl Space for UserConfigAccount {
    const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
        + 1 // bump
        + 1 // is_whitelist (bool)
        + 8 * 3 // balances, commission, history_ref (u64 each)
        + 32 // ref_pubkey (Pubkey)
        + 1 * 2 // claimed, claimed_commission (bool each)
        + (32 + 8) * USER_HISTORY_REF_SIZE_USIZE;
}
