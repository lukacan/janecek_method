use anchor_lang::prelude::*;
use solana_program::{pubkey::Pubkey};

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");

#[program]
pub mod janecek_method {
    use super::*;
    pub fn create_party(ctx: Context<CreateParty>, name: String) -> Result<()> {
        if name.chars().count() > 32 {
            return Err(ErrorCode::NameTooLong.into());
        }

        let party: &mut Account<Party> = &mut ctx.accounts.party;
        let author: &mut Signer = &mut ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        party.bump = *ctx.bumps.get("party").unwrap();
        party.author = *author.key;
        party.created = clock.unix_timestamp;
        party.name = name;
        party.votes = 0;

        Ok(())
    }
    pub fn create_voter(ctx: Context<CreateVoter>) -> Result<()> {
        let voter: &mut Account<Voter> = &mut ctx.accounts.voter;
        voter.author = ctx.accounts.author.key();
        voter.bump = *ctx.bumps.get("voter").unwrap();
        voter.num_votes = 0;
        voter.can_vote = true;
        Ok(())
    }
    pub fn vote_positive(ctx: Context<Vote>) -> Result<()> {
        let voter: &mut Account<Voter> = &mut ctx.accounts.voter;
        let party: &mut Account<Party> = &mut ctx.accounts.party;
        
        
        if voter.can_vote == false{
            // not able to vote
            return Err(ErrorCode::NotAbleToVote.into());
        }
        else if voter.num_votes == 0 {
            party.votes += 1;
            voter.num_votes += 1;
            voter.pos1 = ctx.accounts.party.key();
            return Ok(())
        }
        else if voter.num_votes == 1 && voter.pos1 == party.key(){
            // cant vote 2 times for one party
            return Err(ErrorCode::NoTwoToOneParty.into());
        }
        else if voter.num_votes == 1 && voter.pos1 != party.key(){
            party.votes += 1;
            voter.num_votes += 1;
            voter.pos2 = ctx.accounts.party.key();
            return Ok(())
        }else{
            // no more votes
            return Err(ErrorCode::NotAbleToVote.into());
        }
    }
    pub fn vote_negative(ctx: Context<Vote>) -> Result<()> {
        let voter: &mut Account<Voter> = &mut ctx.accounts.voter;
        let party: &mut Account<Party> = &mut ctx.accounts.party;
        
        
        if voter.num_votes == 2 {
            voter.num_votes -= 1;
            party.votes -= 1;
            voter.can_vote = false;
            voter.neg = ctx.accounts.party.key();
            Ok(())
        } else {
            return Err(ErrorCode::NotAbleToVote.into());
        }
    }
}
#[account]
pub struct Party {
    pub author: Pubkey,
    pub created: i64,
    pub name: String,
    pub votes: i64,
    pub bump: u8,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_NAME_LENGTH: usize = 32 * 4;
const NUM_VOTES_LENGTH: usize = 8;
const BUMP_LENGTH: usize = 1;

impl Party {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + TIMESTAMP_LENGTH
        + STRING_LENGTH_PREFIX
        + MAX_NAME_LENGTH
        + NUM_VOTES_LENGTH
        + BUMP_LENGTH;
}

#[account]
pub struct Voter {
    pub author: Pubkey,
    pub num_votes: i8,
    pub can_vote: bool,
    pub pos1: Pubkey,
    pub pos2: Pubkey,
    pub neg: Pubkey,
    pub bump: u8
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(mut,has_one = author)]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub party: Account<'info, Party>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CreateVoter<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(init, payer = author, space = 200,seeds=[b"new_voter",author.key().as_ref()],bump)]
    pub voter: Account<'info, Voter>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateParty<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(init, payer = author, space = Party::LEN, seeds = [name.as_bytes()],bump)]
    pub party: Account<'info, Party>,
    pub system_program: Program<'info, System>,
}


#[error_code]
pub enum ErrorCode {
    #[msg("Max name length exceeded")]
    NameTooLong,
    #[msg("No 2 positive votes to one party")]
    NoTwoToOneParty,
    #[msg("Vote positive first")]
    VotePosFirst,
    #[msg("Not able to vote")]
    NotAbleToVote,
}
