use crate::{entities::*, errors::Errors, constants::Constants};
use anchor_lang::{prelude::*, solana_program::instruction};

pub fn create_tournament(ctx: Context<CreateTournament>, tournament_id: String, tournament_name: String, reward: u16, max_participant_num: u16) -> Result<()> {    

    let tournament = &mut ctx.accounts.new_tournament;
    
    //tournament.bump = *ctx.bumps.get("new_tournament").unwrap();
    tournament.tournament_id = tournament_id;
    tournament.tournament_name = tournament_name;
    tournament.reward = reward;
    tournament.manager = *ctx.accounts.signer.key;

    //Transfer rewards to tournament pda
    let from = &ctx.accounts.signer;
    let to = tournament;
    
    let reward_as_lamport: u64 = (reward as u64) * Constants::LAMPORTS;
    require!(from.to_account_info().lamports() >= reward_as_lamport, Errors::AccountBalanceNotEnough);

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &from.key(),
        &to.key(),
        reward_as_lamport,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            from.to_account_info(),
            to.to_account_info(),
        ],
    ); 

    Ok(())
}

#[derive(Accounts)]
#[instruction(tournament_id: String, max_participant_num: u16)]
pub struct CreateTournament<'info> {

    //Initilization of the tournament account
    #[account(
        init, 
        payer = signer, 
        space =  Tournament::CONSTANT_LEN + (4 + (32 * max_participant_num)) as usize, 
        seeds = ["tournament".as_bytes(), signer.key().as_ref(), tournament_id.as_bytes()], 
        bump,
    )] 
    pub new_tournament: Account<'info, Tournament>,

    
    //Signer is the owner of new created account
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}