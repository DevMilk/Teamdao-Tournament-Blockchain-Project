import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
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

//Full Story of Tournament
/*
Pegasus: Tournament Manager, 
Yugi: Team Owner, 
Joey: Team Member, 
Kaiba: Individual Participant   

Evertone creates account and gets airdrop. 
Pegasus creates a tournament with 3 SOL prize. (3 SOL is withdrawn from its account to tournament account) 

Yugi creates a team.
Yugi invites Joey to his team.
Joey accepts team invitation and joins team.

Yugi creates a participation voting to join Pegasus's tournament.
Yugi votes as yes in participation voting.
Joey votes as yes in participation voting so Yugi's team joins Pegasus's tournament.

Kaiba joins tournament individually

Yugi doesn't care money so he creates a prize distribution with %10 share of himself and %90 share of joey.
Yugi votes as yes for prize distribution voting.
Joey votes as yes for prize distribution voting and prize distribution changes.

Pegasus is an honest person (An oracle in blockchain term), he 
admits that Yugi's team won tournament so he specify Yugi's team as winner and prize distributed to Yugi as 0.3 SOL, Joey as 2.7 SOL.


Kaiba is so rich and cool that he doesn't even care prize.
*/
describe("Tournament Test", ()=> {

  
  it('Create tournament, get participants, select winner by distributing prize', async () => {
    
    //---CREATE USER ACCOUNTS (CHARACTERS)---
    const [pegasus, _] = await generateUserAccount();
    const [yugi, __] = await generateUserAccount();
    const [joey, ___] = await generateUserAccount();
    const [kaiba, ____] = await generateUserAccount();

    //--- CREATE TOURNAMENT ---
    const tournamentId = generateRandomString(0,8);
    const tournamentName = generateRandomString();
    const tournamentRewardAsLamport = 3 * LAMPORTS_PER_SOL;
    const maxParticipantNum = 10;


    //Get created tournament
    const tournamentPDA = await createTournament(pegasus, tournamentId, tournamentName, tournamentRewardAsLamport,maxParticipantNum);
    console.log("Pegasus created tournament.");

    //CHECK: if tournamentData created correctly
    const tournamentData = await program.account.tournament.fetch(tournamentPDA);
    expect(tournamentData.tournamentId).to.equal(tournamentId);
    expect(tournamentData.tournamentName).to.equal(tournamentName);
    expect(tournamentData.reward.toNumber()).to.equal(tournamentRewardAsLamport);
    expect(tournamentData.manager.toString()).to.equal(pegasus.publicKey.toString());



    //---CREATE TEAM PARTICIPANTS---
    
    //Get created tournament with default parameters
    const teamPDA = await createTeam(yugi);    
    console.log("Yugi created team.")

    await inviteToTeam(yugi,joey.publicKey);
    console.log("Yugi Invited Joey to join his team.")

    await AnswerProposal(joey,teamPDA, true);
    console.log("Joey accepted team invitation of Yugi")


    //---TOURNAMENT ENTRANCE---

    //Entering as a team
    await enterTournament(yugi, tournamentPDA, teamPDA);
    console.log("Yugi attempts to enter Pegasus's tournament as team, now he needs votes of team members to enter")

    await voteTournamentParticipation(yugi, tournamentPDA, true);
    console.log("Yugi votes as yes to tournament participation")

    await voteTournamentParticipation(joey, tournamentPDA, true);
    console.log("Joey votes as yes to tournament participation")

    //Entering as Individual
    await enterTournament(kaiba, tournamentPDA, kaiba.publicKey);
    console.log("Kaiba enters tournament individually")
    
    const tournament = await program.account.tournament.fetch(tournamentPDA);
    const tournamentParticipantList = tournament.participants.map(participant => participant.toString());

    //CHECK: Is team registered in tournament?
    const isTeamInTournament = tournamentParticipantList.includes(teamPDA.toString());
    expect(isTeamInTournament).to.be.equal(true);
    
    //CHECK: Is individual player registered in tournament?
    const isIndividualInTournament = tournamentParticipantList.includes(kaiba.publicKey.toString());
    expect(isIndividualInTournament).to.be.equal(true);

    const prize_dist = [0.1, 0.9];
    const prizeDistPDA = await createPrizeDistribution(yugi, tournamentPDA, prize_dist);
    console.log("Yugi creates prize distribution voting for tournament with %10 as his own share");

    await votePrizeDistribution(yugi, tournamentPDA, prizeDistPDA, true);
    console.log("Yugi votes as yes to prize distribution voting");

    await votePrizeDistribution(joey, tournamentPDA, prizeDistPDA, true);
    console.log("Joey votes as yes to prize distribution voting, prize distribution changed");

    const balanceBeforePrize = await program.provider.connection.getBalance(joey.publicKey)

    await givePrize(pegasus, teamPDA, tournamentPDA);
    console.log("Pegasus admits Yugi's team as winner, prize automatically distributed to Yugi and Joey");

    const balanceAfterPrize = await program.provider.connection.getBalance(joey.publicKey);
    expect(balanceBeforePrize).to.equal(balanceAfterPrize - tournamentRewardAsLamport * prize_dist[1]);
    console.log("Done");
  })


})
