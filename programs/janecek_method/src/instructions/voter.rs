use crate::error::ErrorCode;
use crate::states::party::*;
use crate::states::voter::*;
use anchor_lang::prelude::*;

pub fn create_voter(ctx: Context<CreateVoter>) -> Result<()> {
    let voter: &mut Account<Voter> = &mut ctx.accounts.voter;
    let author: &mut Signer = &mut ctx.accounts.author;
    let bump: u8 = *ctx.bumps.get("voter").unwrap();

    voter.author = author.key();
    voter.bump = bump;
    voter.num_votes = 3;
    Ok(())
}
pub fn vote_positive(ctx: Context<Vote>, _name:String) -> Result<()> {
    let voter: &mut Account<Voter> = &mut ctx.accounts.voter;
    let party: &mut Account<Party> = &mut ctx.accounts.party;

    require!(voter.num_votes > 1, ErrorCode::NoMoreVotes); 

    if voter.num_votes == 3 {
        voter.num_votes -= 1;
        voter.pos1 = party.key();
        party.votes += 1;
        Ok(())
    } else if voter.num_votes == 2 {
        require!(voter.pos1 != party.key(), ErrorCode::NoTwoToOneParty);
        voter.num_votes -= 1;
        voter.pos2 = party.key();
        party.votes += 1;
        Ok(())
    } else {
        Err(ProgramError::InvalidInstructionData.into())
    }
}
pub fn vote_negative(ctx: Context<Vote>, _name:String) -> Result<()> {
    let voter: &mut Account<Voter> = &mut ctx.accounts.voter;
    let party: &mut Account<Party> = &mut ctx.accounts.party;

    require!(voter.num_votes>0, ErrorCode::NoMoreVotes); 
    require!(voter.num_votes <= 1, ErrorCode::VotePosFirst);

    if voter.num_votes == 1 {
        voter.num_votes -= 1;
        voter.neg = party.key();
        party.votes -= 1;
        Ok(())
    } else {
        Err(ProgramError::InvalidInstructionData.into())
    }
}
