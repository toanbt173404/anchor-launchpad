use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Only authority can call this function")]
    Unauthorized,
    #[msg("The provided time must be greater than the start time and less than the end time, or equal to 0.")]
    InvalidTimePublicWls,
    #[msg("The new affiliate value must be greater than the current value.")]
    InvalidAffiliateValue,
    #[msg("Presale is currently active.")]
    SaleIsActive,
    #[msg("Presale has not started yet.")]
    PreSaleNotStarted,
    #[msg("Presale has ended.")]
    PreSaleEnded,
    #[msg("It's not time to claim")]
    PreSaleNotCompleted,
    #[msg("Presale not completed")]
    NotClaimTime,
    #[msg("The whitelist is full.")]
    WhitelistFull,
    #[msg("Token not currency")]
    TokenNotCurrency,
    #[msg("Min amount")]
    MinAmount,
    #[msg("Max amount")]
    MaxAmount,
    #[msg("Presale has reached hard cap")]
    HardCapReached,
    #[msg("Only whitelisted addresses allowed")]
    OnlyWhitelist,
    #[msg("Overflow")]
    Overflow,
    #[msg("Not enough lamports to buy the requested amount of tokens.")]
    InsufficientLamportsToBuyTokens,
    #[msg("Tokens already claimed")]
    ClaimedToken,
    #[msg("It's not time to withdrawFunds")]
    NotWithdrawfundsTime
}
