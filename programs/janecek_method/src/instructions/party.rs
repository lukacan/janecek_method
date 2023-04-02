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
    party.name = name;
    party.votes = 0;

    Ok(())
}

pub fn delete_party(_ctx: Context<DeleteParty>) -> Result<()> {
    Ok(())
}
