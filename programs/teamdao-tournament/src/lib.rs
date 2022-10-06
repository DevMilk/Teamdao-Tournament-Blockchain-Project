use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod errors;
pub mod structs;

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

    pub fn invite_to_team(ctx: Context<InviteToTeam>) -> Result<()> {
        return instructions::invite_to_team::invite_to_team(ctx);
    }

    pub fn answer_proposal(ctx: Context<AnswerProposal>, answer: bool) -> Result<()> {
        return instructions::answer_proposal::answer_proposal(ctx, answer);
    }
}