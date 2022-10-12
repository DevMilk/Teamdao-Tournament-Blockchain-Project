use anchor_lang::prelude::*;

//Simplest record
#[account]
pub struct Record {
    pub bump: u8,
}
impl Record {
    pub const LEN: usize = 
        8 + // discriminator
        1; //bump
}