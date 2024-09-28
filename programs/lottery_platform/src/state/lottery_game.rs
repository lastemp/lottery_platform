use crate::state::lottery_operator::LotteryOperator;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LotteryGame {
    pub owner: Pubkey, // publickey of the lottery game admin
    pub operator: LotteryOperator,
    #[max_len(3)]
    pub country: String, // home country where lottery game is auctioned
    #[max_len(20)]
    pub lottery_game_name: String, // lottery game name
    pub total_amounts_raised: u32, // total amounts raised from sale of tickets
    pub total_amounts_withdrawn: u32, // total amounts withdrawn by participants i.e winners
    pub total_available_funds: u32, // total available funds equal to the lottery game tickets bought by participants
    pub is_initialized: bool,       // is lottery game initiated
    #[max_len(10)]
    pub participants: Vec<Pubkey>, // list of the participants
    pub lottery_winning_percentage: u32, // lottery winning percentage
    pub unit_cost_of_lottery_ticket: u32, // unit cost of lottery ticket
    pub decimals: u8,               // decimals for the token mint
    #[max_len(20)]
    pub value_date: String, // value date of lottery draw
    pub is_matured: bool,           // is lottery game matured
}
