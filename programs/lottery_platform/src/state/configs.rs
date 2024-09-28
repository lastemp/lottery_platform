use crate::state::bond_issuer::BondIssuer;
use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct LotteryGameConfigs {
    #[max_len(5)]
    pub issuers: Vec<BondIssuer>,
    pub is_initialized: bool,
}
