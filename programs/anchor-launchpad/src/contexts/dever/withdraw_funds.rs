use anchor_lang::prelude::*;
use anchor_spl::token::{burn, spl_token, Burn, TokenAccount, Transfer, transfer};
use crate::error::ErrorCode;

use crate::{check_and_update_sale, initialize_pool_amm, send_lamports, LaunchpadAccount};


#[derive(Accounts, Clone)]
pub struct InitializePool<'info> {
    /// CHECK: Safe. The new amm Account to be create, a PDA create with seed = [program_id,serum_market_id, b"amm_associated_seed"]
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// CHECK: Safe. Amm authority, a PDA create with seed = [b"amm authority"]
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe. Amm open_orders Account, a PDA create with seed = [program_id,serum_market_id, b"open_order_associated_seed"]
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe. Pool lp mint account. Must be empty, owned by $authority.
    #[account(mut)]
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: Safe. Coin mint account
    pub coin_mint: AccountInfo<'info>,
    /// CHECK: Safe. Pc mint account
    pub pc_mint: AccountInfo<'info>,
    /// CHECK: Safe. Pool_token_coin Account. Must be non zero, owned by $authority
    pub pool_coin_token_account: AccountInfo<'info>,
    /// CHECK: Safe. Pool_token_pc Account. Must be non zero, owned by $authority.
    pub pool_pc_token_account: AccountInfo<'info>,
    /// CHECK: Safe. Withdraw queue Account. To save withdraw dest_coin & dest_pc account with must cancle orders.
    #[account(mut)]
    pub pool_withdraw_queue: AccountInfo<'info>,
    /// CHECK: Safe. Pool target orders account
    #[account(mut)]
    pub pool_target_orders_account: AccountInfo<'info>,
    /// CHECK: Safe. Token_dest_lp Account. To deposit the initial pool token supply, user_wallet is the owner.
    #[account(mut)]
    pub pool_lp_token_account: AccountInfo<'info>,
    /// CHECK: Safe. Token_temp_lp Account. To save withdraw lp with must cancle orders as temp to transfer later.
    pub pool_temp_lp_token_account: AccountInfo<'info>,
    /// CHECK: Safe. Serum dex program.
    pub serum_program: AccountInfo<'info>,
    /// CHECK: Safe. Serum market Account. serum_dex program is the owner.
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Safe. The user wallet create the pool
    #[account(signer)]
    pub user_wallet: AccountInfo<'info>,
    /// CHECK: Safe. The spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
    /// CHECK: Safe. System program
    pub system_program: Program<'info, System>,
    /// CHECK: Safe. Rent program
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts, Clone)]
pub struct WithdrawFunds<'info> {

    pub launchpad_account: Box<Account<'info, LaunchpadAccount>>,
    #[account(
    )]
    pub vault_launchpad: Box<Account<'info, TokenAccount>>,
    /// CHECK: Safe
    pub amm_program: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        seeds = [
            amm_program.key.as_ref(),
            serum_market.key.as_ref(),
            b"amm_associated_seed"], 
        bump,
        seeds::program = amm_program.key
    )]
    pub amm: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        seeds = [b"amm authority"], 
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        seeds = [
            amm_program.key.as_ref(),
            serum_market.key.as_ref(),
            b"open_order_associated_seed"], 
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: Safe
    pub coin_mint: AccountInfo<'info>,
    /// CHECK: Safe
    pub pc_mint: AccountInfo<'info>,
    /// CHECK: Safe
    pub pool_coin_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub pool_pc_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_withdraw_queue: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_target_orders_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_lp_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub pool_temp_lp_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub serum_program: AccountInfo<'info>,
    /// CHECK: Safe
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(signer)]
    pub user_wallet: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'a, 'b, 'c, 'info> From<&mut WithdrawFunds<'info>>
    for CpiContext<'a, 'b, 'c, 'info, InitializePool<'info>>
{
    fn from(
        accounts: &mut WithdrawFunds<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, InitializePool<'info>> {
        let cpi_accounts = InitializePool {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            lp_mint: accounts.lp_mint.clone(),
            coin_mint: accounts.coin_mint.clone(),
            pc_mint: accounts.pc_mint.clone(),
            pool_coin_token_account: accounts.pool_coin_token_account.clone(),
            pool_pc_token_account: accounts.pool_pc_token_account.clone(),
            pool_withdraw_queue: accounts.pool_withdraw_queue.clone(),
            pool_target_orders_account: accounts.pool_target_orders_account.clone(),
            pool_lp_token_account: accounts.pool_lp_token_account.clone(),
            pool_temp_lp_token_account: accounts.pool_temp_lp_token_account.clone(),
            serum_program: accounts.serum_program.clone(),
            serum_market: accounts.serum_market.clone(),
            user_wallet: accounts.user_wallet.to_account_info().clone(),
            spl_token_program: accounts.spl_token_program.clone(),
            system_program: accounts.system_program.clone(),
            rent: accounts.rent.clone(),
        };
        let cpi_program = accounts.amm_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn initialize_pool_raydium<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitializePool<'info>>,
    nonce: u8,
    open_time: u64,
) -> Result<()> {
    let ix = initialize_pool_amm(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.lp_mint.key,
        ctx.accounts.coin_mint.key,
        ctx.accounts.pc_mint.key,
        ctx.accounts.pool_coin_token_account.key,
        ctx.accounts.pool_pc_token_account.key,
        ctx.accounts.pool_withdraw_queue.key,
        ctx.accounts.pool_target_orders_account.key,
        ctx.accounts.pool_lp_token_account.key,
        ctx.accounts.pool_temp_lp_token_account.key,
        ctx.accounts.serum_program.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.user_wallet.key,
        nonce,
        open_time,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

pub fn withdraw_funds(ctx: Context<WithdrawFunds>, nonce: u8, open_time: u64) -> Result<()> {

    let dever_pubkey =  ctx.accounts.user_wallet.to_account_info().clone();

    let launchpad_account_pubkey = ctx.accounts.launchpad_account.to_account_info().clone();
    let launchpad_account = &mut ctx.accounts.launchpad_account;

    let token_amount = ctx.accounts.vault_launchpad.amount;

    let seeds: &[&[u8]; 3] = &[
        b"lauchpad",
        &dever_pubkey.key().to_bytes()[..],
        &[launchpad_account.clone().bump]
    ];

    check_and_update_sale(launchpad_account)?;

    require!(launchpad_account.is_sale_active == 2 || launchpad_account.is_sale_active == 3, ErrorCode::NotWithdrawfundsTime);

    if launchpad_account.is_sale_active == 2 {
        launchpad_account.is_sale_active = 4;
        let percent_refund: u8 = launchpad_account.creation_fee_option_sol;
        let mut total_arm = launchpad_account.total_buyed * percent_refund as u64 / 100;

        if launchpad_account.launchpad_params_step_1.affiliate > 0 {
            total_arm  = total_arm - (launchpad_account.total_buyed * launchpad_account.launchpad_params_step_1.affiliate as u64 / 100);

        }

        let total_ama_add_liquidity: u64;

        if launchpad_account.launchpad_params_step_1.listing_option == 0 {
            total_ama_add_liquidity = total_arm * launchpad_account.launchpad_params_step_2.liquidity_rate / 100;

            total_arm -= total_ama_add_liquidity;
            //let listing_rate_mul = total_ama_add_liquidity * launchpad_account.launchpad_params_step_2.listing_rate * 10u64.pow(launchpad_account.launchpad_params_step_1.decimals as u32);

            let mut amount_refund = launchpad_account.total_buyed * launchpad_account.launchpad_params_step_2.pre_rate;

            amount_refund = amount_refund *  10u64.pow(launchpad_account.launchpad_params_step_1.decimals as u32) / 10u64.pow(9);


            let type_refund = launchpad_account.launchpad_params_step_2.type_refund;
            if token_amount - amount_refund > 0 {


                if type_refund == 0 {
                    let cpi_accounts = Burn {
                        mint: ctx.accounts.coin_mint.to_account_info(),
                        from: ctx.accounts.vault_launchpad.to_account_info(),
                        authority: ctx.accounts.vault_launchpad.to_account_info(),
                    };
                    burn(
                        CpiContext::new_with_signer(ctx.accounts.spl_token_program.to_account_info(), cpi_accounts, &[&seeds[..]]),
                        token_amount - amount_refund, )?;
                } else {
                    transfer(
                        CpiContext::new_with_signer(
                            ctx.accounts.spl_token_program.to_account_info(),
                            Transfer {
                                from: ctx.accounts.vault_launchpad.to_account_info(),
                                to: ctx.accounts.pool_coin_token_account.to_account_info(),
                                authority: ctx.accounts.launchpad_account.to_account_info()
                            },
                            &[&seeds[..]]
                        ),
                        token_amount - amount_refund
                    )?;
                }
            }
            initialize_pool_raydium(ctx.accounts.into(), nonce, open_time)?;
            send_lamports(launchpad_account_pubkey, dever_pubkey, total_arm)?;
        }

    } else {
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.spl_token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_launchpad.to_account_info(),
                    to: ctx.accounts.pool_coin_token_account.to_account_info(),
                    authority: ctx.accounts.launchpad_account.to_account_info()
                },
                &[&seeds[..]]
            ),
            token_amount
        )?;
    }
    Ok(())
    
}
