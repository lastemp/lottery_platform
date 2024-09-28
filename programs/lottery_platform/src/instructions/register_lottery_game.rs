//! RegisterLotteryGame instruction handler

use {
    crate::{
        error::LotteryGameError,
        state::{
            bond_issuer::BondIssuer, configs::LotteryGameConfigs, deposit_base::DepositBase,
            lottery_game::LotteryGame,
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
    issuer: BondIssuer,             // bond issuer details
    country: String,                // home country where lottery game is issued
    issue_no: String,               // issue no of bond
    type_of_bond: u8, // type of bond i.e Fixed coupon Treasury bonds, Infrastructure bonds
    tenor: u8,        // maturity period i.e between 2-30 years
    coupon_rate: u8,  // coupon rate (%)
    total_amounts_offered: u32, // total amounts offered for the given bond
    minimum_bid_amount: u32, // minimum bid amount
    unit_cost_of_lottery_game: u32, // unit cost of lottery game
    decimals: u8,     // decimals for the token mint
    value_date: String, // value date of bond
    redemption_date: String, // redemption date of bond
}

// issuer length
const ISSUER_LENGTH: usize = 30;
// issuer NO length
const ISSUER_NO_LENGTH: usize = 20;
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
    if params.issuer.issuer.as_bytes().len() > 0
        && params.issuer.issuer.as_bytes().len() <= ISSUER_LENGTH
    {
    } else {
        return Err(LotteryGameError::InvalidIssuerLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(LotteryGameError::InvalidCountryLength.into());
    }

    if params.issue_no.as_bytes().len() > 0 && params.issue_no.as_bytes().len() <= ISSUER_NO_LENGTH
    {
    } else {
        return Err(LotteryGameError::InvalidIssuerNoLength.into());
    }

    // 1 - Fixed coupon Treasury bonds
    // 2 - Infrastructure bonds

    let is_valid_bond_type = {
        match params.type_of_bond {
            1 | 2 => true,
            _ => false,
        }
    };

    if !is_valid_bond_type {
        return Err(LotteryGameError::InvalidTypeOfBond.into());
    }

    // T-Bonds with maturities of between 2-30 years
    if params.tenor >= TENOR_LENGTH && params.tenor <= TENOR_LENGTH_2 {
    } else {
        return Err(LotteryGameError::InvalidBondTenor.into());
    }

    if params.coupon_rate > 0 {
    } else {
        return Err(LotteryGameError::InvalidBondCouponRate.into());
    }

    if params.total_amounts_offered > 0 {
    } else {
        return Err(LotteryGameError::InvalidAmount.into());
    }

    if params.minimum_bid_amount > 0 {
    } else {
        return Err(LotteryGameError::InvalidAmount.into());
    }

    if params.unit_cost_of_lottery_game > 0 {
    } else {
        return Err(LotteryGameError::InvalidAmount.into());
    }

    if params.value_date.as_bytes().len() > 0 && params.value_date.as_bytes().len() <= DATE_LENGTH {
    } else {
        return Err(LotteryGameError::InvalidValueDateLength.into());
    }

    if params.redemption_date.as_bytes().len() > 0
        && params.redemption_date.as_bytes().len() <= DATE_LENGTH
    {
    } else {
        return Err(LotteryGameError::InvalidValueRedemptionLength.into());
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
    lottery_game.issuer.issuer = params.issuer.issuer.to_string();
    lottery_game.country = params.country.to_string();
    lottery_game.issue_no = params.issue_no.to_string();
    lottery_game.type_of_bond = params.type_of_bond;
    lottery_game.tenor = params.tenor;
    lottery_game.coupon_rate = params.coupon_rate;
    lottery_game.total_amounts_offered = params.total_amounts_offered;
    lottery_game.minimum_bid_amount = params.minimum_bid_amount;
    lottery_game.is_initialized = true;
    lottery_game.unit_cost_of_lottery_game = params.unit_cost_of_lottery_game;
    lottery_game.decimals = params.decimals;
    lottery_game.value_date = params.value_date.to_string();
    lottery_game.redemption_date = params.redemption_date.to_string();

    let bond_issuer = BondIssuer {
        issuer: params.issuer.issuer.to_string(),
    };

    // lottery_game_configs
    lottery_game_configs.issuers.push(bond_issuer);

    Ok(())
}
