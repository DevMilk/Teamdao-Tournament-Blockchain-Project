use crate::{entities::*, errors::Errors, constants::Constants, common::Common};
use anchor_lang::{prelude::*, AccountsClose, solana_program::{instruction, blake3::Hash}};
use std::{collections::{BTreeMap}, borrow::Borrow};
/* 
Accountlar Çoklu olarak nasıl gelecek?
her kullanıcı için Withdraw hesabı yapılsın mı yoksa tournament_participant_data'da mı dursun?
vote kısmında account info yeteecek mmi 
user'larda current_team'in ismi değil de pubkey'i tutulsa daha iyi olur 
(seed confirmation yerine address= yapılabilir YAPILAMAZ çünkü otomatik çekmez bu sefer)
heam team_name hem pubkey tutalım?
address = */

pub fn give_prize<'a,'b,'c,'info>(ctx: Context<'a,'b,'c,'info, GivePrize<'info>>, participant: Pubkey) -> Result<()> {    
    
    let tournament = &ctx.accounts.tournament;
    let participation_data =&ctx.accounts.tournament_participation_data;
    let team_manager = &ctx.accounts.signer;
    require!(ctx.remaining_accounts.len()>0, Errors::NoAccountGiven);
    
    let is_individual = ctx.remaining_accounts.len()==1;
    
    require!(team_manager.to_account_info().lamports() >= tournament.reward, Errors::AccountBalanceNotEnough);

    if is_individual {
        let winner = &ctx.remaining_accounts[0];
        require!(
            participant == winner.key(),
            Errors::GivenAccountNotMatchesWithTournamentParticipationData
        );        
        Common::transfer(
            team_manager, 
            winner, 
            tournament.reward
        )//reward as sol
    }
    else {
        let mut prize_distribution_map: BTreeMap<&Pubkey, f32> = BTreeMap::new();
        
        for i in 0..participation_data.members.len() {
            prize_distribution_map.insert(
                &participation_data.members[i], 
                participation_data.prize_distribution[i]
            );
        }
        require!(ctx.remaining_accounts.iter().all(|member| 
            prize_distribution_map.contains_key(&member.key())), 
            Errors::GivenAccountNotMatchesWithTournamentParticipationData
        );
        
        
        ctx.remaining_accounts.iter().for_each(|team_member| {
            let account_info = team_member;
            let prize_rate_as_percent = prize_distribution_map.get(account_info.key).unwrap_or(&0.0);

            
            if *prize_rate_as_percent != 0.0
             {

                Common::transfer(
                    &ctx.accounts.signer, 
                    team_member, 
                    (*prize_rate_as_percent * (tournament.reward as f32)) as u64,
                )

            }
        }

            
        
        )
        //DISTRIBUTE PRIZE
        //tournament.close(.to_account_info());
        
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
    pub tournament: Account<'info, Tournament>,


    #[account(
        mut,
        seeds = ["tournament-participation".as_bytes(), tournament.key().as_ref(), participant.as_ref()],
        bump = tournament_participation_data.bump,
        //close = participant_users[0],
    )]
    pub tournament_participation_data: Account<'info, TournamentParticipation>,

    //Tournament founder = manager's sign
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}