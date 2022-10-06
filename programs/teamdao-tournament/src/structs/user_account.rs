use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub bump: u8,
    pub current_team: Option<Pubkey>
}
impl UserAccount {
    pub const LEN: usize = 
        8 + // discriminator
        32 + // Pubkey
        1 +
        (32+1);
    pub const SEED: &[u8] = "user-account".as_bytes();
}