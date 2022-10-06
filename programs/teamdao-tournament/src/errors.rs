use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("User already in a team")]
    AccountAlreadyInATeam,

    #[msg("Non-Authority members cant make team invitation")]
    NonAuthorityInvitation,
    
    #[msg("Proposal Account is not proposal of given Team Account")]
    NotProposalOfGivenTeamAccount,

    #[msg("Proposal Account is not proposal of given User Account")]
    NotProposalOfGivenUserAcount,

    #[msg("Team Capacity Not Enough, team members cant be higher than 5")]
    TeamCapacityNotEnough,
}