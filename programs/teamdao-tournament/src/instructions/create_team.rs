use crate::entities::*;
use crate::errors::*;
use crate::constants::Constants;
use anchor_lang::prelude::*;

pub fn create_team(ctx: Context<CreateTeam>, team_name: String) -> Result<()> {    
    // setting userdata in user's account
    let team_data = &mut ctx.accounts.team;
    let founder_user = &mut ctx.accounts.team_authority;

    let signer_key = *ctx.accounts.signer.key;

    //Update team data
    team_data.team_name = team_name.clone();
    team_data.bump = *ctx.bumps.get("team").unwrap();
    team_data.authority = signer_key;
    team_data.members.push(signer_key);
    
    //Update founder data
    founder_user.current_team = team_name.clone();
    founder_user.is_authority = true;
    founder_user.team_addr = Some(team_data.key());
    Ok(())
}

#[derive(Accounts)]
#[instruction(team_name: String)]
pub struct CreateTeam<'info> {


    //Initilization of the team
    #[account(
        // Team founder must not be in any team.
        constraint = team_authority.current_team.is_empty() @ Errors::UserAlreadyInATeam,
        //Check team name
        constraint = team_name.len() > Constants::MIN_TEAM_NAME_LENGTH @ Errors::ShortTeamName,
        constraint = team_name.len() < Constants::MAX_TEAM_NAME_LENGTH @ Errors::LongTeamName,
        init, 
        payer = signer, 
        space = Team::LEN, 
        seeds = ["team".as_bytes(), team_name.as_bytes()], 
        bump,
    )] 
    pub team: Account<'info, Team>,

    //Team founder's user account
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()], 
        bump = team_authority.bump
    )]
    pub team_authority: Account<'info, UserAccount>,
    
    //Signer is the one that creates team
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}