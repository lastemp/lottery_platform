//! RegisterLotteryGame instruction handler

use {
    crate::{
        error::LotteryGameError,
        state::{
            configs::LotteryGameConfigs, deposit_base::DepositBase, lottery_game::LotteryGame,
            lottery_operator::LotteryOperator,
        },
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: RegisterLotteryGameParams)]
pub struct RegisterLotteryGame<'info> {
    #[account(
        mut, constraint = lottery_game_configs.is_initialized @ LotteryGameError::AccountNotInitialized
    )]
    pub lottery_game_configs: Account<'info, LotteryGameConfigs>,
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + LotteryGame::INIT_SPACE,
        constraint = !lottery_game.is_initialized @ LotteryGameError::AccountAlreadyInitialized,
        seeds = [b"lottery-game", owner.key().as_ref()],
        bump
    )]
    pub lottery_game: Account<'info, LotteryGame>,
    #[account(init, payer = owner, space = 8 + DepositBase::INIT_SPACE,
        constraint = !deposit_account.is_initialized @ LotteryGameError::AccountAlreadyInitialized
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
pub struct RegisterLotteryGameParams {
    operator: LotteryOperator,        // lottery operator details
    country: String,                  // home country where lottery game is implemented
    lottery_game_name: String,        // lottery game name
    lottery_winning_percentage: u32,  // lottery winning percentage
    unit_cost_of_lottery_ticket: u32, // unit cost of lottery ticket
    decimals: u8,                     // decimals for the token mint
    value_date: String,               // value date of lottery draw
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

pub fn register_lottery_game(
    ctx: Context<RegisterLotteryGame>,
    params: &RegisterLotteryGameParams,
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

    if params.lottery_winning_percentage > 0 {
    } else {
        return Err(LotteryGameError::InvalidAmount.into());
    }

    if params.unit_cost_of_lottery_ticket > 0 {
    } else {
        return Err(LotteryGameError::InvalidAmount.into());
    }

    if params.value_date.as_bytes().len() > 0 && params.value_date.as_bytes().len() <= DATE_LENGTH {
    } else {
        return Err(LotteryGameError::InvalidValueDateLength.into());
    }

    if params.decimals == 0 {
        return Err(LotteryGameError::InvalidNumeric.into());
    }

    let deposit_account = &mut ctx.accounts.deposit_account;
    let lottery_game = &mut ctx.accounts.lottery_game;
    let lottery_game_configs = &mut ctx.accounts.lottery_game_configs;

    // deposit account
    // * - means dereferencing
    deposit_account.owner = *ctx.accounts.owner.key;
    deposit_account.admin_auth_bump = ctx.bumps.pda_auth;
    deposit_account.admin_treasury_vault_bump = Some(ctx.bumps.treasury_vault);
    deposit_account.is_initialized = true;

    // lottery_game
    lottery_game.owner = *ctx.accounts.owner.key;
    lottery_game.operator.operator = params.operator.operator.to_string();
    lottery_game.country = params.country.to_string();
    lottery_game.lottery_game_name = params.lottery_game_name.to_string();
    lottery_game.is_initialized = true;
    lottery_game.is_closed = false;
    lottery_game.lottery_winning_percentage = params.lottery_winning_percentage;
    lottery_game.unit_cost_of_lottery_ticket = params.unit_cost_of_lottery_ticket;
    lottery_game.decimals = params.decimals;
    lottery_game.value_date = params.value_date.to_string();

    let lottery_operator = LotteryOperator {
        operator: params.operator.operator.to_string(),
    };

    // lottery_game_configs
    lottery_game_configs.operators.push(lottery_operator);

    Ok(())
}
