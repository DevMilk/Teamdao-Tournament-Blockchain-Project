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

    //Invitation must be signed by invited user
    #[account(
        mut, 
        seeds = ["invitation-proposal".as_bytes(), signer.key().as_ref(), team.key().as_ref()], 
        bump = invitation_proposal.bump,
        constraint = invited.current_team == None @ Errors::TeamCapacityNotEnough,
        close = signer
    )]
    pub invitation_proposal: Account<'info, InvitationProposal>,

    //Team Account that user got invited
    #[account(
        mut,
        seeds = ["team".as_bytes(), team.team_name.as_bytes()],
        bump = team.bump,
    )]
    pub team: Account<'info, Team>,

    //Invited User
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()],
        bump = invited.bump,
    )]
    pub invited: Account<'info, UserAccount>,

    //Invited User's Sign
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}