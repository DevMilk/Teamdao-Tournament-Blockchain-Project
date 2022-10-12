use anchor_lang::prelude::*;
use crate::constants::Constants;

#[account]
pub struct Tournament {
    pub tournament_id: String,
    pub tournament_name: String,
    pub reward: u64,
    pub participants: Vec<Pubkey>, //Can be single participant or team (Only Necessary for fetching participants)
    pub bump: u8,
    pub manager: Pubkey, // tournament creator, prize payer and the one who decides winner
    pub max_participant_num: u16,
}
impl Tournament {
    pub const CONSTANT_LEN: usize = 
        8 + // discriminator
        (4+16) + // tournament id
        (4+Constants::MAX_TEAM_NAME_LENGTH) + //tournament_name
        8 +//tournament_reward
        //participant size varies on the participants 
        1 + //bump
        8 + //manager pubkey
        2; //max_participant_num
    pub fn check_entrance_availability(&self) -> bool {
        return self.participants.len() < self.max_participant_num as usize;
    }
}