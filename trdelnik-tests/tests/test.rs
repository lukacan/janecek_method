use colored::Colorize;
use fehler::throws;
use program_client::janecek_method_instruction::*;
use trdelnik_client::{
    anyhow::Result,
    solana_sdk::{native_token::lamports_to_sol, system_program},
    *,
};
#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();
    fixture.deploy().await?;

    fixture
        .client
        .airdrop(fixture.person1.pubkey(), 5_000_000_000)
        .await?;

    fixture
        .client
        .airdrop(fixture.person2.pubkey(), 5_000_000_000)
        .await?;

    fixture
        .client
        .airdrop(fixture.person3.pubkey(), 5_000_000_000)
        .await?;

    fixture
}

///---------------------------------------------------------------------------
/// Test create and delete Party
///---------------------------------------------------------------------------
#[trdelnik_test]
async fn test_create_delete_party(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    println!("{}", "Test create and delete Party".yellow());

    create_party(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    delete_party(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    println!("{}", "party1 - strana created and deleted".green());
}



///---------------------------------------------------------------------------
/// Test create and delete Party after voting started
///---------------------------------------------------------------------------
#[trdelnik_test]
async fn test_create_delete_party2(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    println!("{}", "Test create and delete Party after voting started".yellow());

    create_party(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;


    start_voting(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    let result = delete_party(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await;

    match result {
        Ok(_val) => {
            panic!("This should failed")
        }
        Err(_e) => {
            matches!(_e,ClientError::AnchorError(..));
            println!("{}", "Successfuly failed".green())
        }
    }
}

///---------------------------------------------------------------------------
/// Test create and delete Tweet
///---------------------------------------------------------------------------
#[trdelnik_test]
async fn test_create_delete_tweet(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    println!("{}", "Test create and delete Tweet".yellow());

    create_tweet(
        &fixture.client,
        fixture.timestamp,
        "Tweet1 Topic1".to_string(),
        "Tweet1 Content1".to_string(),
        fixture.person1.pubkey(),
        fixture.tweet1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    delete_tweet(
        &fixture.client,
        fixture.person1.pubkey(),
        fixture.tweet1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    println!("{}", "Tweet created and deleted".green());
}


///---------------------------------------------------------------------------
/// Test reinitialize Party
///---------------------------------------------------------------------------
#[trdelnik_test]
async fn test_reinitialize_party(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    println!("{}", "Test re-initialize Party".yellow());

    create_party(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person3.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person3.clone()],
    )
    .await?;

    let result = create_party(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person3.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person3.clone()],
    )
    .await;

    match result {
        Ok(_val) => {
            panic!("This should failed")
        }
        Err(_e) => {
            println!("{}", "Successfuly failed".green())
        }
    }
}



///---------------------------------------------------------------------------
/// Test reinitialize Voter
///---------------------------------------------------------------------------
#[trdelnik_test]
async fn test_reinitialize_voter(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    println!("{}", "Test re-initialize Voter".yellow());

    //fixture.print_state().await?;

    create_voter(
        &fixture.client,
        fixture.person2.pubkey(),
        fixture.voter1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;

    let result = create_voter(
        &fixture.client,
        fixture.person2.pubkey(),
        fixture.voter1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await;

    match result {
        Ok(_val) => {
            panic!("This should failed")
        }
        Err(_e) => {
            println!("{}", "Successfuly failed".green())
        }
    }
}


///---------------------------------------------------------------------------
/// Test reinitialize Tweet
///---------------------------------------------------------------------------
#[trdelnik_test]
async fn test_reinitialize_tweet(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    println!("{}", "Test re-initialize Tweet".yellow());


    create_tweet(
        &fixture.client,
        fixture.timestamp.clone(),
        "Tweet1 Topic1".to_string(),
        "Tweet1 Content1".to_string(),
        fixture.person1.pubkey(),
        fixture.tweet1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    let result = create_tweet(
        &fixture.client,
        fixture.timestamp.clone(),
        "Tweet1 Topic1 reinitialized".to_string(),
        "Tweet1 Content1 reinitialized".to_string(),
        fixture.person1.pubkey(),
        fixture.tweet1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await;

    match result {
        Ok(_val) => {
            panic!("This should failed")
        }
        Err(_e) => {
            println!("{}", "Successfuly failed".green())
        }
    }
}

///---------------------------------------------------------------------------
/// Test create Voter and Vote Negative first
///---------------------------------------------------------------------------
#[trdelnik_test]
async fn test_vote_neg_first(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    println!("{}", "Test Vote negative first".yellow());

    create_voter(
        &fixture.client,
        fixture.person2.pubkey(),
        fixture.voter1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;

    create_party(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    start_voting(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    let result = vote_negative(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person2.pubkey(),
        fixture.voter1,
        fixture.party1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await;

    match result {
        Ok(_val) => {
            panic!("This should failed")
        }
        Err(_e) => {
            matches!(_e,ClientError::AnchorError(..));
            println!("{}", "Successfuly failed".green())
        }
    }
}


///---------------------------------------------------------------------------
/// Test create Voter and Vote Negative Second
///---------------------------------------------------------------------------
#[trdelnik_test]
async fn test_vote_neg_second(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    println!("{}", "Test Vote negative second".yellow());

    create_voter(
        &fixture.client,
        fixture.person2.pubkey(),
        fixture.voter1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;

    create_party(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    start_voting(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    vote_positive(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person2.pubkey(),
        fixture.voter1,
        fixture.party1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;

    let result = vote_negative(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person2.pubkey(),
        fixture.voter1,
        fixture.party1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await;

    match result {
        Ok(_val) => {
            panic!("This should failed")
        }
        Err(_e) => {
            matches!(_e,ClientError::AnchorError(..));
            println!("{}", "Successfuly failed".green())
        }
    }
}


///---------------------------------------------------------------------------
/// Test create Voter and Vote Positive for Same Party Twice
///---------------------------------------------------------------------------
#[trdelnik_test]
async fn test_vote_pos_same_party(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    println!("{}", "Test Vote positive for same Party".yellow());

    create_voter(
        &fixture.client,
        fixture.person2.pubkey(),
        fixture.voter1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;

    create_party(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    start_voting(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    vote_positive(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person2.pubkey(),
        fixture.voter1,
        fixture.party1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;

    let result = vote_positive(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person2.pubkey(),
        fixture.voter1,
        fixture.party1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await;

    match result {
        Ok(_val) => {
            panic!("This should failed")
        }
        Err(_e) => {
            matches!(_e,ClientError::AnchorError(..));
            println!("{}", "Successfuly failed".green())
        }
    }
}

///---------------------------------------------------------------------------
/// Test Voting Regularly (2 different Pos and 1 Neg)
///---------------------------------------------------------------------------
#[trdelnik_test]
async fn test_vote_regular(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    println!(
        "{}",
        "Test Vote regular, two positive votes + one negative".yellow()
    );

    create_voter(
        &fixture.client,
        fixture.person2.pubkey(),
        fixture.voter1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;

    create_party(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    start_voting(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;

    create_party(
        &fixture.client,
        "party2 - strana".to_string(),
        fixture.person2.pubkey(),
        fixture.party2,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;

    start_voting(
        &fixture.client,
        "party2 - strana".to_string(),
        fixture.person2.pubkey(),
        fixture.party2,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;

    vote_positive(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person2.pubkey(),
        fixture.voter1,
        fixture.party1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;

    vote_positive(
        &fixture.client,
        "party2 - strana".to_string(),
        fixture.person2.pubkey(),
        fixture.voter1,
        fixture.party2,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;

    vote_negative(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person2.pubkey(),
        fixture.voter1,
        fixture.party1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;
    println!("{}", "Regularly spent 3 votes successfuly".green())
}

struct Fixture {
    client: Client,
    program: Keypair,

    person1: Keypair,
    person2: Keypair,
    person3: Keypair,

    party1: Pubkey,
    party2: Pubkey,
    voter1: Pubkey,

    tweet1: Pubkey,
    timestamp: String,
}
impl Fixture {
    fn new() -> Self {
        let program = program_keypair(1);

        let person1 = keypair(40);
        let person2 = keypair(41);
        let person3 = keypair(42);

        let timestamp: String = "10000000".to_string();

        let (party1, _) =
            Pubkey::find_program_address(&["party1 - strana".as_bytes()], &program.pubkey());

        let (party2, _) =
            Pubkey::find_program_address(&["party2 - strana".as_bytes()], &program.pubkey());

        let (voter1, _) = Pubkey::find_program_address(
            &["new_voter".as_bytes(), person2.pubkey().as_ref()],
            &program.pubkey(),
        );

        let (tweet1, _) = Pubkey::find_program_address(
            &[person1.pubkey().as_ref(), timestamp.as_bytes()],
            &program.pubkey(),
        );

        Fixture {
            client: Client::new(system_keypair(0)),
            program,
            person1,
            person2,
            person3,
            party1,
            party2,
            voter1,
            tweet1,
            timestamp,
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
        self.client
            .deploy_by_name(&self.program.clone(), "janecek_method")
            .await?;
    }
    #[throws]
    #[allow(dead_code)]
    async fn print_state(&mut self) {
        println!("\n-------------STATE---------------");
        println!(
            "person1 balance: {:?}\n",
            lamports_to_sol(self.client.get_balance(self.person1.pubkey()).await?),
        );
        println!("---------------------------------\n");
    }
}
