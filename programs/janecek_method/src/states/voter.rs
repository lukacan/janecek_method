use anchor_lang::prelude::*;
use crate::states::party::*;
use crate::error::ErrorCode;

#[account]
pub struct Voter {
    pub author: Pubkey,
    pub num_votes: i8,
    pub pos1: Pubkey,
    pub pos2: Pubkey,
    pub neg: Pubkey,
    pub bump: u8,
}
impl Voter {
    const LEN: usize = 8 //discriminator
    + 32 //author
    + 1 // num votes
    + 32 // pos1
    + 32 // pos2
    + 32 // neg
    + 1; // bump
}



#[derive(Accounts)]
pub struct CreateVoter<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init,
        payer = author, 
        space = Voter::LEN, 
        seeds=[b"new_voter",author.key().as_ref()],
        bump)]
    pub voter: Account<'info, Voter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]

pub struct Vote<'info> {
    pub author: Signer<'info>,
    #[account(
        mut,
        has_one = author,
        constraint = voter.num_votes > 0 @ ErrorCode::NoMoreVotes,
        seeds=[b"new_voter",author.key().as_ref()],
        bump = voter.bump)]
    pub voter: Account<'info, Voter>,
    #[account(
        mut,
        seeds=[name.as_bytes()],
        constraint = party.voting_started == true @ ErrorCode::VotingNotStartedYet,
        constraint = party.voting_ends >= Clock::get().unwrap().unix_timestamp @ ErrorCode::VotingAlreadyEnded,
        bump = party.bump)]
    pub party: Account<'info, Party>,
    pub system_program: Program<'info, System>,
}



