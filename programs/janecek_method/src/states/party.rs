use anchor_lang::prelude::*;


#[account]
pub struct Party {
    pub author: Pubkey,
    pub created: i64,
    pub name: String,
    pub votes: i64,
    pub bump: u8,
}


impl Party {
    const LEN: usize = 8 // discriminator
        + 32 // author
        + 8 // created
        + 4 // vector prefix
        + 32 * 4 // number of bytes * size of char
        + 8 // votes
        + 1; // bump
}


#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateParty<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init, 
        payer = author, 
        space = Party::LEN, 
        seeds = [name.as_bytes()],
        bump)]
    pub party: Account<'info, Party>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(name: String)]
pub struct DeleteParty<'info>{
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        mut,
        has_one=author,
        close = author,
        seeds=[name.as_bytes()],
        bump=party.bump)]
    pub party: Account<'info,Party>,
    pub system_program: Program<'info, System>,    
}