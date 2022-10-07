import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { assert } from 'console'
import { generateKeyPair, generateKeyPairSync } from 'crypto'
import { TeamdaoTournament } from '../target/types/teamdao_tournament'
import { LAMPORTS_PER_SOL} from "@solana/web3.js";
describe('teamdao-tournament', () => {
  /*const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)*/

  
  const generateAccount = async (): Promise<anchor.web3.Keypair> => {
    const keypair = anchor.web3.Keypair.generate();
    await program.provider.connection.confirmTransaction(await program.provider.connection.requestAirdrop(keypair.publicKey, LAMPORTS_PER_SOL * 10));
    return keypair;

  }
  const generateUserAccount = async() : Promise<[anchor.web3.Keypair, anchor.web3.PublicKey]> => {
    const newUser = await generateAccount()
    const ix = await program.methods.createUserAccount().accounts({signer: newUser.publicKey}).signers([newUser]);
    
    const userAccountAddress = (await ix.pubkeys()).userAccount;

    const tx = await ix.rpc()

    return [newUser,userAccountAddress];
  }
  const program = anchor.workspace.TeamdaoTournament as Program<TeamdaoTournament>

  
  it('Creating a new account for user', async () => {
    const [_, userAccountAddress] = await generateUserAccount();

    // User Details
    let userDetails = await program.account.userAccount.fetch(
      userAccountAddress
    )
  })

  it('Close My User Account', async () => {
    const [keypair, userAccountAddress] = await generateUserAccount();

    const ix = await program.methods.deleteAccount().accounts({signer: keypair.publicKey}).signers([keypair])
    const userAddress = (await ix.pubkeys()).userAccount
    console.log('User Address :: ', userAddress.toString())

    // Create user's account address
    const tx = await ix.rpc()
    console.log('Your transaction signature', tx)

    // It will return error if it doesn't find an account 
    try {
      await program.account.userAccount.fetch(
        userAccountAddress
      )
      assert(false);
    } catch {
      console.log("Closed User Account")
    }
  })
})
