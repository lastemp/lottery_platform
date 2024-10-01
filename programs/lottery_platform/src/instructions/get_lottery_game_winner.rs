//! GetLotteryGameWinner instruction handler

use {
    crate::{
        error::LotteryGameError,
        state::{
            configs::LotteryGameConfigs, deposit_base::DepositBase, lottery_game::LotteryGame,
            lottery_operator::LotteryOperator,
        },
    },
    anchor_lang::prelude::*,
    solana_program::sysvar::clock::Clock,
};

#[derive(Accounts)]
#[instruction(params: GetLotteryGameWinnerParams)]
pub struct GetLotteryGameWinner<'info> {
    #[account(
        mut, constraint = lottery_game_configs.is_initialized @ LotteryGameError::AccountNotInitialized
    )]
    pub lottery_game_configs: Account<'info, LotteryGameConfigs>,
    #[account(mut,
        constraint = lottery_game.is_initialized @ LotteryGameError::AccountNotInitialized,
        constraint = !lottery_game.is_closed @ LotteryGameError::LotteryGameClosed
    )]
    pub lottery_game: Account<'info, LotteryGame>,
    #[account(mut,has_one = owner,
        constraint = deposit_account.is_initialized @ LotteryGameError::AccountNotInitialized
    )]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"treasury-vault", pda_auth.key().as_ref()], bump)]
    pub treasury_vault: SystemAccount<'info>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct GetLotteryGameWinnerParams {
    operator: LotteryOperator, // lottery operator details
    country: String,           // home country where lottery game is implemented
    lottery_game_name: String, // lottery game name
}

// operator length
const OPERATOR_LENGTH: usize = 30;
// lottery_game_name length
const LOTTERY_GAME_NAME: usize = 20;
// tenor length
const TENOR_LENGTH: u8 = 2;
const TENOR_LENGTH_2: u8 = 30;
// date length
const DATE_LENGTH: usize = 20;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn get_lottery_game_winner(
    ctx: Context<GetLotteryGameWinner>,
    params: &GetLotteryGameWinnerParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.operator.operator.as_bytes().len() > 0
        && params.operator.operator.as_bytes().len() <= OPERATOR_LENGTH
    {
    } else {
        return Err(LotteryGameError::InvalidOperatorLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(LotteryGameError::InvalidCountryLength.into());
    }

    if params.lottery_game_name.as_bytes().len() > 0
        && params.lottery_game_name.as_bytes().len() <= LOTTERY_GAME_NAME
    {
    } else {
        return Err(LotteryGameError::InvalidLotteryGameNameLength.into());
    }
    /*
    if params.value_date.as_bytes().len() > 0 && params.value_date.as_bytes().len() <= DATE_LENGTH {
    } else {
        return Err(LotteryGameError::InvalidValueDateLength.into());
    }
    */

    let deposit_account = &mut ctx.accounts.deposit_account;
    let lottery_game = &mut ctx.accounts.lottery_game;
    let lottery_game_configs = &mut ctx.accounts.lottery_game_configs;

    if lottery_game.lottery_winning_percentage > 0 {
    } else {
        return Err(LotteryGameError::InvalidAmount.into());
    }

    if lottery_game.unit_cost_of_lottery_ticket > 0 {
    } else {
        return Err(LotteryGameError::InvalidAmount.into());
    }

    if lottery_game.decimals == 0 {
        return Err(LotteryGameError::InvalidNumeric.into());
    }

    if lottery_game.participants.is_empty() {
        return Err(LotteryGameError::InvalidParticipants.into());
    }

    // Get current slot (pseudo-randomness based on slot time)
    let clock = Clock::get().unwrap();
    let slot = clock.slot;

    // Use slot number to derive a "random" index
    let random_index: usize = (slot as usize) % lottery_game.participants.len();
    // lottery game winner
    lottery_game.lottery_game_winner_index = random_index as u32;
    let lottery_game_winner: Pubkey = lottery_game.participants[random_index];
    lottery_game.lottery_game_winner = lottery_game_winner;
    lottery_game.is_closed = true;

    Ok(())
}
