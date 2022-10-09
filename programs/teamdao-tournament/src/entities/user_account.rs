use anchor_lang::prelude::*;

use crate::constants::Constants;

#[account]
pub struct UserAccount {
    pub bump: u8,
    pub current_team: String,
    pub tournament_wins: u16
}
impl<'a> UserAccount {
    pub const LEN: usize = 
        8 + // discriminator
        1 +
        (4+Constants::MAX_TEAM_NAME_LENGTH) +
        2;
    //pub const SEED: &'static[u8; 12] = b"user-account"; referencing seeds not works :/
}