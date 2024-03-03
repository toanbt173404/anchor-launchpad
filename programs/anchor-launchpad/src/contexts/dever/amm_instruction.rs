#![allow(clippy::too_many_arguments)]

use anchor_spl::token::spl_token;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitializeInstruction {
    pub nonce: u8,
    pub open_time: u64,
}

impl InitializeInstruction {
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(9);
        buf.push(self.nonce);
        buf.extend_from_slice(&self.open_time.to_le_bytes());
        buf
    }
}

pub fn initialize_pool_amm(
    program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    lp_mint_address: &Pubkey,
    coin_mint_address: &Pubkey,
    pc_mint_address: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    pool_withdraw_queue: &Pubkey,
    pool_target_orders_account: &Pubkey,
    pool_lp_token_account: &Pubkey,
    pool_temp_lp_token_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    user_wallet: &Pubkey,
    nonce: u8,
    open_time: u64,
) -> Result<Instruction, ProgramError> {
    let init_data = InitializeInstruction { nonce, open_time };

    let data = init_data.pack();
    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*lp_mint_address, false),
        AccountMeta::new_readonly(*coin_mint_address, false),
        AccountMeta::new_readonly(*pc_mint_address, false),
        AccountMeta::new_readonly(*pool_coin_token_account, false),
        AccountMeta::new_readonly(*pool_pc_token_account, false),
        AccountMeta::new(*pool_withdraw_queue, false),
        AccountMeta::new(*pool_target_orders_account, false),
        AccountMeta::new(*pool_lp_token_account, false),
        AccountMeta::new_readonly(*pool_temp_lp_token_account, false),
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new_readonly(*serum_market, false),
        AccountMeta::new(*user_wallet, true),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}