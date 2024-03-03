use crate::{check_during_sale_conditions, error::ErrorCode};
use anchor_lang::prelude::*;
use std::ops::Add;

use crate::{
    check_and_update_sale, LaunchpadAccount, MintConfigAccount,
    UserConfigAccount, WhitelistAccount,
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub launchpad_account: Account<'info, LaunchpadAccount>,
    #[account(mut)]
    pub mint_config_account: Account<'info, MintConfigAccount>,
    #[account(mut)]
    pub whitelist_account: Account<'info, WhitelistAccount>,
    #[account(
        init_if_needed,
        payer = contributor,
        seeds = [b"user_config".as_ref(), contributor.key().as_ref() , launchpad_account.key().as_ref()],
        bump,
        space = UserConfigAccount::INIT_SPACE
    )]
    pub user_config_account: Account<'info, UserConfigAccount>,
    pub system_program: Program<'info, System>,
}
impl<'info> Contribute<'info> {
    pub fn contribute(&mut self, amount: u64) -> Result<()> {
        let launchpad = &mut self.launchpad_account;
        let whitelist = &mut self.whitelist_account.whitelist;
        let mint_config = &mut self.mint_config_account;
        let user_config = &mut self.user_config_account;
        check_during_sale_conditions(launchpad)?;

        let contributor_pubkey = &self.contributor.to_account_info().key();

        require!(
            amount >= launchpad.launchpad_params_step_2.min_buy,
            ErrorCode::MinAmount
        );
        require!(
            amount <= launchpad.launchpad_params_step_2.max_buy,
            ErrorCode::MaxAmount
        );
        // Check hard cap
        require!(
            launchpad.total_buyed.add(amount) <= launchpad.launchpad_params_step_2.hard_cap,
            ErrorCode::HardCapReached
        );

        require!(
            whitelist.contains(&contributor_pubkey)
                || launchpad.launchpad_params_step_2.whitelist == 0
                || launchpad.time_public_wls > 0
                || Clock::get()?.unix_timestamp as u64 >= launchpad.time_public_wls,
            ErrorCode::OnlyWhitelist
        );

        if launchpad.launchpad_params_step_1.affiliate > 0
        {   
            user_config.amount_ref += amount;
            
            launchpad.total_history_refs = launchpad
                .total_history_refs
                .checked_add(1)
                .ok_or(ErrorCode::Overflow)?;
            mint_config.commission = mint_config
                .commission
                .checked_add(amount)
                .ok_or(ErrorCode::Overflow)?;
            launchpad.total_commission = launchpad
                .total_commission
                .checked_add(amount)
                .ok_or(ErrorCode::Overflow)?;
        }

        if user_config.balances == 0 {
            launchpad.total_contributors += 1;
        }

        user_config.authority = contributor_pubkey.clone();

        user_config.balances += amount;
        launchpad.total_buyed += amount;

        if launchpad.total_buyed == launchpad.launchpad_params_step_2.hard_cap {
            launchpad.is_sale_active = 2;
        } else {
            check_and_update_sale(launchpad)?;
        };
        
        Ok(())
    }
}
