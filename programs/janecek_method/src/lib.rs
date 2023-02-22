use anchor_lang::prelude::*;

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");

#[program]
pub mod janecek_method {
    use super::*;
    pub fn create_party(ctx: Context<CreateParty>, name: String)-> Result<()>{
        let party: &mut Account<Party> = &mut ctx.accounts.party;
        let author: &mut Signer = &mut ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();
        
        party.author = *author.key;
        party.created = clock.unix_timestamp;
        party.name = name;
        party.votes = 0;
        Ok(())
    }
}
#[account]
pub struct Party{
    pub author: Pubkey,
    pub created: i64,
    pub name: String,
    pub votes: i64, 
    pub bump: u8,
}
#[account]
pub struct Voter{
    pub who: Pubkey,
    pub num_votes: i8,
}
#[derive(Accounts)]
pub struct Vote<'info> {
    pub tweet: Account<'info, Voter>,
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CreateParty<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(init, payer = author, space = 200, seeds = ["name2".as_bytes()],bump)]
    pub party: Account<'info, Party>,
    pub system_program: Program<'info, System>,
}
