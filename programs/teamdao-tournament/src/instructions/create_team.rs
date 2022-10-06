use crate::structs::*;
use crate::errors::*;

use anchor_lang::prelude::*;

pub fn create_team(ctx: Context<CreateTeam>, team_name: String) -> Result<()> {    
    // setting userdata in user's account
    let team_data = &mut ctx.accounts.team;
    let founder_user = &mut ctx.accounts.founder_user;

    let founder_user_key = founder_user.key();

    team_data.team_name = team_name;
    team_data.bump = *ctx.bumps.get("team").unwrap();
    team_data.authority = founder_user_key;
    team_data.members.push(founder_user_key);
    founder_user.current_team = Some(team_data.key());

    Ok(())
}

#[derive(Accounts)]
#[instruction(team_name: String)]
pub struct CreateTeam<'info> {


    //Initilization of the team
    #[account(
        // Team founder must not be in any team.
        constraint = founder_user.current_team == None @ Errors::AccountAlreadyInATeam,
        init, 
        payer = signer, 
        space = Team::LEN, 
        seeds = [Team::SEED, team_name.as_bytes()], 
        bump,
    )] 
    pub team: Account<'info, Team>,

    //Team Founder
    #[account(
        mut,
        signer,
        seeds = [UserAccount::SEED, signer.key().as_ref()], 
        bump = founder_user.bump
    )]
    pub founder_user: Account<'info, UserAccount>,

    

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}