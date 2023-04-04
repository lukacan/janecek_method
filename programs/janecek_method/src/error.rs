use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Max name length exceeded")]
    NameTooLong,
    #[msg("No 2 positive votes to one party")]
    NoTwoToOneParty,
    #[msg("Vote 2 times positive first")]
    VotePosFirst,
    #[msg("No more votes")]
    NoMoreVotes,
    #[msg("Topic too Long")]
    TopicTooLong,
    #[msg("Content too Long")]
    ContentTooLong,
    #[msg("Empty input")]
    EmptyInput,
    #[msg("Voting already started")]
    VotingAlreadyStarted,
    #[msg("Voting not started yet")]
    VotingNotStartedYet,
    #[msg("Voting already ended")]
    VotingAlreadyEnded,
    #[msg("Can`t delete party after voting started")]
    CantDeleteDuringVoting,
}