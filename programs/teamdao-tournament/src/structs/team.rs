use anchor_lang::prelude::*;

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
        4 +(32 * 5) + // members limited with 5
        4 + 30; //team name max 30 character
    pub const SEED: &[u8] = "team".as_bytes();
}