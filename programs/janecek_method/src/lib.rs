use anchor_lang::prelude::*;
use states::*;

pub mod states;
pub mod instructions;
pub mod error;

//declare_id!("Po1RaS8BEDbNcn5oXsFryAeQ6Wn8fvmE111DJaKCgPC"); // trdelnik
declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph"); // deploy


#[program]
pub mod janecek_method {
    use super::*;
    pub fn create_party(ctx: Context<CreateParty>, name: String) -> Result<()> {
        instructions::create_party(ctx, name)
    }
    pub fn start_voting(ctx: Context<StartVoting>, name: String)-> Result<()>{
        instructions::start_voting(ctx,name)
    }
    pub fn delete_party(ctx:Context<DeleteParty>, name: String)-> Result<()>{
        instructions::delete_party(ctx,name)
    }
    pub fn create_voter(ctx: Context<CreateVoter>) -> Result<()> {
        instructions::create_voter(ctx)
    }
    pub fn vote_positive(ctx: Context<Vote>, name:String) -> Result<()> {
        instructions::vote_positive(ctx,name)
    }
    pub fn vote_negative(ctx: Context<Vote>, name:String) -> Result<()> {
        instructions::vote_negative(ctx,name)
    }
    pub fn create_tweet(ctx:Context<CreateTweet>,_time_stamp:String, topic: String, content: String)-> Result<()>{
        instructions::create_tweet(ctx,_time_stamp, topic, content)
    }
    pub fn update_tweet_topic(ctx:Context<UpdateTweet>, topic: String)->Result<()>{
        instructions::update_tweet_topic(ctx, topic)
    }
    pub fn update_tweet_content(ctx:Context<UpdateTweet>, content: String)-> Result<()>{
        instructions::update_tweet_content(ctx, content)
    }
    pub fn delete_tweet(ctx:Context<DeleteTweet>)-> Result<()>{
        instructions::delete_tweet(ctx)
    }
}



