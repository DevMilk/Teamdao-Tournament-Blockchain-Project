import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { assert } from 'console'
import { generateKeyPair, generateKeyPairSync } from 'crypto'
import { TeamdaoTournament } from '../target/types/teamdao_tournament'
import { LAMPORTS_PER_SOL, AccountMeta} from "@solana/web3.js";
import { expect } from 'chai'

const program = anchor.workspace.TeamdaoTournament as Program<TeamdaoTournament>

  //#region HELPER FUNCTIONS

  //generates new keypair, creates new user account with a balance and return keypair and PDA
  const generateUserAccount = async() : Promise<[anchor.web3.Keypair, anchor.web3.PublicKey]> => {
    //Generate keypair for new user
    const newUser = anchor.web3.Keypair.generate();
    
    //Airdrop Sols to new user
    await program.provider.connection.confirmTransaction(await program.provider.connection.requestAirdrop(newUser.publicKey, LAMPORTS_PER_SOL * 20));

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
  const createTeam = async(
    userKeypair : anchor.web3.Keypair, 
    name: string = generateRandomString()
  ): Promise<anchor.web3.PublicKey> => {
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
    userToInvitePubKey: anchor.web3.PublicKey
  ): Promise<anchor.web3.PublicKey> => {
    
    const ix = await program.methods
      .inviteToTeam(userToInvitePubKey)
      .accounts(
        {
          signer: teamAuthorityKeypair.publicKey
        })
      .signers([teamAuthorityKeypair]);
    await ix.pubkeys();
    const tx = await ix.rpc();

    return (await ix.pubkeys()).invitationProposal; 
  }

  //Leave current team
  const leaveCurrentTeam = async(
    userKeypair: anchor.web3.Keypair
  ) => {

      //const teamPDA = await program.account.team.fetch(await program.account.userAccount.)
      await program.methods
      .leaveTeam()
      .accounts(
        {
          //teamMember: userPDA,
          signer: userKeypair.publicKey
        }
      )
      .signers([userKeypair])
      .rpc();
    }

  //Answer team invite yes (true) or not (false)
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

    return isThrowedError;
  }

  //transfers team ownership
  const transferTeamOwnership = async(
    currentOwner : anchor.web3.Keypair, 
    newOwner: anchor.web3.PublicKey
  ): Promise<anchor.web3.PublicKey> => {
    const ix = await program.methods
      .transferTeamOwnership(newOwner)
      .accounts(
        {
          signer: currentOwner.publicKey
        })
      .signers([currentOwner]);

    const tx = await ix.rpc();
    return (await ix.pubkeys()).team; 
  }

  //generates team for given user account and team name and returns Team PDA
  const createTournament= async(
    userKeypair : anchor.web3.Keypair, 
    tournamentId : string = generateRandomString(0,8),
    tournamentName: string = generateRandomString(),
    tournamentReward: number = 5,
    maxParticipantNum : number = 20,
    ): Promise<anchor.web3.PublicKey> => {


    const ix = await program.methods
      .createTournament(tournamentId, tournamentName, new anchor.BN(tournamentReward),maxParticipantNum)
      .accounts(
        {
          signer: userKeypair.publicKey
        })
      .signers([userKeypair]);

    const tx = await ix.rpc();
    return (await ix.pubkeys()).newTournament; 
  }

  //generates a voting proposal to enter tournament or not
  const enterTournament= async(
    teamAuthorityKeypair : anchor.web3.Keypair, 
    tournamentPDA : anchor.web3.PublicKey,
    teamPDAOrIndividual: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {

    const ix = await program.methods
      .enterTournament(teamPDAOrIndividual)
      .accounts(
        {
          tournament: tournamentPDA,
          signer: teamAuthorityKeypair.publicKey
        })
      .signers([teamAuthorityKeypair]);

    const tx = await ix.rpc();
    return (await ix.pubkeys()).tournamentParticipationData; 
  }

  const voteTournamentParticipation= async(
    indvidualOrTeamAuthorityKeypair : anchor.web3.Keypair, 
    tournamentPDA : anchor.web3.PublicKey,
    answer: boolean
    ): Promise<anchor.web3.PublicKey> => {

    const ix = await program.methods
      .voteTournamentParticipation(answer)
      .accounts(
        {
          tournament: tournamentPDA,
          signer: indvidualOrTeamAuthorityKeypair.publicKey
        })
      .signers([indvidualOrTeamAuthorityKeypair]);

    const tx = await ix.rpc();
    return (await ix.pubkeys()).tournamentParticipationData; 
  }

  const createPrizeDistribution= async(
    teamOwner : anchor.web3.Keypair, 
    tournamentPDA : anchor.web3.PublicKey,
    prizeDistribution: number[],
    ): Promise<anchor.web3.PublicKey> => {

    const ix = await program.methods
      .createPrizeDistribution(tournamentPDA, prizeDistribution, generateRandomString(0,5))
      .accounts(
        {
          signer: teamOwner.publicKey
        })
      .signers([teamOwner]);

    const tx = await ix.rpc();
    return (await ix.pubkeys()).distributionVoting; 
  }

  const votePrizeDistribution= async(
    participant : anchor.web3.Keypair, 
    tournamentPDA : anchor.web3.PublicKey,
    prizeDistributionPDA: anchor.web3.PublicKey,
    answer: boolean
    ): Promise<anchor.web3.PublicKey> => {

    const ix = await program.methods
      .votePrizeDistribution(answer, tournamentPDA)
      .accounts(
        {
          distributionVoting: prizeDistributionPDA,
          signer: participant.publicKey
        })
      .signers([participant]);

    const tx = await ix.rpc();
    return (await ix.pubkeys()).tournamentParticipationData; 
  }

  const givePrize= async(
    tournamentManager : anchor.web3.Keypair, 
    winnerIndividualOrTeam : anchor.web3.PublicKey,
    tournamentPDA: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
      

    const team = await program.account.team.fetchNullable(winnerIndividualOrTeam).catch(err=>undefined);
    const winnerAccounts = team ? team.members : [winnerIndividualOrTeam]; 
    const winnerAccountInfos = winnerAccounts
      .map(account=> {
        const meta: AccountMeta = {pubkey: account, isWritable: true, isSigner: false}
        return meta;
      }
    );
    console.log(winnerAccountInfos);
    const ix = await program.methods
      .givePrize(winnerIndividualOrTeam)
      .accounts(
        {
          tournament: tournamentPDA,
          signer: tournamentManager.publicKey
        })
      .remainingAccounts(winnerAccountInfos)
      .signers([tournamentManager]);

    const tx = await ix.rpc();
    return (await ix.pubkeys()).tournamentParticipationData; 
  }
  //generates random string (for random team name, tournament name etc.)
  const generateRandomString = (start = 2, end = 8) : string => 
    Math.random().toString(36).slice(start,end);

  //#endregion
  /*
describe('teamdao-tournament', () => {
  
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
    const isThrowedError = await isThrowsError( program.account.userAccount.fetch(userAccountAddress) );
    expect(isThrowedError).to.equal(true);

  })

  it('Can create a new team', async () => {
    //create user
    const [userKeypair, userAccountPDA] = await generateUserAccount();

    //create team
    const teamName = generateRandomString();
    const teamPDA = await createTeam(userKeypair,teamName);

    const team = await program.account.team.fetch(
      teamPDA
    )
    expect(team.authority == userKeypair.publicKey);

  })

  it('Throws error if user creates new team if user is already a team member', async () => {
    const [userKeypair, userAccountPDA] = await generateUserAccount();
    const teamName = generateRandomString();
    await createTeam(userKeypair,teamName);

    const isThrowedError = await isThrowsError( createTeam(userKeypair,"team2") );
    expect(isThrowedError).to.equal(true);

  })
  
  it('Can allow user to invite to team, invited member can join the team, team owner can transfer ownership', async () => {
    const [teamCaptain, teamCaptainPDA] = await generateUserAccount();
    const [userToInviteKeypair, __] = await generateUserAccount();

    const teamName = generateRandomString();
    const teamPDA = await createTeam(teamCaptain,teamName);


    await inviteToTeam(teamCaptain,userToInviteKeypair.publicKey);
    await AnswerProposal(userToInviteKeypair,teamPDA,true);
    await transferTeamOwnership(teamCaptain, userToInviteKeypair.publicKey);

    const teamData = await program.account.team.fetch(teamPDA);
    const foundMember = teamData.members.map(x=>x.toString()).includes(userToInviteKeypair.publicKey.toString());

    expect(foundMember).to.equal(true);
    expect(teamData.authority.toString()).to.equal(userToInviteKeypair.publicKey.toString());

  })

  it('Can allow user to leave from team and if there is no member left then close team', async () => {
    const [teamCaptain, teamCaptainPDA] = await generateUserAccount();

    const teamName = generateRandomString();
    const teamPDA = await createTeam(teamCaptain,teamName);

    const teamData = await program.account.team.fetch(teamPDA);
    const foundMemberBeforeLeave = teamData.members.map(x=>x.toString()).includes(teamCaptain.publicKey.toString());

    await leaveCurrentTeam(teamCaptain);
    const isThrowedError = await isThrowsError( program.account.team.fetch(teamPDA ));
    expect(isThrowedError && foundMemberBeforeLeave).to.equal(true);

  })

  


})
*/
describe("Tournament Test", ()=> {

  
  it('Create tournament, get participants, select winner by distributing prize', async () => {
    
    //---CREATE TOURNAMENT---
    const [tournamentManager, _] = await generateUserAccount();

    //Parameters
    const tournamentId = generateRandomString(0,8);
    const tournamentName = generateRandomString();
    const tournamentRewardAsLamport = 3 * LAMPORTS_PER_SOL;
    const maxParticipantNum = 10;


    //Get created tournament
    const tournamentPDA = await createTournament(tournamentManager, tournamentId, tournamentName, tournamentRewardAsLamport,maxParticipantNum);


    //CHECK: if tournamentData created correctly
    const tournamentData = await program.account.tournament.fetch(tournamentPDA);
    expect(tournamentData.tournamentId).to.equal(tournamentId);
    expect(tournamentData.tournamentName).to.equal(tournamentName);
    expect(tournamentData.reward.toNumber()).to.equal(tournamentRewardAsLamport);
    expect(tournamentData.manager.toString()).to.equal(tournamentManager.publicKey.toString());



    //---CREATE TEAM PARTICIPANTS---
    const [teamCap, ll] = await generateUserAccount();
    const [teamMember, ___] = await generateUserAccount();
    //Get created tournament with default parameters
    const teamPDA = await createTeam(teamCap);    
    await inviteToTeam(teamCap,teamMember.publicKey);
    await AnswerProposal(teamMember,teamPDA, true);



    //---CREATE INDIVIDUAL PARTICIPANT---
    const [individualParticipant, l] = await generateUserAccount();




    //---TOURNAMENT ENTRANCE---

    //Entering as a team
    const tournamentProposalPDA = await enterTournament(teamCap, tournamentPDA, teamPDA);
    await voteTournamentParticipation(teamCap, tournamentPDA, true);
    await voteTournamentParticipation(teamMember, tournamentPDA, true);

    //Entering as Individual
    await enterTournament(individualParticipant, tournamentPDA, individualParticipant.publicKey);

    
    const tournament = await program.account.tournament.fetch(tournamentPDA);
    const tournamentParticipantList = tournament.participants.map(participant => participant.toString());
    //CHECK: Is team registered in tournament?
    const isTeamInTournament = tournamentParticipantList.includes(teamPDA.toString());
    expect(isTeamInTournament).to.be.equal(true);
    
    //CHECK: Is individual player registered in tournament?
    const isIndividualInTournament = tournamentParticipantList.includes(individualParticipant.publicKey.toString());
    expect(isIndividualInTournament).to.be.equal(true);

    console.log("All Entered tournament")

    const prize_dist = [0.1, 0.9];
    const prizeDistPDA = await createPrizeDistribution(teamCap, tournamentPDA, prize_dist);
    console.log("vote")
    await votePrizeDistribution(teamCap, tournamentPDA, prizeDistPDA, true);
    console.log("vote")
    await votePrizeDistribution(teamMember, tournamentPDA, prizeDistPDA, true);

    const balanceBeforePrize = await program.provider.connection.getBalance(teamMember.publicKey)
    console.log(await program.provider.connection.getBalance(tournamentManager.publicKey));
    await givePrize(tournamentManager, teamPDA, tournamentPDA);
    console.log("ok");
    const balanceAfterPrize = await program.provider.connection.getBalance(teamMember.publicKey);
    expect(balanceBeforePrize).to.equal(balanceAfterPrize - tournamentRewardAsLamport * prize_dist[1]);
  })


})
