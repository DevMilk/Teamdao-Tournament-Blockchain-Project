use anchor_lang::prelude::*;

use crate::constants::Constants;

#[account]
pub struct VoteRecord {
    pub bump: u8,
}
impl VoteRecord {
    pub const LEN: usize = 
        8 + // discriminator
        1;
}