// admin instructions
pub mod init;
pub mod register_lottery_game;

// public instructions
pub mod buy_lottery_ticket;
pub mod create_token;
pub mod get_lottery_game_winner;
pub mod register_participant;
pub mod transfer_token;
pub mod withdraw_lottery_game_winnings;

// bring everything in scope
pub use {
    buy_lottery_ticket::*, create_token::*, get_lottery_game_winner::*, init::*,
    register_lottery_game::*, register_participant::*, transfer_token::*,
    withdraw_lottery_game_winnings::*,
};
