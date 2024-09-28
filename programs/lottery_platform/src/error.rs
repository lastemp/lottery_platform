use anchor_lang::prelude::*;

#[error_code]
pub enum LotteryGameError {
    // Lottery game
    #[msg("Invalid issuer length")]
    InvalidIssuerLength,
    #[msg("Invalid issuer no length")]
    InvalidIssuerNoLength,
    #[msg("Invalid type of bond")]
    InvalidTypeOfBond,
    #[msg("Invalid bond tenor")]
    InvalidBondTenor,
    #[msg("Invalid bond coupon rate")]
    InvalidBondCouponRate,
    #[msg("Invalid value date length")]
    InvalidValueDateLength,
    #[msg("Invalid redemption date length")]
    InvalidValueRedemptionLength,
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Available balance should match tranfer amount.")]
    MismatchedAmount,
    #[msg("Invalid numeric value.")]
    InvalidNumeric,
    #[msg("Invalid minimum bid amount.")]
    InvalidMinimumBidAmount,
    #[msg("Invalid bond maturity status.")]
    InvalidBondMaturityStatus,

    //
    #[msg("Invalid country length")]
    InvalidCountryLength,

    // Arithmetic
    #[msg("Arithmetic operation failed.")]
    InvalidArithmeticOperation,

    // participant
    #[msg("Invalid full names length")]
    InvalidFullNamesLength,
    #[msg("Participant has no active status.")]
    InvalidParticipantStatus,
    #[msg("Insufficient funds.")]
    InsufficientFunds,

    // account
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}
