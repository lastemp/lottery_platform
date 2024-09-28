//! lottery_platform program entrypoint

pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("AA7crqzoJE15cTM8aQGrPP7FLHsCc8i2YcFkQNNwhShA");

#[program]
pub mod lottery_platform {
    use super::*;

    // admin instructions
    pub fn init(ctx: Context<Init>) -> Result<()> {
        instructions::init(ctx)
    }

    pub fn register_lottery_game(
        ctx: Context<RegisterLotteryGame>,
        params: RegisterLotteryGameParams,
    ) -> Result<()> {
        instructions::register_lottery_game(ctx, &params)
    }

    // public instructions
    pub fn register_participant(
        ctx: Context<RegisterParticipant>,
        params: RegisterParticipantParams,
    ) -> Result<()> {
        instructions::register_participant(ctx, &params)
    }

    pub fn buy_lottery_ticket(
        ctx: Context<BuyLotteryTicket>,
        params: BuyLotteryTicketParams,
    ) -> Result<()> {
        instructions::buy_lottery_ticket(ctx, &params)
    }

    pub fn create_token(ctx: Context<CreateToken>, params: CreateTokenParams) -> Result<()> {
        instructions::create_token(ctx, &params)
    }

    pub fn transfer_token(ctx: Context<TransferToken>, params: TransferTokenParams) -> Result<()> {
        instructions::transfer_token(ctx, &params)
    }
}
