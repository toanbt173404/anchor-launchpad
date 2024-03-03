use anchor_lang::prelude::*;
use crate::ConfigAccount;

#[derive(Accounts)]
#[instruction(add_fee_un_con: Pubkey)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        space = ConfigAccount::INIT_SPACE,
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
        add_fee_un_con: Pubkey
    ) -> Result<()> {
        self.config_account.set_inner(ConfigAccount {
            bump: bump.config_account,        
            authority: self.authority.key(),
            creation_fee: 200000000, //0.2 SOL
            add_fee_un_con,
            creation_fee_option_sol: 99
        });
        Ok(())
    }
}