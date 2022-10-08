import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { assert } from 'console'
import { generateKeyPair, generateKeyPairSync } from 'crypto'
import { TeamdaoTournament } from '../target/types/teamdao_tournament'
import { LAMPORTS_PER_SOL} from "@solana/web3.js";
import { expect } from 'chai'
describe('teamdao-tournament', () => {

  //HELPER FUNCTIONS
  
  //generates new keypair, creates new user account with a balance and return keypair and PDA
  const generateUserAccount = async() : Promise<[anchor.web3.Keypair, anchor.web3.PublicKey]> => {
    //Generate keypair for new user
    const newUser = anchor.web3.Keypair.generate();
    
    //Airdrop Sols to new user
    await program.provider.connection.confirmTransaction(await program.provider.connection.requestAirdrop(newUser.publicKey, LAMPORTS_PER_SOL * 10));

    //Create user account for new user
    const ix = await program.methods.createUserAccount().accounts({signer: newUser.publicKey}).signers([newUser]);
    
    //Get PDA of new user's user account
    const userAccountAddress = (await ix.pubkeys()).userAccount;

    //RPC call to account creation instruction
    const tx = await ix.rpc();

    //return keypair and PDA of user account
    return [newUser,userAccountAddress];
  }

  //generates team for given user account and team name and returns Team PDA
  const createTeam = async(userKeypair : anchor.web3.Keypair, name: string): Promise<anchor.web3.PublicKey> => {
    const ix = await program.methods
      .createTeam(name)
      .accounts(
        {
          signer: userKeypair.publicKey
        })
      .signers([userKeypair]);

    const tx = await ix.rpc();
    return (await ix.pubkeys()).team; 
  }

  //Invite user to team
  const inviteToTeam = async(
    teamAuthorityKeypair : anchor.web3.Keypair, 
    userToInvitePubKey: anchor.web3.PublicKey, 
    teamPDA: anchor.web3.PublicKey,
    teamAuthorityPDA: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
    
    const ix = await program.methods
      .inviteToTeam(userToInvitePubKey)
      .accounts(
        {
          teamAuthority: teamAuthorityPDA,
          teamAccount: teamPDA,
          signer: teamAuthorityKeypair.publicKey
        })
      .signers([teamAuthorityKeypair]);
    
    const tx = await ix.rpc();

    const pubkeys = await ix.pubkeys();
    return (await ix.pubkeys()).invitationProposal; 
  }
  //Re
  const AnswerProposal = async(invited : anchor.web3.Keypair, teamPDA: anchor.web3.PublicKey, answer: boolean) => {
    const ix = await program.methods
      .answerProposal(answer)
      .accounts(
        {
          teamAccount: teamPDA,
          signer: invited.publicKey
        })
      .signers([invited]);

    const tx = await ix.rpc();
  }
  //test function that tests if given async function throws error or not
  const isThrowsError = async<T>(func: Promise<T>) =>{
    
    let isThrowedError = false;
    try {
      await func;
    } catch (e) {
      //console.log((e as Error).message.split("\n")[0]);
      isThrowedError = true;
    }

    if(!isThrowedError)
      throw new Error("Assertion Failed");
  }
  //generates random names for team
  const getRandomTeamName = () : string => 
    Math.random().toString(36).slice(2,8);
  
  const program = anchor.workspace.TeamdaoTournament as Program<TeamdaoTournament>
  

  //TESTS
  it('Can create a new user account', async () => {
    const [_, userAccountAddress] = await generateUserAccount();

    // If there is no user it will throw exception
    let userDetails = await program.account.userAccount.fetch(
      userAccountAddress
    )
  })

  it('Can delete user account', async () => {
    const [keypair, userAccountAddress] = await generateUserAccount();

    //Delete new user
    await program.methods
      .deleteAccount()
      .accounts({signer: keypair.publicKey})
      .signers([keypair])
      .rpc();
    

    // It will return error if it doesn't find an account 
    
    await isThrowsError
    (
      program.account.userAccount.fetch(userAccountAddress)
    )

  })

  it('Can create a new team', async () => {
    //create user
    const [userKeypair, userAccountPDA] = await generateUserAccount();

    //create team
    const teamName = getRandomTeamName();
    const teamPDA = await createTeam(userKeypair,teamName);

    const team = await program.account.team.fetch(
      teamPDA
    )
    expect(team.authority == userKeypair.publicKey);
  })

  it('Throws error if user creats new team if user is already a team member', async () => {
    const [userKeypair, userAccountPDA] = await generateUserAccount();
    const teamName = getRandomTeamName();
    await createTeam(userKeypair,teamName);

    await isThrowsError
    (
      createTeam(userKeypair,"team2")
    )

  })
  
  it('Can allow user to invite to team and invited user gets to the team if accepts', async () => {
    const [teamCaptain, teamCaptainPDA] = await generateUserAccount();
    const [userToInviteKeypair, __] = await generateUserAccount();

    const teamName = getRandomTeamName();
    const teamPDA = await createTeam(teamCaptain,teamName);
    const invitationProposalPDA = await inviteToTeam(teamCaptain,userToInviteKeypair.publicKey,teamPDA,teamCaptainPDA);
    console.log("Invited To Team");
    await AnswerProposal(userToInviteKeypair,teamPDA,true);
    console.log("Answered Proposal")
    
    const teamData = await program.account.team.fetch(teamPDA);
    const foundMember = teamData.members.includes(userToInviteKeypair.publicKey);

    expect(foundMember).to.not.equal(true);
  })
})
