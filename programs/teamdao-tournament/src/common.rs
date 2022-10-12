pub mod common {
    use anchor_lang::{prelude::{AccountInfo, Signer}, ToAccountInfo};

    pub fn transfer_from_signer<'a>(from: &Signer<'a>, to: &AccountInfo<'a>, reward_as_lamport: u64){

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            from.key,
            to.key,
            reward_as_lamport 
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
             &[
                from.to_account_info(),
                to.to_account_info()
            ],
        ); 
    }

    pub fn transfer<'a>(from: &AccountInfo<'a>, to: &AccountInfo<'a>, reward_as_lamport: u64) -> anchor_lang::solana_program::entrypoint::ProgramResult{

        **from.try_borrow_mut_lamports()? -= reward_as_lamport;
        **to.try_borrow_mut_lamports()? += reward_as_lamport;
        Ok(())
    }
        
}
