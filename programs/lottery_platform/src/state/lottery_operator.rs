use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct LotteryOperator {
    #[max_len(30)]
    pub operator: String, // company operating the lottery
}
