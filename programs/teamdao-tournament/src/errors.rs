use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("User already in a team")]
    UserAlreadyInATeam,

    #[msg("Non-Authoritzed members cant make team invitation")]
    NonAuthorizedInvitation,
    
    #[msg("Proposal Account is not proposal of given Team Account")]
    NotProposalOfGivenTeamAccount,

    #[msg("Proposal Account is not proposal of given User Account")]
    NotProposalOfGivenUserAcount,

    #[msg("Team Capacity Not Enough, team members cant be higher than 5")]
    TeamCapacityNotEnough,

    #[msg("Non-Authoritzed members cant create tournament participation voting")]
    NonAuthorizedParticipation,
}