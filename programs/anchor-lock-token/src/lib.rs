use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer, transfer},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod lock_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,amount: u64, duration: i64) -> Result<()> {
        let lock_account = &mut ctx.accounts.lock_account;
        lock_account.owner = *ctx.accounts.owner.key;
        lock_account.unlock_time = Clock::get()?.unix_timestamp + duration;
        ctx.accounts.send_tokens_to_lock_account(amount)?;
        Ok(())
    }

    pub fn unlock(ctx: Context<Unlock>) -> Result<()> {
        let clock = Clock::get()?;
        let lock_account = &mut ctx.accounts.lock_account;

        require!(
            clock.unix_timestamp > lock_account.unlock_time,
            ErrorCode::LockStillActive
        );
        let amount = lock_account.amount.clone();
        lock_account.amount = 0;
        ctx.accounts.transfer_lock_token_back_to_user(amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub lock_mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed, 
        payer = owner, 
        space = 8 + 40,        
        seeds = [b"lock".as_ref(), &owner.key().as_ref()],
        bump
    )]
    pub lock_account: Account<'info, LockAccount>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = lock_mint,
        associated_token::authority = lock_account
    )]
    pub lock_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn send_tokens_to_lock_account(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.owner_token_account.to_account_info(),
            to: self.lock_token_account.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        transfer(
            CpiContext::new(self.token_program.to_account_info(), cpi_accounts),
            amount.into()
        )
    }
}

#[derive(Accounts)]
pub struct Unlock<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner)]
    pub lock_account: Account<'info, LockAccount>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lock_token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LockAccount {
    pub bump: u8,
    pub owner: Pubkey,
    pub unlock_time: i64,
    pub amount: u64,
}

impl<'info> Unlock<'info> {
    pub fn transfer_lock_token_back_to_user(&mut self, amount: u64) -> Result<()> {
        let seeds: &[&[u8]; 3] = &[
            b"lock",
            &self.lock_account.owner.to_bytes()[..],
            &[self.lock_account.bump]
        ];

        transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.lock_token_account.to_account_info(),
                    to: self.owner_token_account.to_account_info(),
                    authority: self.lock_account.to_account_info(),
                },
                &[&seeds[..]]
            ),
            amount
        )
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("The lock is still active and cannot be unlocked yet.")]
    LockStillActive,
}
