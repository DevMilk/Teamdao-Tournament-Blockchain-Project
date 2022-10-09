use anchor_lang::prelude::*;

#[account]
pub struct TournamentProposal {
    pub bump: u8,
    pub ok_votes: u16,
    pub total_votes: u16,
    pub team_size: u8
}
impl TournamentProposal {
    pub const LEN: usize = 
        8 + // discriminator
        1 +
        2 +
        2 +
        1;
}