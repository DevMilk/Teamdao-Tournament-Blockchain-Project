use crate::entities::*;
use crate::errors::*;
use anchor_lang::prelude::*;



pub fn create_prize_distribution(ctx: Context<CreatePrizeDistribution>, _tournament_key: Pubkey, distribution: Vec<f32>, _id: String) -> Result<()> {    
    //Check distribution is valid
    require!(distribution.iter().sum::<f32>() == 1.0, Errors::InvalidPrizeDistribution);

    let distribution_voting = &mut ctx.accounts.distribution_voting;
    
    distribution_voting.bump = *ctx.bumps.get("distribution_voting").unwrap();
    distribution_voting.prize_distribution = distribution;


    Ok(())
}

#[derive(Accounts)]
#[instruction(tournament_key: Pubkey, distribution: Vec<f32>, id: String)]
pub struct CreatePrizeDistribution<'info> {

    //Team owner that creating prize distribution voting (Auto)
    #[account(
        mut,
        seeds = ["user-account".as_bytes(), signer.key().as_ref()],
        bump = team_owner.bump,
        constraint = team_owner.is_authority @ Errors::NonAuthorizedVotingCreation
    )]
    pub team_owner: Account<'info, UserAccount>,

    //Team Account (Auto)
    #[account(
        mut, 
        seeds = ["team".as_bytes(), team_owner.current_team.as_bytes()], 
        bump = team_account.bump,
    )]
    pub team_account: Account<'info, Team>,

    //Tournament participation data (Auto)
    #[account(
        mut,
        seeds = [
            "tournament-participation".as_bytes(), 
            tournament_key.as_ref(), 
            //If it is individual (non-team member) then pass its pubkey if it is not then pass team_addr
            team_account.key().as_ref()
        ],
        bump = tournament_participation_data.bump,
        //Note: If someone is joined the team after team joined tournament, user cant vote or get share from tournament

    )]
    pub tournament_participation_data: Account<'info, TournamentParticipation>,

    //distribution voting what increases ok
    #[account(
        init, 
        space = DistributionVoting::CONSTANT_LEN + (4 + 4 * tournament_participation_data.members.len()),
        payer = signer,
        seeds = [
            "distribution-voting".as_bytes(), 
            tournament_participation_data.key().as_ref(), 
            id.as_bytes()
        ], 
        bump,
    )]
    pub distribution_voting: Account<'info, DistributionVoting>,

    //Team owner's sign
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}