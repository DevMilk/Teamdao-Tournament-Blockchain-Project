use crate::entities::*;
use crate::errors::*;
use crate::constants::Constants;
use anchor_lang::prelude::*;

pub fn answer_proposal(ctx: Context<AnswerProposal>, answer: bool) -> Result<()> {    

    let team = &mut ctx.accounts.team_account;
    let invited = &mut ctx.accounts.invited;
    
    
    if answer == true{
        //If user rejects invitation we dont need to check the capacity
        require!(team.members.len() < Constants::MAX_TEAM_MEMBER_COUNT, Errors::TeamCapacityNotEnough); 
        invited.current_team = team.team_name.clone();
        invited.team_addr = Some(team.key());
        team.members.push(*ctx.accounts.signer.key);
    }
    
    Ok(())
}
//constraint =  //User must not be in a team to accept but can be invited
#[derive(Accounts)]
pub struct AnswerProposal<'info> {


    //Invited User
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()],
        bump = invited.bump,
    )]
    pub invited: Account<'info, UserAccount>,

    //Team Account that user got invited (must be specified in accounts)
    #[account(
        mut,
        seeds = ["team".as_bytes(), team_account.team_name.as_bytes()],
        bump = team_account.bump,
    )]
    pub team_account: Account<'info, Team>,


    //Invitation must be signed by invited user
    //Invited user must not be in a team because one user can only have one team
    #[account(
        mut, 
        seeds = ["invitation-proposal".as_bytes(), signer.key().as_ref(), team_account.key().as_ref()], 
        bump = invitation_proposal.bump,
        constraint = invited.current_team.is_empty() @ Errors::UserAlreadyInATeam,
        close = signer //Closed the proposal so if user leaves team, closing this will allow user to get invitation again.
    )]
    pub invitation_proposal: Account<'info, VoteRecord>,

    //Invited User's Sign
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}