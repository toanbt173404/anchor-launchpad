use anchor_lang::prelude::*;
use crate::error::ErrorCode;

use crate::{check_and_update_sale, LaunchpadAccount, UserConfigAccount};

#[derive(Accounts)]
pub struct UnContribute<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub launchpad_account: Account<'info, LaunchpadAccount>,
    /// CHECK: This is safe because we're only reading the account to close it afterward.
    #[account(
        mut,
        close = authority,
        has_one = authority,
        constraint = user_config_account.authority == authority.key(),
    )]
    pub user_config_account: Account<'info, UserConfigAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> UnContribute<'info> {
    pub fn un_contribute(&mut self) -> Result<()> {

        let launchpad = &mut self.launchpad_account;
        let user_config = &mut self.user_config_account;

        check_and_update_sale(launchpad)?;

        require!(launchpad.is_sale_active == 0 || launchpad.is_sale_active == 1 || launchpad.is_sale_active == 3, ErrorCode::NotClaimTime);
        
        let mut percentRefund: u8 = 100;

        if launchpad.is_sale_active == 0 {
            percentRefund = 90;

            if launchpad.launchpad_params_step_1.affiliate > 0 && launchpad.launchpad_params_step_1.currency != user_config.ref_pubkey {
                
            }
        }



        Ok(())
    }
}
