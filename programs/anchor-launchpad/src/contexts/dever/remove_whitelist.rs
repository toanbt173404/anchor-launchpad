use anchor_lang::prelude::*;

use crate::{LaunchpadAccount, WhitelistAccount};

#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"whitelist".as_ref(), launchpad_account.key().as_ref()],
        bump = whitelist_account.bump
    )]
    pub whitelist_account: Account<'info, WhitelistAccount>,
    pub launchpad_account: Account<'info, LaunchpadAccount>,
}

