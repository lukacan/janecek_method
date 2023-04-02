use anchor_lang::prelude::*;


pub const TOPIC_LENGTH:usize = 32;
pub const CONTENT_LENGTH:usize = 500;

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub created: i64,
    pub last_modified: i64,
    pub topic: String,
    pub content: String,
    pub bump: u8,
}

impl Tweet {
    const LEN: usize = 8 //discriminator
    + 32 // author
    + 8 // created
    + 8 // last modified
    + 4 // topic prefix
    + TOPIC_LENGTH * 4 // topic
    + 4 // content prefix
    + CONTENT_LENGTH*4 // content
    + 1; // bump
}

#[derive(Accounts)]
#[instruction(time_stamp: String)]
pub struct CreateTweet<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init,
        payer=author,
        space= Tweet::LEN,
        seeds=[author.key().as_ref(),time_stamp.as_bytes()],
        bump)]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTweet<'info> {
    pub author: Signer<'info>,
    #[account(
        mut,
        has_one = author)]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteTweet<'info>{
    pub author: Signer<'info>,
    #[account(mut,has_one=author,close = author)]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,

}
