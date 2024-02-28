use anchor_lang::prelude::*;
mod contexts;
mod error;
mod helper;
mod states;

use contexts::*;
use error::ErrorCode;
use helper::*;
use states::*;

declare_id!("4Vefj73cKWKUa4VD2nrMfXMY5W3KVJMHJ9q1nfG4TJKT");

#[program]
pub mod anchor_launchpad {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, add_fee_un_on: Pubkey) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps, add_fee_un_on)?;
        Ok(())
    }

    pub fn change_create_fee(
        ctx: Context<ChangeCreateFee>,
        add_fee_un_on: Pubkey,
        new_fee: u64,
        creation_fee_option_sol: u8,
    ) -> Result<()> {
        ctx.accounts
            .change_create_fee(add_fee_un_on, new_fee, creation_fee_option_sol)?;
        Ok(())
    }

    pub fn create_launchpad(
        ctx: Context<CreatLaunchpad>,
        params1: LaunchpadParamsStep1,
        params2: LaunchpadParamsStep2,
        params3: LaunchpadParamsStep3,
    ) -> Result<()> {
        ctx.accounts
            .create_launchpad_data(&ctx.bumps, params1.clone(), params2, params3)?;

        //send token to pool
        ctx.accounts.send_tokens_to_pool(params1.total_token_to)?;
        Ok(())
    }

    pub fn add_to_whitelist(
        ctx: Context<AddToWhitelist>,
        pubkeys_to_add: Vec<Pubkey>,
    ) -> Result<()> {
        let whitelist_account = &mut ctx.accounts.whitelist_account;
        whitelist_account.add_pubkeys(pubkeys_to_add)
    }

    pub fn remove_from_whitelist(
        ctx: Context<RemoveFromWhitelist>,
        pubkeys_to_remove: Vec<Pubkey>,
    ) -> Result<()> {
        let whitelist_account = &mut ctx.accounts.whitelist_account;
        whitelist_account.remove_pubkeys(pubkeys_to_remove)
    }

    pub fn update_time_public_wls(
        ctx: Context<UpdateTimePublicWls>,
        time_public: u64,
    ) -> Result<()> {
        let launchpad_account = &mut ctx.accounts.launchpad_account;
        require!(
            (time_public > launchpad_account.launchpad_params_step_2.start_time
                || time_public == 0)
                && (time_public < launchpad_account.launchpad_params_step_2.end_time
                    || time_public == 0),
            ErrorCode::InvalidTimePublicWls
        );

        if launchpad_account.launchpad_params_step_2.whitelist == 0 {
            launchpad_account.launchpad_params_step_2.whitelist = 1;
        }

        launchpad_account.time_public_wls = time_public;

        Ok(())
    }

    pub fn update_whitelist_status(
        ctx: Context<UpdateWhitelistStatus>,
        new_whitelist_status: u64,
    ) -> Result<()> {
        let launchpad_account = &mut ctx.accounts.launchpad_account;
        launchpad_account.launchpad_params_step_2.whitelist = new_whitelist_status;
        Ok(())
    }

    pub fn change_info_social_network(
        ctx: Context<ChangeInfoSocialNetwork>,
        new_social_info: LaunchpadParamsStep3,
    ) -> Result<()> {
        let launchpad_account = &mut ctx.accounts.launchpad_account;
        launchpad_account.launchpad_params_step_3 = new_social_info;
        Ok(())
    }

    pub fn update_affiliate(ctx: Context<UpdateAffiliate>, new_affiliate: u8) -> Result<()> {
        let launchpad_account = &mut ctx.accounts.launchpad_account;
        require!(
            new_affiliate > launchpad_account.launchpad_params_step_1.affiliate,
            ErrorCode::InvalidAffiliateValue
        );
        launchpad_account.launchpad_params_step_1.affiliate = new_affiliate;

        Ok(())
    }

    pub fn contribute(ctx: Context<Contribute>, amount: u64, mint_addr: Pubkey) -> Result<()> {
        let contributor_lamports = **ctx
            .accounts
            .contributor
            .to_account_info()
            .try_borrow_lamports()?;

        require!(
            contributor_lamports >= amount,
            ErrorCode::InsufficientLamportsToBuyTokens
        );
        let contributor = &mut ctx.accounts.contributor;
        let launchpad_account = &mut ctx.accounts.launchpad_account;

        send_lamports(
            contributor.to_account_info(),
            launchpad_account.to_account_info(),
            amount,
        )?;

        ctx.accounts.contribute(amount, mint_addr)?;
        Ok(())
    }

    pub fn claim_token(ctx: Context<ClaimTokens>) -> Result<()> {
        ctx.accounts.claimTokens()?;
        Ok(())
    }
}
