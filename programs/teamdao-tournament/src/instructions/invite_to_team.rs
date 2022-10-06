use crate::structs::*;
use crate::errors::*;
use anchor_lang::prelude::*;


pub fn invite_to_team(ctx: Context<InviteToTeam>) -> Result<()> {    
    // setting userdata in user's account
    let proposal = &mut ctx.accounts.invitation_proposal;
    
    proposal.bump = *ctx.bumps.get("invitation_proposal").unwrap();
    proposal.team = ctx.accounts.team.key();
    proposal.invited = ctx.accounts.invited.key();

    Ok(())
}

#[derive(Accounts)]
pub struct InviteToTeam<'info> {

    //Initialization of Team Invitation Proposal
    #[account(
        init,
        payer = signer,
        seeds = [InvitationProposal::SEED, invited.key().as_ref(), team.key().as_ref()],
        bump,
        space = InvitationProposal::LEN
    )] 
    pub invitation_proposal: Account<'info, InvitationProposal>,

    //Invited user
    #[account(
        mut,
        seeds = [UserAccount::SEED, invited.key().as_ref()], 
        bump = invited.bump
        
    )] 
    pub invited: Account<'info, UserAccount>,

    #[account(
        mut, 
        seeds = [Team::SEED, team.team_name.as_bytes()], 
        bump = invited.bump,
        //Only authority of the team can invite someone
        constraint = team.authority == signer.key() @ Errors::NonAuthorityInvitation
    )]
    pub team: Account<'info, Team>,


    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}