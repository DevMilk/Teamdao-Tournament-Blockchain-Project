use crate::entities::*;
use crate::errors::*;

use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;

pub fn leave_team(ctx: Context<LeaveTeam>) -> Result<()> {    
    // setting userdata in user's account
    let team = &mut ctx.accounts.team_account;
    let team_member = &mut ctx.accounts.team_member;
    let signer = &mut ctx.accounts.signer;
    
    let member_key = *signer.key; //Check
    let is_authority = team.authority == member_key;

    //Empty leaving member's current team field
    team_member.current_team.clear();

    //Delete user pubKey from team members
    team.members.retain(|&x| x != member_key);

    //If there is no member left, then delete team account
    if team.members.len() == 0 {
        team.close(signer.to_account_info());
    }
    //If there are other members, assign authority to latest joined
    else if is_authority {
        let new_team_authority = &team.members[0];
        team.authority = *new_team_authority;
    }
    Ok(())
}
//constraint =  //User must not be in a team to accept but can be invited
#[derive(Accounts)]
pub struct LeaveTeam<'info> {


    //Invited User
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()],
        bump = team_member.bump,
    )]
    pub team_member: Account<'info, UserAccount>,


    //Team of user
    #[account(
        mut,
        seeds = ["team".as_bytes(), team_member.current_team.as_bytes()],
        bump = team_account.bump,
    )]
    pub team_account: Account<'info, Team>,

    

    //Invited User's Sign
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}