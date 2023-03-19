use anchor_lang::prelude::*;
use solana_program::{pubkey, pubkey::Pubkey};

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");
pub const ADMIN: Pubkey = pubkey!("G6ScTg7oSQPoKv6WtikLJeFgnE85TJBCPFAaiha7qbzJ");

#[program]
pub mod janecek_method {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let votingsystem: &mut Account<VotingSystem> = &mut ctx.accounts.VotingSystem;
        votingsystem.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn create_party(ctx: Context<CreateParty>, name: String) -> Result<()> {
        if name.chars().count() > 32 {
            return Err(ErrorCode::NameTooLong.into());
        }

        let party: &mut Account<Party> = &mut ctx.accounts.party;
        let author: &mut Signer = &mut ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();


        party.bump = *ctx.bumps.get("party").unwrap();
        party.author = *author.key;
        party.votingsystem = *ctx.accounts.votingsystem.key;
        party.created = clock.unix_timestamp;
        party.name = name;
        party.votes = 0;

        Ok(())
    }
    pub fn create_voter(ctx: Context<CreateVoter>) -> Result<()> {
        let voter: &mut Account<Voter> = &mut ctx.accounts.voter;
        voter.num_votes = 0;
        voter.can_vote = true;
        Ok(())
    }
    pub fn vote_positive(ctx: Context<Vote>) -> Result<()> {
        let voter: &mut Account<Voter> = &mut ctx.accounts.voter;
        let party: &mut Account<Party> = &mut ctx.accounts.party;

        if voter.num_votes == 0 {
            party.votes += 1;
            voter.num_votes += 1;
            voter.pos1 = ctx.accounts.party.key();
            Ok(())
        } else if voter.num_votes == 1 && voter.can_vote == true && voter.pos1 != party.key() {
            party.votes += 1;
            voter.num_votes += 1;
            voter.pos2 = party.key();
            Ok(())
        } else {
            return Err(ErrorCode::NotAllowedOperation.into());
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
            return Err(ErrorCode::NotAllowedOperation.into());
        }
    }
}

#[account]
pub struct VotingSystem {
    pub authority: Pubkey,
}
#[account]
pub struct Party {
    pub author: Pubkey,
    pub votingsystem: Pubkey,
    pub created: i64,
    pub name: String,
    pub votes: i64,
    pub bump: u8,
}
#[account]
pub struct Voter {
    pub num_votes: i8,
    pub can_vote: bool,
    pub pos1: Pubkey,
    pub pos2: Pubkey,
    pub neg: Pubkey,
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

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=authority,space=210)]
    pub VotingSystem: Account<'info, VotingSystem>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateParty<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(init, payer = author, space = Party::LEN, seeds = [name.as_bytes(),
    votingsystem.key().as_ref()],bump)]
    pub party: Account<'info, Party>,
    #[account(mut)]
    pub votingsystem: Account<'info, VotingSystem>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateVoter<'info> {
    #[account(mut,address=ADMIN @ ErrorCode::PermissionDenied)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = 200)]
    pub voter: Account<'info, Voter>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut, signer)]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub party: Account<'info, Party>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Party Name Too Long")]
    NameTooLong,
    #[msg("Only admin can add new Voters")]
    PermissionDenied,
    #[msg("Operation not allowed")]
    NotAllowedOperation,
}
