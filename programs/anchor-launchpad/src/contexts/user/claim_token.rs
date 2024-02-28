use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use crate::error::ErrorCode;

use crate::{ LaunchpadAccount, MintConfigAccount, UserConfigAccount};

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub claimer: Signer<'info>,
    pub launchpad_mint: Account<'info, Mint>,
    #[account(mut)]
    pub launchpad_account: Account<'info, LaunchpadAccount>,
    #[account(mut)]
    pub mint_config_account: Account<'info, MintConfigAccount>,
    #[account(
        init_if_needed,
        payer = claimer,
        associated_token::mint = launchpad_mint,
        associated_token::authority = claimer
    )]
    pub claimer_token_account: Account<'info, TokenAccount>,
    #[account(
        mut, 
        constraint = user_config_account.authority == claimer.key(),
    )]
    pub user_config_account: Account<'info, UserConfigAccount>,
    #[account(
      mut
    )]
    pub vault_launchpad: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimTokens<'info> {
    pub fn claimTokens(&mut self) -> Result<()> {
        let launchpad = &mut self.launchpad_account;
        let user_config_account = &mut self.user_config_account;

        require!(
            launchpad.is_sale_active == 4,
            ErrorCode::MinAmount
        );
        require!(
            !user_config_account.claimed,
            ErrorCode::ClaimedToken
        );

        let amount = user_config_account.balances * launchpad.launchpad_params_step_2.pre_rate;

        let seeds = &[
            b"lauchpad",
            &self.launchpad_account.dever_address.to_bytes()[..],
            &[self.launchpad_account.bump]
        ];

        //send token to from pool to user 
        transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.vault_launchpad.to_account_info(),
                    to: self.claimer_token_account.to_account_info(),
                    authority: self.launchpad_account.to_account_info(),
                },
                &[&seeds[..]]
            ),
            amount
        )?;
        Ok(())
    }

}

