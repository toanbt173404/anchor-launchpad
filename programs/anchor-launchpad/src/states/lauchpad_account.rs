use anchor_lang::prelude::*;
use crate::error::ErrorCode;

pub const COMMISION_DATA_SIZE_USIZE: usize = 50;

#[account]
pub struct LaunchpadAccount {
  pub bump: u8,
  pub add_fee_un_con: Pubkey,
  pub dever_address: Pubkey,
  pub is_sale_active: u8,
  pub creation_fee_option_sol: u8,
  pub total_contributors : u64,
  pub total_commission: u64,
  pub time_public_wls:u64,
  pub total_whitelist:u64,
  pub total_buyed: u64,
  pub total_history_refs: u64,
  pub commission_data: Vec<CommissionData>,
  pub launchpad_params_step_1: LaunchpadParamsStep1,
  pub launchpad_params_step_2: LaunchpadParamsStep2,
  pub launchpad_params_step_3: LaunchpadParamsStep3,
}

#[derive(Default, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CommissionData {
    pub mint_pubkey: Pubkey,
    pub amount: u64
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LaunchpadParamsStep1 {
    pub currency: Pubkey,
    pub listing_option: u8,
    pub affiliate: u8,
    pub total_token_to: u64,
    pub decimals: u8,
    pub contract_token: Pubkey, //mint pubkey
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LaunchpadParamsStep2 {
    pub pre_rate: u64,
    pub whitelist: u64,
    pub liquidity_lock_day: u64,
    pub router: Pubkey, //Raydium Pubkey
    pub soft_cap: u64,
    pub hard_cap: u64,
    pub min_buy: u64,
    pub max_buy: u64,
    pub type_refund: u64, // 0 - burn , 1 - refund
    pub liquidity_rate: u64,
    pub listing_rate: u64,
    pub start_time: u64,
    pub end_time: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LaunchpadParamsStep3 {
    pub logo_url: String,
    pub website: String,
    pub facebook: String,
    pub twitter: String,
    pub github: String,
    pub telegram: String,
    pub instagram: String,
    pub reddit: String,
    pub discord: String,
    pub youtube: String,
    pub description: String,
}


impl Space for LaunchpadAccount {
    // Account discriminator (8 bytes) + fixed-size fields + size of arrays
    // Note: This calculation assumes all fields are included and estimated sizes for dynamic content
    const INIT_SPACE: usize = 8 // Account discriminator added by Anchor for each account
        + 1 // bump
        + 32 // add_fee_un_con
        + 32 // dever_address
        + 1 // is_sale_active
        + 1 // creation_fee_option_sol
        + 8 // total_contributors
        + 8 // total_commission
        + 8 // time_public_wls
        + 8 // total_whitelist
        + 8 // total_buyed
        + 8 // total_history_refs
        + 75 // Placeholder size for LaunchpadParamsStep1, adjust based on actual structure
        + 128 // Placeholder size for LaunchpadParamsStep2, adjust again as needed
        + 300 // Placeholder size for LaunchpadParamsStep3, adjust based on maximum expected string lengths
        + (32 + 8) * 100;
}


#[account]
pub struct WhitelistAccount {
    pub bump: u8,
    pub launchpad_account: Pubkey,
    pub whitelist: Vec<Pubkey>,
}

impl WhitelistAccount {
    pub fn add_pubkeys(&mut self, pubkeys_to_add: Vec<Pubkey>) -> Result<()> {
        for pubkey in pubkeys_to_add {
            if !self.whitelist.contains(&pubkey) {
                if self.whitelist.len() == self.whitelist.capacity() {
                    return Err(ErrorCode::WhitelistFull.into());
                }
                self.whitelist.push(pubkey);
            } 
        }
        Ok(())
    }

    pub fn remove_pubkeys(&mut self, pubkeys_to_remove: Vec<Pubkey>) -> Result<()> {
        for pubkey in pubkeys_to_remove {
            if let Some(index) = self.whitelist.iter().position(|x| *x == pubkey) {
                self.whitelist.remove(index);
            }
        }
        Ok(())
    }
}
   