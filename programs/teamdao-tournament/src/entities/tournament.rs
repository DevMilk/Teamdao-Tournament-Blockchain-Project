use anchor_lang::prelude::*;
use crate::constants::Constants;

#[account]
pub struct Tournament {
    pub tournament_id: String,
    pub tournament_name: String,
    pub reward: u16,
    pub participants: Vec<Pubkey>, //Can be single participant or team
    pub bump: u8,
    pub manager: Pubkey // tournament creator, prize payer and the one who decides winner 
}
impl Tournament {
    pub const CONSTANT_LEN: usize = 
        8 + // discriminator
        (4+16) + // tournament id
        (4+Constants::MAX_TEAM_NAME_LENGTH) + //tournament_name
        2 +//tournament_reward
        //(4 + (32 * Constants::MAX_PARTICIPANT_COUNT)) + // Max participant count is 20.
        1 + //bump
        8; //manager
}