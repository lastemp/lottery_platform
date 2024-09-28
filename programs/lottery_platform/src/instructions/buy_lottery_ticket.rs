//! BuyLotteryTicket instruction handler

use {
    crate::{
        error::LotteryGameError,
        state::{lottery_game::LotteryGame, participant::Participant},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(params: BuyLotteryTicketParams)]
pub struct BuyLotteryTicket<'info> {
    #[account(mut,
        constraint = lottery_game.is_initialized @ LotteryGameError::AccountNotInitialized
    )]
    pub lottery_game: Account<'info, LotteryGame>,
    #[account(mut,has_one = owner,
        constraint = participant.active @ LotteryGameError::InvalidParticipantStatus
    )]
    pub participant: Account<'info, Participant>,
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BuyLotteryTicketParams {
    pub amount: u32,
}

pub fn buy_lottery_ticket(
    ctx: Context<BuyLotteryTicket>,
    params: &BuyLotteryTicketParams,
) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(LotteryGameError::InvalidAmount.into());
    }

    let sender = &ctx.accounts.owner;
    let sender_tokens = &ctx.accounts.sender_tokens;
    let recipient_tokens = &ctx.accounts.recipient_tokens;
    let token_program = &ctx.accounts.token_program;
    let lottery_game = &mut ctx.accounts.lottery_game;
    let participant = &mut ctx.accounts.participant;
    let unit_cost_of_lottery_game: u32 = lottery_game.unit_cost_of_lottery_game;
    let total_amounts_accepted = lottery_game.total_amounts_accepted;
    let total_available_funds = lottery_game.total_available_funds;
    let minimum_bid_amount = lottery_game.minimum_bid_amount;
    let total_units_lottery_game: u32 = participant.total_units_lottery_game;
    let available_funds: u32 = participant.available_funds;
    let decimals = lottery_game.decimals as u64;
    let _amount = params.amount;

    if _amount < minimum_bid_amount {
        return Err(LotteryGameError::InvalidMinimumBidAmount.into());
    }

    // Get unit_lottery_game from the product of unit_cost_of_lottery_game and _amount
    let unit_lottery_game = unit_cost_of_lottery_game
        .checked_mul(_amount)
        .ok_or(LotteryGameError::InvalidArithmeticOperation)?;

    // Increment total_units_lottery_game with new unit_lottery_game
    participant.total_units_lottery_game = total_units_lottery_game
        .checked_add(unit_lottery_game)
        .ok_or(LotteryGameError::InvalidArithmeticOperation)?;

    // Increment available_funds with new _amount
    participant.available_funds = available_funds
        .checked_add(_amount)
        .ok_or(LotteryGameError::InvalidArithmeticOperation)?;

    // Increment total_amounts_accepted with new _amount
    lottery_game.total_amounts_accepted = total_amounts_accepted
        .checked_add(_amount)
        .ok_or(LotteryGameError::InvalidArithmeticOperation)?;

    // Increment total_available_funds with new _amount
    lottery_game.total_available_funds = total_available_funds
        .checked_add(_amount)
        .ok_or(LotteryGameError::InvalidArithmeticOperation)?;

    let base: u32 = 10;
    let exponent = lottery_game.decimals as u32;

    // lets get the amount in decimal format
    // 10 ** 9 * 3(base 10, 9 decimals, 3 amount), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    let result = (base).pow(exponent);
    let _amount = (_amount as u64)
        .checked_mul(result as u64)
        .ok_or(LotteryGameError::InvalidArithmeticOperation)?;

    lottery_game.participants.push(*sender.key);

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: sender_tokens.to_account_info(),
                to: recipient_tokens.to_account_info(),
                authority: sender.to_account_info(),
            },
        ),
        _amount,
    )?;

    Ok(())
}
