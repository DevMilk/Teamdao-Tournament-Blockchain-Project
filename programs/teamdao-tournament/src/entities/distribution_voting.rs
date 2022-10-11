use anchor_lang::prelude::*;

#[account]
pub struct DistributionVoting {
    pub bump: u8,
    pub ok_votes: u8,
    pub total_votes: u8,
    pub prize_distribution: Vec<f32>
}
impl DistributionVoting {
    pub const CONSTANT_LEN: usize = 
        8 + // discriminator
        1 +
        1 +
        1;
}