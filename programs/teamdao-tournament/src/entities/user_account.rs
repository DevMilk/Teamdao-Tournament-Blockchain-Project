use anchor_lang::prelude::*;

use crate::constants::Constants;

#[account]
pub struct UserAccount {
    pub bump: u8,
    pub current_team: String, //current team's name
    pub team_addr: Option<Pubkey>, //team address
    pub is_authority: bool, //Is user a team authority?
}
impl UserAccount {
    pub const LEN: usize = 
        8 + // discriminator
        1 + //bump
        (4+Constants::MAX_TEAM_NAME_LENGTH) + //current_team's name 
        (1+32) + //team_addr
        1;  //is_authority
    //pub const SEED: &'static[u8; 12] = b"user-account"; referencing seeds not works :/
}