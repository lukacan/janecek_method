use anchor_lang::prelude::*;

pub const TOPIC_LENGTH:usize = 32;
pub const CONTENT_LENGTH:usize = 500;

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub created: i64,
    pub topic: String,
    pub content: String,
}

impl Tweet {
    const LEN: usize = 8 //discriminator
    + 32 // aithor
    + 8 // created
    + 4 // topic prefix
    + TOPIC_LENGTH * 4 // topic
    + 4 // content prefix
    + CONTENT_LENGTH*4; // content 
}

#[derive(Accounts)]
pub struct CreateTweet<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init,
        payer=author,
        space= Tweet::LEN)]
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
