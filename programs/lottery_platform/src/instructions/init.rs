//! Init instruction handler

use {
    crate::{error::LotteryGameError, state::configs::LotteryGameConfigs},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct Init<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + LotteryGameConfigs::INIT_SPACE,
        constraint = !treasury_bonds_configs.is_initialized @ LotteryGameError::AccountAlreadyInitialized,
        seeds = [b"lottery-game-configs"],
        bump
    )]
    pub treasury_bonds_configs: Account<'info, LotteryGameConfigs>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init(ctx: Context<Init>) -> Result<()> {
    let treasury_bonds_configs = &mut ctx.accounts.treasury_bonds_configs;

    // treasury bonds
    treasury_bonds_configs.is_initialized = true;

    Ok(())
}
