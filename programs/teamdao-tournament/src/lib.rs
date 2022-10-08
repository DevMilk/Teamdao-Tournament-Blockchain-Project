use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod errors;
pub mod entities;
pub mod constants;

declare_id!("Anm8zfCbBFfZ4sWC3qRX5KKMpFdr7cLHbNGq1EKerACC");

#[program]
pub mod teamdao_tournament {

    use super::*;

    pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {
        return instructions::create_user_account::create_user_account(ctx);
    }

    pub fn delete_account(ctx: Context<DeleteAccount>) -> Result<()> {
        return instructions::delete_account::delete_account(ctx);
    }

    pub fn create_team(ctx: Context<CreateTeam>, team_name: String) -> Result<()> {
        return instructions::create_team::create_team(ctx, team_name);
    }

    pub fn invite_to_team(ctx: Context<InviteToTeam>, invited_pubkey: Pubkey) -> Result<()> {
        return instructions::invite_to_team::invite_to_team(ctx, invited_pubkey);
    }

    pub fn answer_proposal(ctx: Context<AnswerProposal>, answer: bool) -> Result<()> {
        return instructions::answer_proposal::answer_proposal(ctx, answer);
    }

    pub fn leave_team(ctx: Context<LeaveTeam>) -> Result<()> {
        return instructions::leave_team::leave_team(ctx);
    }

    pub fn create_tournament(ctx: Context<CreateTournament>, tournament_id: String, tournament_name: String,  reward: u16) -> Result<()> {
        return instructions::create_tournament::create_tournament(ctx, tournament_id, tournament_name, reward );
    }
}