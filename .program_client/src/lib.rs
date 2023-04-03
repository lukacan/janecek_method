// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod janecek_method_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 214u8, 204u8, 101u8, 166u8, 163u8, 239u8, 244u8, 13u8, 110u8, 64u8, 106u8, 230u8,
        81u8, 141u8, 186u8, 208u8, 155u8, 78u8, 83u8, 194u8, 215u8, 103u8, 17u8, 94u8, 15u8, 137u8,
        68u8, 170u8, 153u8, 74u8, 59u8,
    ]);
    pub async fn create_party(
        client: &Client,
        i_name: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_party: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                janecek_method::instruction::CreateParty { name: i_name },
                janecek_method::accounts::CreateParty {
                    author: a_author,
                    party: a_party,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn create_party_ix(
        i_name: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_party: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: janecek_method::instruction::CreateParty { name: i_name }.data(),
            accounts: janecek_method::accounts::CreateParty {
                author: a_author,
                party: a_party,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn delete_party(
        client: &Client,
        i_name: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_party: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                janecek_method::instruction::DeleteParty { name: i_name },
                janecek_method::accounts::DeleteParty {
                    author: a_author,
                    party: a_party,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn delete_party_ix(
        i_name: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_party: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: janecek_method::instruction::DeleteParty { name: i_name }.data(),
            accounts: janecek_method::accounts::DeleteParty {
                author: a_author,
                party: a_party,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn create_voter(
        client: &Client,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_voter: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                janecek_method::instruction::CreateVoter {},
                janecek_method::accounts::CreateVoter {
                    author: a_author,
                    voter: a_voter,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn create_voter_ix(
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_voter: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: janecek_method::instruction::CreateVoter {}.data(),
            accounts: janecek_method::accounts::CreateVoter {
                author: a_author,
                voter: a_voter,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn vote_positive(
        client: &Client,
        i_name: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_voter: anchor_lang::solana_program::pubkey::Pubkey,
        a_party: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                janecek_method::instruction::VotePositive { name: i_name },
                janecek_method::accounts::Vote {
                    author: a_author,
                    voter: a_voter,
                    party: a_party,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn vote_positive_ix(
        i_name: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_voter: anchor_lang::solana_program::pubkey::Pubkey,
        a_party: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: janecek_method::instruction::VotePositive { name: i_name }.data(),
            accounts: janecek_method::accounts::Vote {
                author: a_author,
                voter: a_voter,
                party: a_party,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn vote_negative(
        client: &Client,
        i_name: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_voter: anchor_lang::solana_program::pubkey::Pubkey,
        a_party: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                janecek_method::instruction::VoteNegative { name: i_name },
                janecek_method::accounts::Vote {
                    author: a_author,
                    voter: a_voter,
                    party: a_party,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn vote_negative_ix(
        i_name: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_voter: anchor_lang::solana_program::pubkey::Pubkey,
        a_party: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: janecek_method::instruction::VoteNegative { name: i_name }.data(),
            accounts: janecek_method::accounts::Vote {
                author: a_author,
                voter: a_voter,
                party: a_party,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn create_tweet(
        client: &Client,
        i__time_stamp: String,
        i_topic: String,
        i_content: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_tweet: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                janecek_method::instruction::CreateTweet {
                    _time_stamp: i__time_stamp,
                    topic: i_topic,
                    content: i_content,
                },
                janecek_method::accounts::CreateTweet {
                    author: a_author,
                    tweet: a_tweet,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn create_tweet_ix(
        i__time_stamp: String,
        i_topic: String,
        i_content: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_tweet: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: janecek_method::instruction::CreateTweet {
                _time_stamp: i__time_stamp,
                topic: i_topic,
                content: i_content,
            }
            .data(),
            accounts: janecek_method::accounts::CreateTweet {
                author: a_author,
                tweet: a_tweet,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn update_tweet_topic(
        client: &Client,
        i_topic: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_tweet: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                janecek_method::instruction::UpdateTweetTopic { topic: i_topic },
                janecek_method::accounts::UpdateTweet {
                    author: a_author,
                    tweet: a_tweet,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn update_tweet_topic_ix(
        i_topic: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_tweet: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: janecek_method::instruction::UpdateTweetTopic { topic: i_topic }.data(),
            accounts: janecek_method::accounts::UpdateTweet {
                author: a_author,
                tweet: a_tweet,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn update_tweet_content(
        client: &Client,
        i_content: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_tweet: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                janecek_method::instruction::UpdateTweetContent { content: i_content },
                janecek_method::accounts::UpdateTweet {
                    author: a_author,
                    tweet: a_tweet,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn update_tweet_content_ix(
        i_content: String,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_tweet: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: janecek_method::instruction::UpdateTweetContent { content: i_content }.data(),
            accounts: janecek_method::accounts::UpdateTweet {
                author: a_author,
                tweet: a_tweet,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn delete_tweet(
        client: &Client,
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_tweet: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                janecek_method::instruction::DeleteTweet {},
                janecek_method::accounts::DeleteTweet {
                    author: a_author,
                    tweet: a_tweet,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn delete_tweet_ix(
        a_author: anchor_lang::solana_program::pubkey::Pubkey,
        a_tweet: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: janecek_method::instruction::DeleteTweet {}.data(),
            accounts: janecek_method::accounts::DeleteTweet {
                author: a_author,
                tweet: a_tweet,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
}
