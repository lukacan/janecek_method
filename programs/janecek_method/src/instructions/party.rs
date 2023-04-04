use crate::error::ErrorCode;
use crate::states::party::*;
use anchor_lang::prelude::*;

pub fn create_party(ctx: Context<CreateParty>, name: String) -> Result<()> {
    require!(name.chars().count() <= 32,ErrorCode::NameTooLong);
    require!(name.chars().count() > 0,ErrorCode::EmptyInput);


    let party: &mut Account<Party> = &mut ctx.accounts.party;
    let author: &mut Signer = &mut ctx.accounts.author;
    let clock: Clock = Clock::get().unwrap();
    let bump: u8 = *ctx.bumps.get("party").unwrap(); 
    

    party.bump = bump;
    party.author = *author.key;
    party.created = clock.unix_timestamp;
    party.voting_started = false;
    party.voting_ends = 0;
    party.name = name;
    party.votes = 0;

    Ok(())
}

pub fn start_voting(ctx: Context<StartVoting>, _name: String) -> Result<()> {
    let party: &mut Account<Party> = &mut ctx.accounts.party;

    require!(party.voting_started == false,ErrorCode::VotingAlreadyStarted);
    
    let clock: Clock = Clock::get().unwrap();
    party.voting_started = true;
    party.voting_ends = clock.unix_timestamp + (7 * 24 * 60 * 60);
    Ok(())
}

pub fn delete_party(_ctx: Context<DeleteParty>, _name: String) -> Result<()> {
    Ok(())
}
