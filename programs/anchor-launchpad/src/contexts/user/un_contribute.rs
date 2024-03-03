use crate::{error::ErrorCode, send_lamports_with_signer, MintConfigAccount};
use anchor_lang::prelude::*;

use crate::{check_and_update_sale, LaunchpadAccount, UserConfigAccount};

#[derive(Accounts)]
pub struct UnContribute<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub launchpad_account: Account<'info, LaunchpadAccount>,
    
    /// CHECK: This is safe because we're only reading the account to close it afterward.
    #[account(
        mut,
        close = contributor,
        constraint = user_config_account.authority == contributor.key(),
    )]
    pub user_config_account: Account<'info, UserConfigAccount>,
    #[account(mut)]
    pub mint_config_account: Account<'info, MintConfigAccount>,
    /// CHECK: This is safe because we're only reading the account to close it afterward.
    /// CHECK: Initialize an account to store oracle observations, the account must be created off-chain, constract will initialzied it
    #[account(
        mut,         
        constraint = add_fee_un_con.key() == launchpad_account.add_fee_un_con.key(),
    )]
    pub add_fee_un_con: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UnContribute<'info> {
    pub fn un_contribute(&mut self) -> Result<()> {
        let user_config = &mut self.user_config_account;
        let dever_address_bytes = self.launchpad_account.dever_address.to_bytes();
        let launchpad_mint = self.mint_config_account.mint.to_bytes();
        let bump = self.launchpad_account.bump;
        let launchpad = &mut self.launchpad_account;

        check_and_update_sale(launchpad)?;

        require!(
            launchpad.is_sale_active == 0
                || launchpad.is_sale_active == 1
                || launchpad.is_sale_active == 3,
            ErrorCode::NotClaimTime
        );

        let mut percent_refund: u8 = 100;

          let seeds = &[
            b"launchpad",
            &launchpad_mint[..],
            &dever_address_bytes[..],
            &[bump],
        ];
        if launchpad.is_sale_active == 0 {
            percent_refund = 90;
            if launchpad.launchpad_params_step_1.affiliate > 0
            {
               if user_config.amount_ref > 0 {
                launchpad.total_commission -= user_config.amount_ref;
               }
            }
            let amount_refund = user_config.balances * (100 - percent_refund as u64) / 100;

            launchpad.total_buyed -= user_config.balances;
            launchpad.total_contributors -= 1;

            //send refund to user
            send_lamports_with_signer(
                launchpad.to_account_info(),
                self.add_fee_un_con.to_account_info(),
                amount_refund,
                seeds
            )?;
        }

        send_lamports_with_signer(
            launchpad.to_account_info(),
            self.contributor.to_account_info(),
            user_config.balances * percent_refund as u64 / 100,
            seeds
        )?;

        Ok(())
    }
}
