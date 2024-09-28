use crate::state::bond_issuer::BondIssuer;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LotteryGame {
    pub owner: Pubkey, // publickey of the lottery game admin
    pub issuer: BondIssuer,
    #[max_len(3)]
    pub country: String, // home country where lottery game is auctioned
    #[max_len(20)]
    pub issue_no: String, // issue no of bond
    pub type_of_bond: u8, // type of bond i.e Fixed coupon Treasury bonds, Infrastructure bonds
    pub tenor: u8,        // maturity period i.e between 2-30 years
    pub coupon_rate: u8,  // coupon rate (%)
    pub total_amounts_offered: u32, // total amounts offered for the given bond
    pub total_amounts_accepted: u32, // total amounts accepted from bondholders (participants)
    pub total_available_funds: u32, // total available funds equal to the lottery game tickets owned by participants
    pub minimum_bid_amount: u32,    // minimum bid amount
    pub is_initialized: bool,       // is lottery game initiated
    #[max_len(10)]
    pub participants: Vec<Pubkey>, // list of the participants
    pub unit_cost_of_lottery_game: u32, // unit cost of lottery game
    pub decimals: u8,               // decimals for the token mint
    #[max_len(20)]
    pub value_date: String, // value date of bond
    #[max_len(20)]
    pub redemption_date: String, // redemption date of bond
    pub is_matured: bool,           // is lottery game matured
}
