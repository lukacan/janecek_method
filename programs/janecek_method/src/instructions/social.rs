use crate::error::ErrorCode;
use crate::states::social::*;
use anchor_lang::prelude::*;

pub fn create_tweet(ctx: Context<CreateTweet>, topic: String, content: String) -> Result<()> {
    require!(topic.chars().count() <= TOPIC_LENGTH,ErrorCode::TopicTooLong);
    require!(topic.chars().count()  > 0,ErrorCode::EmptyInput);

    require!(content.chars().count() <= CONTENT_LENGTH,ErrorCode::ContentTooLong);
    require!(content.chars().count() > 0,ErrorCode::EmptyInput);


    let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
    let author: &mut Signer = &mut ctx.accounts.author;
    let clock: Clock = Clock::get().unwrap();

    tweet.author = author.key();
    tweet.created = clock.unix_timestamp;
    tweet.topic = topic;
    tweet.content = content;

    Ok(())
}
pub fn update_tweet_topic(ctx: Context<UpdateTweet>, topic: String) -> Result<()> {
    require!(topic.chars().count() <= TOPIC_LENGTH,ErrorCode::TopicTooLong);
    require!(topic.chars().count() > 0,ErrorCode::EmptyInput);
    
    let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
    tweet.topic = topic;

    Ok(())
}

pub fn update_tweet_content(ctx: Context<UpdateTweet>, content: String) -> Result<()> {
    require!(content.chars().count() <= CONTENT_LENGTH,ErrorCode::ContentTooLong);
    require!(content.chars().count() > 0,ErrorCode::EmptyInput);

    let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
    tweet.content = content;

    Ok(())
}
pub fn delete_tweet(_ctx: Context<UpdateTweet>) -> Result<()> {
    Ok(())
}
