use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("User already in a team")]
    UserAlreadyInATeam,

    #[msg("User not in team")]
    UserNotInTeam,

    #[msg("Non-Authorized members cant make team invitation")]
    NonAuthorizedInvitation,
    
    #[msg("Proposal Account is not proposal of given Team Account")]
    NotProposalOfGivenTeamAccount,

    #[msg("Proposal Account is not proposal of given User Account")]
    NotProposalOfGivenUserAcount,

    #[msg("Team Capacity Not Enough, team members cant be higher than 5")]
    TeamCapacityNotEnough,

    #[msg("Non-Authorized members cant create tournament participation voting")]
    NonAuthorizedParticipation,

    #[msg("Account balance is not enough to provide tournament rewards")]
    AccountBalanceNotEnough,

    #[msg("Tournament capacity is full")]
    TournamentCapacityFull,

    #[msg("Team members cant enter tournament individually")]
    TeamMembersCantEnterTournamentIndividually,

    #[msg("Given account not matches with tournament participation data")]
    GivenAccountNotMatchesWithTournamentParticipationData,

    #[msg("Team owner cant leave team without transfering team ownership to other members if there is any")]
    TeamOwnerCantLeaveTeam,

    #[msg("Minimum team name length must be at least 5")]
    ShortTeamName,

    #[msg("Accounts must be given on remaining account")]
    NoAccountGiven,

    #[msg("Participant Parameter is not valid for signer or signer's team account")]
    ParticipantParameterInvalid
}