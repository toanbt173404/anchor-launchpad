use anchor_lang::prelude::*;

use crate::LaunchpadAccount;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct UpdateTimePublicWls<'info> {
    #[account(
        mut,
        constraint = launchpad_account.dever_address == authority.key() @ ErrorCode::Unauthorized
    )]
    pub launchpad_account: Account<'info, LaunchpadAccount>,
    pub authority: Signer<'info>,
}
