use crate::{entities::*, errors::Errors, constants::Constants};
use anchor_lang::{prelude::*, solana_program::instruction};

pub fn create_tournament(ctx: Context<CreateTournament>, tournament_id: String, tournament_name: String, reward: u16) -> Result<()> {    

    let tournament = &mut ctx.accounts.new_tournament;
    
    tournament.bump = *ctx.bumps.get("new_tournament").unwrap();
    tournament.tournament_id = tournament_id;
    tournament.tournament_name = tournament_name;
    tournament.reward = reward;
    tournament.manager = *ctx.accounts.signer.key;

    let from = &ctx.accounts.signer;
    let to = tournament;
    /*let ix = anchor_lang::solana_program::system_instruction::transfer(
        &from.key(),
        &to.key(),
        reward * Constants::LAMPORTS,
    );*/
    /*anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            from.to_account_info(),
            to.to_account_info(),
        ],
    );*/

    Ok(())
}

#[derive(Accounts)]
#[instruction(tournament_id: String)]
pub struct CreateTournament<'info> {

    //Initilization of the tournament account
    #[account(
        init, 
        payer = signer, 
        space =  Tournament::LEN, 
        seeds = ["tournament".as_bytes(), tournament_id.as_bytes()], 
        bump,
    )] 
    pub new_tournament: Account<'info, Tournament>,

    
    //Signer is the owner of new created account
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}