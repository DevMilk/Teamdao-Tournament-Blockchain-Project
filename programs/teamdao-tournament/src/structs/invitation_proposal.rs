use anchor_lang::prelude::*;

#[account]
pub struct InvitationProposal {
    pub bump: u8,
}
impl InvitationProposal {
    pub const LEN: usize = 
        8 + // discriminator
        1;
    //pub const SEED: &'static[u8; 19] = b"invitation-proposal"; referencing seeds not works :/
}