use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};

 use solana_sdk:: {
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};

use solana_transaction_status_client_types::{
    UiTransactionEncoding,
    option_serializer::OptionSerializer,
};

use std::path::PathBuf; // File path handling
use std::str::FromStr; // String parsing, covert strings to other types

fn main() {

    // step 2: Collect command line arguments to the program 

    let args: Vec<String> = std::env::args().collect(); 

// step 3: Load the wallet keypair 
let keypair_path : PathBuf =  [&args[1]].iter().collect(); //First user argument (args[0] is the program name)
let payer: Keypair = read_keypair_file(keypair_path).expect("Failed to read keypair file");

// step 4: connect to the solana

let rpc_url = String::from(&args[2]); //Second user argument
let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed()); /* Solana has different levels of "how sure are you this transaction happened":

processed - Node saw it (might be rolled back)
confirmed - Supermajority voted on it (pretty safe) ← we use this
finalized - Absolutely certain (slowest) */

//step 5: parse the program id from the command line argument
let program_id = Pubkey::from_str(&args[3]).expect("Invalid program id");

//step 6: print wallet info and balances

println!("Payer: {}", payer.pubkey());
let lamports = client.get_balance(&payer.pubkey()).unwrap();
let sol = lamports as f64 / solana_sdk::native_token::LAMPORTS_PER_SOL as f64;
println!("Balance: {} SDL ({} lamports)", sol, lamports); //Like wei in Ethereum. 1 SOL = 1,000,000,000 lamports (1 billion). f64: Type casting. lamports is u64 (integer), we need f64 (float) for division. unswap(): Like .expect() but without a custom message. If it fails, you get a generic panic. Use .expect() for better debugging.

//Step 7: Request airdrop if needed

if sol < 1.0 {
    println!("Requesting airdrop....");
    let airdrop_signature = client
        .request_airdrop(&payer.pubkey(), 1_000_000_000) 
        .expect("Airdrop request failed");

    while !client 
        .confirm_transaction(&airdrop_signature)
        .unwrap()
    {
        println!("Waiting for airdrop confirmation...");
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    println!("Airdrop received!");
}

//Step 9: Create the instruction

let ix = Instruction::new_with_borsh (
    program_id,
    &(),
    vec![], //Borsh is a serialization format (like JSON but binary). It encodes our data for the blockchain. There's also new_with_bincode and new_with_bytes.
);

// Step 10: Build and sign the transaction

let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
tx.sign(&[&payer], client.get_latest_blockhash().unwrap());

/* 

payer                    // Keypair (the actual wallet)
&payer                   // &Keypair (reference to wallet)
[&payer]                 // [&Keypair; 1] (array with one reference)
&[&payer]                // &[&Keypair] (slice - reference to array) 

 two references inside the slice array : tx.sign(&[&signer1, &signer2], blockhash);

*/

//Step 11: Send the transaction
 
let sig = client.send_and_confirm_transaction(&tx).expect("Transaction failed"); //end_transactionFire and forget, returns immediatelysend_and_confirm_transactionWaits until confirmed
println!("Transaction successful: {}", sig);

//Step 12: Fetch transaction logs

let tx_info = client
        .get_transaction_with_config(
        &sig,
        RpcTransactionConfig {
            encoding : Some(UiTransactionEncoding::Json),
            commitment: Some (CommitmentConfig::confirmed()),
            max_supported_transaction_version: Some(0),
        }
    )
    .expect("Failed to fetch transaction info");

//Step 13: Parse and print logs
 
if let Some(meta) = tx_info.transaction.meta {
     if let OptionSerializer::Some(logs) = meta.log_messages {
        println!("Transaction logs:");
        for (i, log) in logs.iter().enumerate() {
            println!("{} : {}", i, log);
        }
     } else {
        println!("No logs");
     }

} else {
    println!("No transaction metadata found");
}
}
