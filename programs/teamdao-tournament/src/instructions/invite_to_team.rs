

use crate::entities::*;
use crate::errors::*;
use anchor_lang::prelude::*;

pub fn invite_to_team(ctx: Context<InviteToTeam>, _invited_pubkey: Pubkey) -> Result<()> {    
    // setting userdata in user's account
    let proposal = &mut ctx.accounts.invitation_proposal;

    proposal.bump = *ctx.bumps.get("invitation_proposal").unwrap();

    Ok(())
}

#[derive(Accounts)]
#[instruction(invited_pubkey: Pubkey)]
pub struct InviteToTeam<'info> {

    //Team Authority (Auto)
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()], //Check if it is signer's user account
        bump = team_authority.bump
    )] 
    pub team_authority: Account<'info, UserAccount>,

    
    //Team Account
    #[account(
        mut, 
        seeds = ["team".as_bytes(), team_authority.current_team.as_bytes()],  //Checks if it is the signer = team_authority's team
        bump = team_account.bump,
        //Only authority of the team can invite someone
        constraint = team_account.authority == signer.key() @ Errors::NonAuthorizedInvitation
    )]
    pub team_account: Account<'info, Team>,

    //Team Authority (Auto)
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), invited_pubkey.as_ref()],
        bump = invited.bump,
        constraint = invited.current_team != team_account.team_name
    )] 
    pub invited: Account<'info, UserAccount>,
    
    //Initialization of Team Invitation Proposal
    #[account(
        init,
        payer = signer,
        seeds = ["invitation-proposal".as_bytes(), invited_pubkey.as_ref(), team_account.key().as_ref()],
        bump,
        space = VoteRecord::LEN
    )] 
    pub invitation_proposal: Account<'info, VoteRecord>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}