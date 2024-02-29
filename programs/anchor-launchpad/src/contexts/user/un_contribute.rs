use crate::error::ErrorCode;
use anchor_lang::prelude::*;

use crate::{check_and_update_sale, send_lamports, LaunchpadAccount, UserConfigAccount};

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
    /// CHECK: This is safe because we're only reading the account to close it afterward.
    /// CHECK: Initialize an account to store oracle observations, the account must be created off-chain, constract will initialzied it
    #[account(mut)]
    pub add_fee_un_con: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UnContribute<'info> {
    pub fn un_contribute(&mut self) -> Result<()> {
        let launchpad = &mut self.launchpad_account;
        let user_config = &mut self.user_config_account;

        check_and_update_sale(launchpad)?;

        require!(
            launchpad.is_sale_active == 0
                || launchpad.is_sale_active == 1
                || launchpad.is_sale_active == 3,
            ErrorCode::NotClaimTime
        );

        let mut percent_refund: u8 = 100;

        if launchpad.is_sale_active == 0 {
            percent_refund = 90;
            if launchpad.launchpad_params_step_1.affiliate > 0
                && launchpad.launchpad_params_step_1.currency != user_config.ref_pubkey
            {
                for history_entry in user_config.history_ref.iter() {
                    if let Some(commission) = launchpad
                        .commission_data
                        .iter_mut()
                        .find(|c| c.mint_pubkey == history_entry.token_ref)
                    {
                        commission.amount -= history_entry.amount;
                        launchpad.total_commission -= history_entry.amount;
                    }
                }
            }
            let amount_refund = user_config.balances * (100 - percent_refund as u64) / 100;

            launchpad.total_buyed -= user_config.balances;
            launchpad.total_contributors -= 1;

            //send refund to user
            send_lamports(
                launchpad.to_account_info(),
                self.add_fee_un_con.to_account_info(),
                amount_refund,
            )?;
        }

        send_lamports(
            launchpad.to_account_info(),
            self.authority.to_account_info(),
            user_config.balances * percent_refund as u64 / 100,
        )?;

        Ok(())
    }
}
