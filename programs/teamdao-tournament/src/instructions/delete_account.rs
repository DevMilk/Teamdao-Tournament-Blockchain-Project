use crate::entities::user_account::*;
use anchor_lang::prelude::*;

pub fn delete_account(ctx: Context<DeleteAccount>) -> Result<()> {
    msg!("Account Closed successfully");
    msg!(&ctx.accounts.user_account.bump.to_string());
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteAccount<'info> {
    
    //Deleted account must be account of signer
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()], 
        bump = user_account.bump,
        close=signer
    )]
    pub user_account: Account<'info, UserAccount>,


    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}