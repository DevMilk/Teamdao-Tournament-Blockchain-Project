use crate::structs::*;
use crate::errors::*;

use anchor_lang::prelude::*;

pub fn answer_proposal(ctx: Context<AnswerProposal>, answer: bool) -> Result<()> {    
    // setting userdata in user's account
    let team = &mut ctx.accounts.team;
    let invited = &mut ctx.accounts.invited;
    
    
    if answer == true{
        //If user rejects invitation we dont need to check the capacity
        require!(team.members.len() < 5, Errors::AccountAlreadyInATeam); 
        invited.current_team = Some(team.key());
        team.members.push(invited.key());
    }
    
    Ok(())
}
//constraint =  //User must not be in a team to accept but can be invited
#[derive(Accounts)]
#[instruction(answer: bool)]
pub struct AnswerProposal<'info> {

    #[account(
        mut, 
        seeds = [InvitationProposal::SEED, signer.key().as_ref(), invitation_proposal.team.as_ref()], 
        bump = invitation_proposal.bump,
        constraint = invitation_proposal.invited == signer.key() @ Errors::NonAuthorityInvitation,
        constraint = invitation_proposal.team == team.key() @ Errors::NotProposalOfGivenTeamAccount,
        constraint = invitation_proposal.invited == invited.key() @ Errors::NotProposalOfGivenUserAcount,
        constraint = invited.current_team == None @ Errors::TeamCapacityNotEnough,
        close = signer
    )]
    pub invitation_proposal: Account<'info, InvitationProposal>,

    #[account(
        mut,
        seeds = [Team::SEED, team.team_name.as_bytes()],
        bump = team.bump,
    )]
    pub team: Account<'info, Team>,

    #[account(
        mut,
        seeds = [UserAccount::SEED, signer.key().as_ref()],
        bump = invited.bump,
    )]
    pub invited: Account<'info, UserAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}