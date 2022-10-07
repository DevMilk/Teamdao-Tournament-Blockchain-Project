use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub bump: u8,
    pub current_team: Option<Pubkey>
}
impl<'a> UserAccount {
    pub const LEN: usize = 
        8 + // discriminator
        32 + // Pubkey
        1 +
        (32+1);
    //pub const SEED: &'static[u8; 12] = b"user-account"; referencing seeds not works :/
}