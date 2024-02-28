use anchor_lang::prelude::*;
use crate::{send_lamports, states::LaunchpadAccount, ConfigAccount, LaunchpadParamsStep1, LaunchpadParamsStep2, LaunchpadParamsStep3, MintConfigAccount};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer, transfer},
};
#[derive(Accounts)]
pub struct CreatLaunchpad<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub launchpad_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = authority,
        space = LaunchpadAccount::INIT_SPACE,
        seeds = [b"lauchpad".as_ref(), &authority.key().as_ref()],
        bump
    )]
    pub launchpad_account: Account<'info, LaunchpadAccount>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = launchpad_mint,
        associated_token::authority = launchpad_account
    )]
    pub vault_launchpad: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"config".as_ref(), &authority.key().as_ref()],
        bump
    )]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(
        init_if_needed,
        payer = authority,
        space = MintConfigAccount::INIT_SPACE,
        seeds = [b"mint_config".as_ref(), &authority.key().as_ref()],
        bump
    )]
    pub mint_config_account: Account<'info, MintConfigAccount>,
    #[account(
        mut,
        associated_token::mint = launchpad_mint,
        associated_token::authority = authority
    )]
    pub authority_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreatLaunchpad<'info> {
    pub fn create_launchpad_data(
        &mut self,
        bump: &CreatLaunchpadBumps,
        params1: LaunchpadParamsStep1,
        params2: LaunchpadParamsStep2,
        params3: LaunchpadParamsStep3,
    ) -> Result<()> {
        //send creation fee to pool 
        let authority =  &mut self.authority;
        let launchpad_account = &mut self.launchpad_account;
        send_lamports(authority.to_account_info(), launchpad_account.to_account_info(), self.config_account.creation_fee)?;

        self.launchpad_account.set_inner(LaunchpadAccount {
            bump: bump.launchpad_account,
            add_fee_un_con : self.config_account.add_fee_un_on,
            dever_address : self.authority.key(),
            is_sale_active: 0,
            creation_fee_option_sol: self.config_account.creation_fee_option_sol,
            total_contributors: 0,
            total_commission: 0,
            time_public_wls: 0,
            total_whitelist: 0,
            total_buyed: 0,
            total_history_refs: 0,
            launchpad_params_step_1: params1,
            launchpad_params_step_2: params2,
            launchpad_params_step_3: params3,
        });

        self.mint_config_account.set_inner(MintConfigAccount {
            bump: bump.launchpad_account,
            authority: self.authority.key(),
            mint: self.launchpad_mint.key(),
            commission: 0,
        });
        Ok(())
    }

    pub fn send_tokens_to_pool(&self, amount_to_sell: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.authority_token_account.to_account_info(),
            to: self.vault_launchpad.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        transfer(
            CpiContext::new(self.token_program.to_account_info(), cpi_accounts),
            amount_to_sell.into()
        )
    }
}