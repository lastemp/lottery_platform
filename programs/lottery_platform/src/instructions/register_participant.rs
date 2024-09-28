//! RegisterParticipant instruction handler

use {
    crate::{error::LotteryGameError, state::participant::Participant},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: RegisterParticipantParams)]
pub struct RegisterParticipant<'info> {
    // init means to create account
    // bump to use unique address for account
    #[account(
        init,
        payer = owner,
        space = 8 + Participant::INIT_SPACE,
        seeds = [b"participant", owner.key().as_ref()],
        bump
    )]
    pub participant: Account<'info, Participant>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterParticipantParams {
    full_names: String, // full names i.e first name, middlename, surname
    country: String,    // home country of participant
}

// full names length
const FULL_NAMES_LENGTH: usize = 50;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn register_participant(
    ctx: Context<RegisterParticipant>,
    params: &RegisterParticipantParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.full_names.as_bytes().len() > 0
        && params.full_names.as_bytes().len() <= FULL_NAMES_LENGTH
    {
    } else {
        return Err(LotteryGameError::InvalidFullNamesLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(LotteryGameError::InvalidCountryLength.into());
    }

    let participant = &mut ctx.accounts.participant;

    // * - means dereferencing
    participant.owner = *ctx.accounts.owner.key;
    participant.full_names = params.full_names.to_string();
    participant.country = params.country.to_string();
    participant.active = true;

    Ok(())
}
