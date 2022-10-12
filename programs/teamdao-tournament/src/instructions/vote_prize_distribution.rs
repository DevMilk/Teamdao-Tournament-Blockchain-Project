use crate::entities::*;
use crate::errors::*;
use anchor_lang::AccountsClose;
use anchor_lang::prelude::*;


pub fn vote_prize_distribution(ctx: Context<VotePrizeDistribution>, 
    answer: bool, _tournament: Pubkey) -> Result<()> {    


    let voting = &mut ctx.accounts.distribution_voting;
    let participant_data = &mut ctx.accounts.tournament_participation_data;
    let voter = &ctx.accounts.signer;

    ctx.accounts.vote_record.bump = *ctx.bumps.get("vote_record").unwrap();
    require!(participant_data.members.contains(voter.key), Errors::ParticipantNotFound);

    voting.total_votes +=1;
    voting.ok_votes += if answer == true {1} else {0};
    
    //Voting ends if total votes is equal to participant count (not always equals to team member length)
    if 3 * voting.ok_votes as usize >= 2 * participant_data.members.len() {
        participant_data.prize_distribution = voting.prize_distribution.clone();
        let _ = &ctx.accounts.distribution_voting.close(voter.to_account_info());
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(answer: bool, tournament: Pubkey)]
pub struct VotePrizeDistribution<'info> {

    //Voter
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()],
        bump = voter_member.bump,
    )]
    pub voter_member: Account<'info, UserAccount>,

    //Team Account (Auto)
    #[account(
        mut, 
        seeds = ["team".as_bytes(), voter_member.current_team.as_bytes()], 
        bump = team_account.bump,
    )]
    pub team_account: Account<'info, Team>,

    //Tournament participation data (Auto)
    #[account(
        mut,
        seeds = [
            "tournament-participation".as_bytes(), 
            tournament.key().as_ref(), 
            //If it is individual (non-team member) then pass its pubkey if it is not then pass team_addr
            team_account.key().as_ref()
        ],
        bump = tournament_participation_data.bump,
        //Note: If someone is joined the team after team joined tournament, user cant vote or get share from tournament

    )]
    pub tournament_participation_data: Account<'info, TournamentParticipation>,

    //distribution voting what increases ok (Must given)
    #[account(mut)]
    pub distribution_voting: Account<'info, DistributionVoting>,

    //Vote record to prevent from voting again
    #[account(
        init, 
        payer = signer,
        seeds = ["vote-record".as_bytes(), distribution_voting.key().as_ref(), signer.key().as_ref()], 
        space = Record::LEN,
        bump,
    )]
    pub vote_record: Account<'info, Record>,

    //Invited User's Sign
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}