//! WithdrawLotteryGameWinnings instruction handler

use {
    crate::{
        error::LotteryGameError,
        state::{deposit_base::DepositBase, lottery_game::LotteryGame, participant::Participant},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
    },
};

#[derive(Accounts)]
#[instruction(params: WithdrawLotteryGameWinningsParams)]
pub struct WithdrawLotteryGameWinnings<'info> {
    #[account(mut,
        constraint = lottery_game.is_initialized @ LotteryGameError::AccountNotInitialized,
        constraint = lottery_game.is_closed @ LotteryGameError::LotteryGameClosed
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
    #[account(mut,
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
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawLotteryGameWinningsParams {
    pub amount: u32,
}

pub fn withdraw_lottery_game_winnings(
    ctx: Context<WithdrawLotteryGameWinnings>,
    params: &WithdrawLotteryGameWinningsParams,
) -> Result<()> {
    msg!("Validate inputs");
    if params.amount == 0 {
        return Err(LotteryGameError::InvalidAmount.into());
    }

    let lottery_game = &mut ctx.accounts.lottery_game;
    let participant = &mut ctx.accounts.participant;
    let sender_tokens = &ctx.accounts.sender_tokens;
    let recipient_tokens = &ctx.accounts.recipient_tokens;
    let mint_token = &ctx.accounts.mint_token;
    let deposit_account = &ctx.accounts.deposit_account;
    let pda_auth = &mut ctx.accounts.pda_auth;
    let treasury_vault = &mut ctx.accounts.treasury_vault;
    let token_program = &ctx.accounts.token_program;
    let total_available_funds = lottery_game.total_available_funds;
    let available_funds: u32 = participant.available_funds;
    let decimals: u8 = lottery_game.decimals;
    let _amount = params.amount;
    let lottery_game_winner: Pubkey = lottery_game.lottery_game_winner;

    if lottery_game_winner != *ctx.accounts.owner.key {
        return Err(LotteryGameError::InvalidLotteryGameWinner.into());
    }

    // participant's available funds should exceed zero
    if available_funds == 0 {
        return Err(LotteryGameError::InsufficientFunds.into());
    }

    // treasury's available funds should exceed zero
    if total_available_funds == 0 {
        return Err(LotteryGameError::InsufficientFunds.into());
    }

    // treasury's available funds should match transfer amount
    if total_available_funds == _amount {
    } else {
        return Err(LotteryGameError::MismatchedAmount.into());
    }

    // Reset participant's available funds
    participant.available_funds = 0;

    // Reset participant's total units lottery ticket
    participant.total_units_lottery_ticket = 0;

    // Deduct actual_amount(sold unit_cost_of_lottery_ticket) from total_available_funds
    lottery_game.total_available_funds = total_available_funds
        .checked_sub(_amount)
        .ok_or(LotteryGameError::InvalidArithmeticOperation)?;

    let base: u32 = 10;
    let exponent = lottery_game.decimals as u32;
    // lets get the amount in decimal format
    // 10 ** 9 * 3(base 10, 9 decimals, 3 amount), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    let result = (base).pow(exponent);
    let _amount = (_amount as u64)
        .checked_mul(result as u64)
        .ok_or(LotteryGameError::InvalidArithmeticOperation)?;

    // Transfer funds from treasury vault to recipient
    let cpi_accounts = TransferChecked {
        from: sender_tokens.to_account_info(),
        mint: mint_token.to_account_info(),
        to: recipient_tokens.to_account_info(),
        authority: treasury_vault.to_account_info(),
    };

    let seeds = &[
        b"treasury-vault",
        pda_auth.to_account_info().key.as_ref(),
        &[deposit_account.admin_treasury_vault_bump.unwrap()],
    ];

    let signer = &[&seeds[..]];

    let cpi = CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, signer);

    transfer_checked(cpi, _amount, decimals)?;

    Ok(())
}
