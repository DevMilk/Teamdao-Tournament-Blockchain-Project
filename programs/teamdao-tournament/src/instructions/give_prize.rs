use crate::{entities::*, errors::Errors, common::common};
use anchor_lang::{prelude::*, AccountsClose};
use std::{collections::{BTreeMap}};


pub fn give_prize<'a,'b,'c,'info>(ctx: Context<'a,'b,'c,'info, GivePrize<'info>>, participant: Pubkey) -> Result<()> {    
    
    let tournament = &ctx.accounts.tournament;
    let participation_data =&ctx.accounts.tournament_participation_data;
    let tournament_manager = &ctx.accounts.signer;

    //Accounts of prize distribution must be given in remaining accounts
    require!(ctx.remaining_accounts.len()>0, Errors::NoAccountGiven);
    
    //Check if winner is an individual participant or team
    let is_individual = ctx.remaining_accounts.len()==1;

    //If participant is an individual participant, then transfer all reward directly
    if is_individual {
        let winner = &ctx.remaining_accounts[0];

        //Check if given account is the winner
        require!(
            participant == winner.key(),
            Errors::GivenAccountNotMatchesWithTournamentParticipationData
        );        
        common::transfer_from_signer(
            tournament_manager, 
            winner, 
            tournament.reward
        )//reward as sol
    }
    //If it is a team participation, then transfer rewards by share defined in participation data.
    //In case of accounts are given in different order from participant data, we use treeMap to make sure we send correctly. 
    //If someone leaved team after joining tournament, he/she still get share
    else {
        //Create treeMap of member-prize_share pairs
        let mut prize_distribution_map: BTreeMap<&Pubkey, f32> = BTreeMap::new();
        
        for i in 0..participation_data.members.len() {
            prize_distribution_map.insert(
                &participation_data.members[i], 
                participation_data.prize_distribution[i]
            );
        }

        //Check there is no one in remaining account that not participated in tournament
        require!(ctx.remaining_accounts.iter().all(|member| 
            prize_distribution_map.contains_key(&member.key())), 
            Errors::GivenAccountNotMatchesWithTournamentParticipationData
        );
        
        
        //Distribute rewards to team participants by share
        ctx.remaining_accounts.iter().for_each(|team_member| {
            let account_info = team_member;
            let prize_rate_as_percent = prize_distribution_map.get(account_info.key).unwrap_or(&0.0);
            
            if *prize_rate_as_percent != 0.0
             {

                common::transfer(
                    &tournament.to_account_info(),//&ctx.accounts.signer, 
                    team_member, 
                    (*prize_rate_as_percent * (tournament.reward as f32)) as u64,
                );
            }
        }
        );

        //Close tournament account and send remaining sols to tournament manager 
        tournament.close(tournament_manager.to_account_info());
        
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(participant: Pubkey)]
pub struct GivePrize<'info> {

    //Must be given
    #[account(
        mut, 
        constraint = tournament.manager == signer.key(),
        close = signer
    )] 
    pub tournament: Box<Account<'info, Tournament>>,


    //Tournament participation data of winner participant or team
    #[account(
        mut,
        seeds = ["tournament-participation".as_bytes(), tournament.key().as_ref(), participant.as_ref()],
        bump = tournament_participation_data.bump,
        //Check if participant is entered tournament or not (There can be teams that created proposal but not entered)
        constraint = tournament_participation_data.is_entered @ Errors::TeamNotInTournament
    )]
    pub tournament_participation_data: Account<'info, TournamentParticipation>,

    //Tournament founder = manager = prize giver's sign
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}