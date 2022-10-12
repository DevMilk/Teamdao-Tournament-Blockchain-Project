use crate::{entities::*, errors::Errors};
use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;

pub fn vote_tournament_participation(ctx: Context<VoteTournamentParticipation>, answer: bool) -> Result<()> {    
    // setting userdata in user's account
    let participation = &mut ctx.accounts.tournament_participation_data;
    let team = &mut ctx.accounts.team_account;
    let tournament = &mut ctx.accounts.tournament;
    let team_owner = &mut ctx.accounts.signer;
    let vote_record = &mut ctx.accounts.vote_record;

    vote_record.bump = *ctx.bumps.get("vote_record").unwrap();

    //If tournament is full, then close the proposal and return error
    if !tournament.check_entrance_availability() {
        participation.close(team_owner.to_account_info());
        return Err(Errors::TournamentCapacityFull.into());
    }

    participation.total_votes += 1;
    participation.ok_votes += if answer == true {1} else {0};
    
    //Check if voting is done (i am not closing proposal to prevent duplicates amongs tournament participants)
    if 3 * participation.ok_votes as usize >= 2 * team.members.len() {
        //If majority is at least 2/3 of team then it is ok to enter the tournament

        let tournament = &mut ctx.accounts.tournament;

        participation.members = team.members.clone();
        let equal_percentage = 1.0 / (team.members.len() as f32);
        participation.prize_distribution = vec![equal_percentage; team.members.len()];
        tournament.participants.push(team.key());
        participation.is_entered = true;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct VoteTournamentParticipation<'info> {

    //Team Authority (Auto)
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()], //Check if it is signer's user account
        bump = team_member.bump
    )] 
    pub team_member: Account<'info, UserAccount>,


    //Team Account (Auto)
    #[account(
        mut, 
        seeds = ["team".as_bytes(), team_member.current_team.as_bytes()], 
        bump = team_account.bump,
    )]
    pub team_account: Account<'info, Team>,

    
    //Team Invitation Proposal (Auto)
    #[account(
        mut,
        seeds = [
            "tournament-participation".as_bytes(), 
            tournament.key().as_ref(), 
            //If it is individual (non-team member) then pass its pubkey if it is not then pass team_addr
            team_account.key().as_ref()
        ],
        bump = tournament_participation_data.bump,
    )]
    pub tournament_participation_data: Account<'info, TournamentParticipation>,

    //User can only vote for once and cant change decision in this instruction
    #[account(
        init,
        space = VoteRecord::LEN,
        payer = signer,
        seeds = ["vote-record".as_bytes(), tournament_participation_data.key().as_ref(), signer.key().as_ref()],
        bump,
    )]
    pub vote_record: Account<'info, VoteRecord>,

    #[account(mut)] 
    pub tournament: Box<Account<'info, Tournament>>,
    //Signer must be team authority
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}