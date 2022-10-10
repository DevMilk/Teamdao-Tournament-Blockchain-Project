use anchor_lang::prelude::*;

use crate::constants::Constants;

#[account]
pub struct UserAccount {
    pub bump: u8,
    pub current_team: String,
    pub team_addr: Option<Pubkey>,
    pub is_authority: bool,
    pub tournament_wins: u16,
}
impl UserAccount {
    pub const LEN: usize = 
        8 + // discriminator
        1 +
        (1+32) +
        (4+Constants::MAX_TEAM_NAME_LENGTH) +
        1 +
        2;
    //pub const SEED: &'static[u8; 12] = b"user-account"; referencing seeds not works :/
}