import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { assert } from 'console'
import { generateKeyPair, generateKeyPairSync } from 'crypto'
import { TeamdaoTournament } from '../target/types/teamdao_tournament'
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
describe('teamdao-tournament', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.TeamdaoTournament as Program<TeamdaoTournament>

  
  it('Creating a new account for user', async () => {
    
    let ix = await program.methods.createUserAccount();
    
    /*
    const newUser = anchor.web3.Keypair.generate();
    await program.provider.connection.requestAirdrop(
      newUser.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 10
    );
    ix = ix.accounts({
      user_account: newUser.publicKey,
    }).signers([newUser]);
    // Works in here: https://github.com/coral-xyz/anchor-by-example/blob/master/programs/onchain-voting/programs/onchain-voting/src/lib.rs
    */
   
    const userAccountAddress = (await ix.pubkeys()).userAccount;
    console.log('User account address :: ', userAccountAddress.toString())

    // Create user's facebook address
    const tx = await ix.rpc()
    console.log('Your transaction signature', tx)

    // User Details
    let userDetails = await program.account.userAccount.fetch(
      userAccountAddress
    )
  })

  it('Close My Facebook Account', async () => {
    const ix = await program.methods.deleteAccount()
    const userAccountAddress = (await ix.pubkeys()).userAccount
    console.log('usrFaceBook Address :: ', userAccountAddress.toString())

    // Create user's account address
    const tx = await ix.rpc()
    console.log('Your transaction signature', tx)

    // It will return error if it doesn't find an account 
    try {
      let userDetails = await program.account.userAccount.fetch(
        userAccountAddress
      )
      assert(false);
    } catch {
      console.log("Closed User Account")
    }
  })
})
