use anchor_lang::prelude::*;
use crate::{states::LaunchpadAccount, ConfigAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        space = LaunchpadAccount::INIT_SPACE,
        seeds = [b"config".as_ref(), &authority.key().as_ref()],
        bump
    )]
    pub config_account: Account<'info, ConfigAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        bump: &InitializeBumps,
        add_fee_un_on: Pubkey
    ) -> Result<()> {
        self.config_account.set_inner(ConfigAccount {
            bump: bump.config_account,        
            authority: self.authority.key(),
            creation_fee: 20000000, //0.2 SOL
            add_fee_un_on,
            creation_fee_option_sol: 99
        });
        Ok(())
    }
}