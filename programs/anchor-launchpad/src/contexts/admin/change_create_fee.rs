use crate::ConfigAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(add_fee_un_con: Pubkey, new_fee: u64, creation_fee_option_sol: u8)]
pub struct ChangeCreateFee<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub config_account: Account<'info, ConfigAccount>,
    pub system_program: Program<'info, System>,
}
impl<'info> ChangeCreateFee<'info> {
    pub fn change_create_fee(
        &mut self,
        add_fee_un_con: Pubkey,
        new_fee: u64,
        creation_fee_option_sol: u8,
    ) -> Result<()> {
        self.config_account.creation_fee_option_sol = creation_fee_option_sol;
        self.config_account.add_fee_un_con = add_fee_un_con;
        self.config_account.creation_fee = new_fee;
        Ok(())
    }
}
