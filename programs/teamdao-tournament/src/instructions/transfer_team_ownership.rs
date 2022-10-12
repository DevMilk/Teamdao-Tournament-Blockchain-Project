use crate::entities::*;
use crate::errors::*;
use anchor_lang::prelude::*;

pub fn transfer_team_ownership(ctx: Context<TransferTeamOwnership>, new_owner: Pubkey) -> Result<()> {    

    let team_data = &mut ctx.accounts.team;
    let new_owner_user = &mut ctx.accounts.new_owner_user;

    //Check if given user is member of the team or not
    require!(new_owner_user.current_team == team_data.team_name, Errors::UserNotInTeam);

    new_owner_user.is_authority = true;
    team_data.authority = new_owner;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(new_owner:Pubkey)]
pub struct TransferTeamOwnership<'info> {

    //Team Captain (Auto)
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()], 
        bump = team_authority.bump
    )]
    pub team_authority: Account<'info, UserAccount>,
    
    //Team Account (Auto)
    #[account(
        mut, 
        seeds = ["team".as_bytes(), team_authority.current_team.as_bytes()], 
        bump = team.bump,
    )] 
    pub team: Account<'info, Team>,

    #[account(
        mut, 
        seeds = ["user-account".as_bytes(), new_owner.as_ref()], 
        bump = new_owner_user.bump
    )] 
    pub new_owner_user: Account<'info, UserAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}