use crate::{entities::*, errors::Errors, constants::Constants, common::Common};
use anchor_lang::{prelude::*, AccountsClose, solana_program::{instruction, blake3::Hash}};
use std::{collections::HashMap, borrow::Borrow};
/* 
Accountlar Çoklu olarak nasıl gelecek?
her kullanıcı için Withdraw hesabı yapılsın mı yoksa tournament_participant_data'da mı dursun?
vote kısmında account info yeteecek mmi 
user'larda current_team'in ismi değil de pubkey'i tutulsa daha iyi olur 
(seed confirmation yerine address= yapılabilir YAPILAMAZ çünkü otomatik çekmez bu sefer)
heam team_name hem pubkey tutalım?
address = */

pub fn give_prize<'a,'b,'c,'info>(ctx: Context<'a,'b,'c,'info, GivePrize<'info>>, participant: Pubkey) -> Result<()> {    

    msg!("1");
    
    let tournament = &ctx.accounts.tournament;
    let participation_data =&ctx.accounts.tournament_participation_data;

    require!(ctx.remaining_accounts.len()>0, Errors::NoAccountGiven);
    
    let is_individual = ctx.remaining_accounts.len()==1;
    
    if is_individual {
        let winner = &ctx.remaining_accounts[0];
        require!(
            participant == winner.key(),
            Errors::GivenAccountNotMatchesWithTournamentParticipationData
        );        
        Common::transfer(
            &ctx.accounts.signer, 
            winner, 
            (tournament.reward as u64) * Constants::LAMPORTS
        )//reward as sol
    }
    else {
        msg!("2");
        let mut prize_distribution_map: HashMap<&Pubkey, u8> = HashMap::new();
        for (index, member) in participation_data.members.iter().enumerate() {
            prize_distribution_map.insert(member, participation_data.prize_distribution[index]);
        }
        
        require!(ctx.remaining_accounts.iter().all(|member| 
            prize_distribution_map.contains_key(&member.key())), 
            Errors::GivenAccountNotMatchesWithTournamentParticipationData
        );
        
        msg!("3");
        ctx.remaining_accounts.iter().for_each(|team_member| {
            msg!("4");
            let account_info = team_member;
            let prize_rate = match prize_distribution_map.get(account_info.key) {
                Some(prize_rate) => *prize_rate,
                None => 0
            };
            msg!("5");
            if prize_rate != 0 {

                let prize_of_member = (prize_rate as u64) * Constants::LAMPORTS / 100;

                Common::transfer(
                    &ctx.accounts.signer, 
                    team_member, 
                    (tournament.reward as u64) * Constants::LAMPORTS
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