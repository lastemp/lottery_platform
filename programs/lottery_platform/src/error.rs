use anchor_lang::prelude::*;

#[error_code]
pub enum LotteryGameError {
    // Lottery game
    #[msg("Invalid operator length")]
    InvalidOperatorLength,
    #[msg("Invalid lottery game name length")]
    InvalidLotteryGameNameLength,
    #[msg("Invalid value date length")]
    InvalidValueDateLength,
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Available balance should match tranfer amount.")]
    MismatchedAmount,
    #[msg("Invalid numeric value.")]
    InvalidNumeric,
    #[msg("Invalid lottery ticket amount.")]
    InvalidLotteryTicketAmount,
    #[msg("Lottery game is closed.")]
    LotteryGameClosed,
    #[msg("Invalid lottery game winner.")]
    InvalidLotteryGameWinner,

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
    #[msg("Participant(s) missing.")]
    InvalidParticipants,

    // account
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}
