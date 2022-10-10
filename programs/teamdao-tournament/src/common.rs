pub mod Common {
    use anchor_lang::{prelude::{AccountInfo, Signer}, ToAccountInfo, solana_program::msg};

    pub fn transfer<'a>(from: &Signer<'a>, to: &AccountInfo<'a>, reward_as_lamport: u64){

        msg!(reward_as_lamport.to_string().as_str());
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            from.key,
            to.key,
            reward_as_lamport,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
             &[
                from.to_account_info(),
                to.to_account_info()
            ],
        ); 
    }
}
