use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let program_id = Pubkey::from_str(&args[1]).expect("Invalid program ID");

    // Connect to local cluster
    let rpc_url = String::from("http://localhost:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Generate a new keypair for paying fees
    let payer = Keypair::new();

    // Request airdrop of 1 SOL for transaction fees
    println!("Requesting airdrop...");
    let airdrop_signature = client
        .request_airdrop(&payer.pubkey(), 1_000_000_000)
        .expect("Failed to request airdrop");

    // Wait for airdrop confirmation
    loop {
        if client
            .confirm_transaction(&airdrop_signature)
            .unwrap_or(false)
        {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("Airdrop confirmed");

    // Create the instruction
    let ix = Instruction::new_with_borsh(
        program_id,
        &(),    // Empty instruction data
        vec![], // No accounts needed
    );

    // Sign and send transaction
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], client.get_latest_blockhash().unwrap());

    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => {
            println!("Transaction Signature: {}", sig);

            // Fetch transaction details with logs
            let tx_info = client
                .get_transaction_with_config(
                    &sig,
                    RpcTransactionConfig {
                        encoding: Some(
                            solana_transaction_status_client_types::UiTransactionEncoding::Json,
                        ),
                        commitment: Some(CommitmentConfig::confirmed()),
                        max_supported_transaction_version: Some(0),
                    },
                )
                .expect("Failed to get transaction info");

            if let Some(meta) = tx_info.transaction.meta {
                if let solana_transaction_status_client_types::option_serializer::OptionSerializer::Some(logs) = meta.log_messages {
                    println!("--- Transaction Logs ---");
                    for log in logs.iter() {
                        println!("{}", log);
                    }
                } else {
                    println!("No logs available for this transaction.");
                }
            } else {
                println!("Transaction metadata not found.");
            }
        }
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
}
