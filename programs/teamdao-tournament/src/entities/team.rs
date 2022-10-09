use anchor_lang::prelude::*;
use crate::constants::Constants;
#[account]
pub struct Team {
    pub authority: Pubkey, // Authority of this account
    pub bump: u8,
    pub members: Vec<Pubkey>, //max 5,
    pub team_name: String
}
impl Team {
    pub const LEN: usize = 
        8 + // discriminator
        32 + // Pubkey
        1 + //bump
        4 +(32 * Constants::MAX_TEAM_MEMBER_COUNT) + // members limited with 5
        4 + Constants::MAX_TEAM_NAME_LENGTH; //team name max 30 character
    //pub const SEED: &'static[u8; 4] = b"team"; referencing seeds not works :/
}