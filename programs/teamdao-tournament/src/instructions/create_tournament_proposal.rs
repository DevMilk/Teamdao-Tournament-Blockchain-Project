use crate::{entities::*, errors::Errors};
use anchor_lang::prelude::*;

pub fn create_tournament_proposal(ctx: Context<CreateTournamentProposal>) -> Result<()> {    
    // setting userdata in user's account
    let proposal = &mut ctx.accounts.tournament_proposal;

    proposal.bump = *ctx.bumps.get("tournament_proposal").unwrap();
    
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTournamentProposal<'info> {

    //Team Authority (Auto)
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()], //Check if it is signer's user account
        bump = team_authority.bump
    )] 
    pub team_authority: Account<'info, UserAccount>,


    //Team Account (Auto)
    #[account(
        mut, 
        seeds = ["team".as_bytes(), team_authority.current_team.as_bytes()],  //Checks if it is the signer = team_authority's team
        bump = team_account.bump,
        //Only authority of the team can invite someone
        constraint = team_account.authority == signer.key() @ Errors::NonAuthorizedInvitation
    )]
    pub team_account: Account<'info, Team>,

    
    //Initialization of Team Invitation Proposal
    #[account(
        init,
        payer = signer,
        seeds = ["tournament-proposal".as_bytes(), tournament.key().as_ref(), team_account.key().as_ref()],
        bump,
        space = TournamentProposal::LEN
    )] 
    pub tournament_proposal: Account<'info, TournamentProposal>,

    #[account(mut)] 
    pub tournament: Account<'info, Tournament>,
    //Signer must be team authority
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}