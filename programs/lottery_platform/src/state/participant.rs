use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Participant {
    pub owner: Pubkey, // publickey of the participant
    #[max_len(50)]
    pub full_names: String, // full names i.e first name, middlename, surname
    #[max_len(3)]
    pub country: String, // home country of participant
    pub active: bool,  // status of participant
    pub total_units_lottery_ticket: u32, // total units of lottery game tickets bought by participant
    pub available_funds: u32, // available funds equal to the lottery game owned by participant
}
