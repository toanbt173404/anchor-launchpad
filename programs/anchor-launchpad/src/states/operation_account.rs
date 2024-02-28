use anchor_lang::prelude:: *;

pub const LIST_UER_REF_SIZE_USIZE: usize = 50;
pub const WHITELIST_MINT_SIZE_USIZE: usize = 100;

#[account()]
#[derive(Debug)]
pub struct OperationState {
    pub bump: u8,
    pub whitelist_addresses: [Pubkey; WHITELIST_MINT_SIZE_USIZE],
    pub list_users_ref: [Pubkey; LIST_UER_REF_SIZE_USIZE],
}

impl OperationState {
    pub const LEN: usize = 8 + 1 + 32 * LIST_UER_REF_SIZE_USIZE + 32 * WHITELIST_MINT_SIZE_USIZE;
}

