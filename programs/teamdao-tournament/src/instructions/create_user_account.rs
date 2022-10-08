use crate::structs::*;
use anchor_lang::prelude::*;


pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {    
    // setting userdata in user's account
    let users_account_data = &mut ctx.accounts.user_account;
    
    users_account_data.bump = *ctx.bumps.get("user_account").unwrap();
    //users_account_data.current_team = String::from("");

    Ok(())
}

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {

    //Initilization of User Account
    #[account(
        init, 
        payer = signer, 
        space = UserAccount::LEN, 
        seeds = ["user-account".as_bytes(), signer.key().as_ref()], 
        bump,
    )] 
    pub user_account: Account<'info, UserAccount>,

    //Signer is the owner of new created account
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}