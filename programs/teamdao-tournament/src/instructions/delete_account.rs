use crate::{entities::user_account::*, errors::Errors};
use anchor_lang::prelude::*;

pub fn delete_account(_ctx: Context<DeleteAccount>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteAccount<'info> {
    
    //Deleted account must be account of signer
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()], 
        bump = user_account.bump,
        constraint = user_account.current_team.is_empty() @ Errors::TeamMembersCantDeleteTheirAccountWithoutLeavingTeam,
        close=signer
    )]
    pub user_account: Account<'info, UserAccount>,


    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}