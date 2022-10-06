use anchor_lang::prelude::*;

#[account]
pub struct InvitationProposal {
    pub invited: Pubkey, // Authority of this account
    pub bump: u8,
    pub team: Pubkey, //max 5,
}
impl InvitationProposal {
    pub const LEN: usize = 
        8 + // discriminator
        32 + // Pubkey
        1 + //bump
        32;
    pub const SEED: &[u8] = "invitation-proposal".as_bytes();
}