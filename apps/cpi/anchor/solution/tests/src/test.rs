use std::str::FromStr;

use anchor_client::solana_sdk::signature::Signer;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        system_program,
    },
    Client, Cluster,
};

#[test]
fn test() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(
        Cluster::Localnet,
        &payer,
        CommitmentConfig::confirmed(),
    );

    let counter_program = client.program(counter::ID).unwrap();
    let factory_program = client.program(factory::ID).unwrap();

    // Init
    let counter_account = Keypair::new();

    let res = factory_program
        .request()
        .accounts(factory::accounts::Init {
            payer: payer.pubkey(),
            counter: counter_account.pubkey(),
            counter_program: counter::ID,
            system_program: system_program::ID,
        })
        .signer(&payer)
        .signer(&counter_account)
        .args(factory::instruction::Init {})
        .send();

    let counter_state: counter::Counter =
        counter_program.account(counter_account.pubkey()).unwrap();

    assert_eq!(counter_state.count, 0);

    // Increment
    let res = factory_program
        .request()
        .accounts(factory::accounts::Inc {
            counter: counter_account.pubkey(),
            counter_program: counter::ID,
        })
        .signer(&payer)
        .args(factory::instruction::Inc {})
        .send();

    let counter_state: counter::Counter =
        counter_program.account(counter_account.pubkey()).unwrap();

    assert_eq!(counter_state.count, 1);
}
