use crate::{error::ErrorCode, send_lamports_with_signer, MintConfigAccount};
use anchor_lang::{prelude::*, system_program::Transfer};

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
        /// CHECK: This is safe because we're only reading the account to close it afterward.
    /// CHECK: Initialize an account to store oracle observations, the account must be created off-chain, constract will initialzied it
    #[account(
        mut,         
    )]
    pub dever: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UnContribute<'info> {
    pub fn un_contribute(&mut self) -> Result<()> {
        let binding = self.dever.key();
        let dever_address = binding.as_ref();
        // let launchpad_mint = self.mint_config_account.mint.as_ref();
        let bump = self.launchpad_account.bump;

        let amount_lamports = self.launchpad_account.to_account_info().lamports().clone();
        let user_balance = self.user_config_account.balances;
        let is_sale_active = self.launchpad_account.is_sale_active;

        let launchpad = &mut self.launchpad_account;

        check_and_update_sale(launchpad)?;

        require!(
            is_sale_active == 0 || is_sale_active == 1 || is_sale_active == 3,
            ErrorCode::NotClaimTime
        );

        let mut percent_refund: u8 = 100;
        let seeds = &[&b"launchpad"[..], dever_address, &[bump][..]];

        if is_sale_active == 0 {
            percent_refund = 90;
            if launchpad.launchpad_params_step_1.affiliate > 0 {
                if self.user_config_account.amount_ref > 0 {
                    launchpad.total_commission -= self.user_config_account.amount_ref;
                }
            }
            let amount_refund = self.user_config_account.balances * (100 - percent_refund as u64) / 100;

            launchpad.total_buyed -= self.user_config_account.balances;
            launchpad.total_contributors -= 1;


            msg!("Launchpad account lamports: {}", amount_lamports);

        
            //Send refund to user
            // send_lamports_with_signer(
            //     launchpad.to_account_info(),
            //     launchpad.to_account_info(),
            //     1,
            //     seeds
            // )?;
        }
        // send_lamports_with_signer(
        //     launchpad.to_account_info(),
        //     self.contributor.to_account_info(),
        //     user_balance * percent_refund as u64 / 100,
        //     seeds
        // )?;
        Ok(())
    }
}
