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
}

#[trdelnik_test]
async fn test_create_party(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    println!("Program ID {:?}", fixture.program.pubkey());

    //fixture.print_state().await?;

    create_party(
        &fixture.client,
        "party1 - strana".to_string(),
        fixture.person1.pubkey(),
        fixture.party1,
        system_program::id(),
        [fixture.person1.clone()],
    )
    .await?;
    println!("party1 - strana created");
}

#[trdelnik_test]
async fn test_create_voter(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    println!("Program ID {:?}", fixture.program.pubkey());

    create_voter(
        &fixture.client,
        fixture.person2.pubkey(),
        fixture.voter1,
        system_program::id(),
        [fixture.person2.clone()],
    )
    .await?;
    println!("New voter created");
}
struct Fixture {
    client: Client,
    program: Keypair,

    person1: Keypair,
    person2: Keypair,

    party1: Pubkey,
    voter1: Pubkey,
}
impl Fixture {
    fn new() -> Self {
        let program = program_keypair(1);

        let person1 = keypair(40);
        let person2 = keypair(41);

        let (party1, _) =
            Pubkey::find_program_address(&["party1 - strana".as_bytes()], &program.pubkey());
        let (voter1, _) = Pubkey::find_program_address(
            &["new_voter".as_bytes(), person2.pubkey().as_ref()],
            &program.pubkey(),
        );

        Fixture {
            client: Client::new(system_keypair(0)),
            program,
            person1,
            person2,
            party1,
            voter1,
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
    async fn print_state(&mut self) {
        println!("\n-------------STATE---------------");
        println!(
            "person1 balance: {:?}\n",
            lamports_to_sol(self.client.get_balance(self.person1.pubkey()).await?),
        );
        println!("---------------------------------\n");
    }
}
