use anchor_lang::prelude::*;

use crate::LaunchpadAccount;
use crate::error::ErrorCode;


pub fn send_lamports<'a>(from: AccountInfo<'a>, to: AccountInfo<'a>, amount: u64) -> Result<()> {
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &from.key(),
        &to.key(),
        amount.into(),
    );

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            from.to_account_info(),
            to.to_account_info(),
        ],
    ).map_err(|err| err.into())
}

pub fn check_during_sale_conditions(launchpad_account: &Account<LaunchpadAccount>) -> Result<()> {
    require!(launchpad_account.is_sale_active == 0, ErrorCode::SaleIsActive);
    require!(
        Clock::get()?.unix_timestamp as u64 >= launchpad_account.launchpad_params_step_2.start_time,
        ErrorCode::PreSaleNotStarted
    );
    require!(
        Clock::get()?.unix_timestamp as u64 <= launchpad_account.launchpad_params_step_2.end_time,
        ErrorCode::PreSaleEnded
    );

    Ok(())
}

pub fn check_and_update_sale(launchpad_account: &mut Account<LaunchpadAccount>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp as u64;

    if current_time > launchpad_account.launchpad_params_step_2.end_time {
        if launchpad_account.total_buyed >= launchpad_account.launchpad_params_step_2.soft_cap {
            launchpad_account.is_sale_active = 2;
        } else {
            launchpad_account.is_sale_active = 3;
        }
    }
    Ok(())
}