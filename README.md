# Teamdao Tournament Blockchain Program

## Installation and Testing

### Package Installation
- ``` yarn ```
- ``` npm install ```

### Building, Deploying & Testing Program
- Get the program ID by using command on terminal: 
``` 
     $ anchor keys list
```
Here, make sure you update your program ID in **Anchor.toml** and **lib.rs**. You can use ``` anchor deploy ``` to get valid programId

- Build, Start Clustering, Deploy and Test in single command line:
``` 
     $ anchor test
```
If program not found, check programId is invalid or not.


## Features

User can:
- Create an account
- Delete an account
- Create a team
- Invite new members to team
- Transfer ownership of team
- Join a team by getting invited
- Create a tournament
- Distribute prize of tournament he/she created by selecting winner team or individual of tournament
- Enter a tournament as an individual participant
- Enter a tournament with team after voting for tournament participation initialized by team owner
- Change prize distribution by getting enough votes by participants (Default prize distribution is equal share)

## Rules
- User must be in team during voting gets finished to be a participant as a team in a tournament
- Votes are accepted if 2/3 of members/participants voted
- Prize must be given in tournament creation by transfering prize to tournament PDA account, SOLs are stored in PDA until tournament ends and it cant be changed during that time so that users can be entirely sure that tournament prize exists.
- When creating a tournament, manager must specify the maximum number of participants (team is considered as 1 participant) and prize
- Team owner cant transfer ownership to non-team member
- A team member can be invited by other teams but cant accept it without leaving current team
- Users cant join a team without reference (invitation) 
- Only team owners can send team invitations
- Team names are unique and their length must between 5 and 30 characters
- A user can be a participant of multiple tournaments but cant be a member of multiple teams

## WHAT IF CASES

WHAT IF:

- After tournament finished, prize winner teams' member accounts array gets to give_prize instruction in different order from the participant data?

  ``` Account input order doesn't matter on prize distribution because participant data both stores prize distribution array and member order of prize distribution (It is initialized in last vote before entering tournament as a team and cant be changed). When distributing prize, BTreeMap is created from participant data, algorithm finds input accounts' prize share from BTreeMap with O(1) complexity.    ```

- Prize distribution's sum is not equal to %100 ?

  ``` It is checked in a constraint. ```

- There is 12 member in team, 8 of them voted as yes to participate in tournament X and others are not voted?

  ``` Team enters tournament. It is not needed to wait for others because yes vote count already bigger or equal than 2/3 of all members. ```

- User creates user accounts again?

  ``` It is not allowed to create user account again because PDA is getting only its public key as seed (O(1) complexity) ```

- User creates team even he/she already is a team member?

  ```  It is not allowed to create a team if user is registered in a team, there is constraint for it. (O(1) complexity) ```

- User tries to invite a user that already in same team?

  ``` It is allowed to invite someone from other teams but there is a constraint that checks if invited user already in same team (by checking current_team name on invited user) with (O(1) complexity) ```

- User tries to invite a user who dont have user account?

  ```Seeds will give error during AutoPopulation. (O(1)) ```

- Other duplicate problems occurs (sending invitation again, same person tries to vote again, same team tries to enter tournament again while voting is existing, trying to give prize again etc.) ?

  ``` All this cases are handled using PDAs like a HashMap with O(1) complexity too. ```

- Someone want to create a big tournament for 500+ people while solana stack can only carry up to 10Kb?

  ``` Memory issues are considered for accounts that contains scalable data structures with big size potential such as Vector of Pubkeys etc. Tournament accounts are defined in context as a Box (Box<Account<'info, Tournament>>) for storing big data on heap and using the data on stack to point it because in solana, account stack can only keep up to 10Kb for PDA accounts. ```

## Program Design & File Structure
Instructions are designed to minimize number of account inputs directly from user by taking advantage of AutoPopulation features on project-serum/anchor and reducing boilerplate codes.

File structure decouples instructions and accounts to make code easy to read and understand.

- **Entities** (team, user_account, etc.) listed in ``` entities ``` folder. It also contains space of entities.
- **Instruction** structure and business logics of instructions listed in  ``` instructions ``` folder.
- **Common** functions (functions shared between multiple instructions) are defined in ``` common.rs ``` file.
- **Constants** (Maximum team name length, Lamport per sol etc.) are defined in ```constants.rs ``` file.
- **Errors** defined in ```errors.rs``` file.
- **Entrypoints and Program Id** defined in ```lib.rs``` file.