use anchor_lang::prelude::*;

use crate::constants::Constants;

#[account]
pub struct TournamentParticipation {
    pub bump: u8,
    pub ok_votes: u16,
    pub total_votes: u16,
    pub prize_distribution: Vec<f32>, //total is 100
    pub members: Vec<Pubkey>, //If member size = 1 then individual
    pub is_entered: bool,
}
impl TournamentParticipation {
    pub const LEN: usize = 
        8 +// discriminator
        1 +
        2 +
        2 +
        (4 + (4 * Constants::MAX_TEAM_MEMBER_COUNT))+//prize_distribution
        (4 + (32 * Constants::MAX_TEAM_MEMBER_COUNT))+//members
        1;

}