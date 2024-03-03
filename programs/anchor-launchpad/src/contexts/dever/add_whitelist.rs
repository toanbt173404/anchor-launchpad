use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::{LaunchpadAccount, WhitelistAccount};

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(mut)]
    pub dever: Signer<'info>,
    #[account(
        init_if_needed,
        payer = dever,
        seeds = [b"whitelist".as_ref(), launchpad_account.key().as_ref(), dever.key().as_ref()],
        bump,
        space = 8 + 1 + 32 + 4 + (32 * 100)
    )]
    pub whitelist_account: Account<'info, WhitelistAccount>,
    #[account(
        mut,
        constraint = launchpad_account.dever_address == dever.key() @ ErrorCode::Unauthorized
    )]
    pub launchpad_account: Account<'info, LaunchpadAccount>,
    pub system_program: Program<'info, System>,
}


