use crate::{entities::*, errors::Errors};
use anchor_lang::{prelude::*};

pub fn enter_tournament(ctx: Context<EnterTournament>, participant: Pubkey) -> Result<()> {    

    let tournament = &mut ctx.accounts.tournament;
    let tournament_participation_data = &mut ctx.accounts.tournament_participation_data;
    let individual_or_team_owner = &mut ctx.accounts.user_account;

    //Check if given pubkey is an individual or team. 
    //If individual check if he have a team.
    //If it is a team then check if signer is authority or not.
    //A team member cant join tournament individually

    tournament_participation_data.ok_votes = 0;
    tournament_participation_data.is_entered = false;
    tournament_participation_data.bump = *ctx.bumps.get("tournament_participation_data").unwrap();

    //If it is individual, directly join to tournament. If it is a team owner then enter tournament on voting. 
    match individual_or_team_owner.team_addr 
    {
        Some(team_addr)=> {
            require!(participant == team_addr, Errors::ParticipantParameterInvalid);
        }, 
        None => {
            
            require!(participant == *ctx.accounts.signer.key, Errors::ParticipantParameterInvalid);
            tournament_participation_data.is_entered = true;
            tournament.participants.push(*ctx.accounts.signer.key);
            tournament_participation_data.members.push(*ctx.accounts.signer.key);
        }
    }

    

    Ok(())
}

#[derive(Accounts)]
#[instruction(participant: Pubkey)]
pub struct EnterTournament<'info> {

    //Tournament to join
    #[account(
        mut,
        constraint = tournament.check_entrance_availability() @ Errors::TournamentCapacityFull
    )] 
    pub tournament: Box<Account<'info, Tournament>>,

    //tournament_participation data
    #[account(
        init,
        space = TournamentParticipation::LEN,
        payer = signer,
        //constraint = participant == match &user_account.team_addr {Some(team_addr)=> *team_addr, None => signer.key()} @ Errors::ParticipantParameterInvalid,
        seeds = [
            "tournament-participation".as_bytes(), 
            tournament.key().as_ref(), 
            //If it is individual (non-team member) then pass its pubkey if it is not then pass team_addr
            participant.as_ref()
        ],
        bump,
    )]
    pub tournament_participation_data: Account<'info, TournamentParticipation>,


    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()], 
        bump = user_account.bump,
        constraint = user_account.team_addr == None || user_account.is_authority @ Errors::NonAuthorizedParticipation
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}