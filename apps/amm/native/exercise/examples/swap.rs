use solana_address::Address;
use solana_client::rpc_client::RpcClient;
use solana_program_pack::Pack;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Signer, read_keypair_file},
    system_instruction,
    transaction::Transaction,
};
use std::path::PathBuf;
use std::str::FromStr;

use amm::Cmd;

/*
KEYPAIR=$HOME/.config/solana/id.json
RPC=https://api.devnet.solana.com
PROGRAM_ID=9Xsm3WVTBY6ALbUhRTDzt5wVZiNN52BU5kXUR3m6ERZ
MINT_A=73Rgt8CZCJez89VtJdRd84kfUPVSprS2Sy5V7Skmq1bU
MINT_B=GK4c9bYHnKEDeKMXfve9xWFQ7byjjaWWdGNrzBW2Geep

cargo run --example swap $KEYPAIR $RPC $PROGRAM_ID $MINT_A $MINT_B
*/
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let keypair_path: PathBuf = [&args[1]].iter().collect();
    let payer =
        read_keypair_file(keypair_path).expect("Cannot read keypair file");

    // Connect to local cluster
    let rpc_url = String::from(&args[2]);
    let client =
        RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let program_id = Pubkey::from_str(&args[3]).expect("Invalid program ID");

    // Mints
    let mint_a = Pubkey::from_str(&args[4]).expect("Invalid mint a");
    let mint_b = Pubkey::from_str(&args[5]).expect("Invalid mint b");

    // Pool PDA
    let fee: u16 = 500;
    let (pool_pda, pool_bump) = Pubkey::find_program_address(
        &[
            amm::constants::POOL_AUTH,
            mint_a.as_ref(),
            mint_b.as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        &program_id,
    );

    let (mint_pool_pda, mint_pool_bump) = Pubkey::find_program_address(
        &[
            amm::constants::POOL_MINT,
            mint_a.as_ref(),
            mint_b.as_ref(),
            fee.to_le_bytes().as_ref(),
        ],
        &program_id,
    );

    let pool_a = get_ata(&mint_a, &pool_pda);
    let pool_b = get_ata(&mint_b, &pool_pda);

    let ata_a = get_ata(&mint_a, &payer.pubkey());
    let ata_b = get_ata(&mint_b, &payer.pubkey());
    let ata_liq = get_ata(&mint_pool_pda, &payer.pubkey());

    println!("ATA A: {:?}", ata_a);
    println!("ATA B: {:?}", ata_b);
    println!("ATA liq: {:?}", ata_liq);

    // Swap
    println!("--- Swap ---");

    let a_for_b = true;
    let amt_in = 1e6 as u64;
    let min_amt_out = (0.95 * 1e6) as u64;

    let ix = create_swap_ix(
        program_id,
        payer.pubkey(),
        fee,
        mint_a,
        mint_b,
        pool_pda,
        pool_bump,
        pool_a,
        pool_b,
        ata_a,
        ata_b,
        a_for_b,
        amt_in,
        min_amt_out,
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    let blockhash = client.get_latest_blockhash().unwrap();
    tx.sign(&[&payer], blockhash);

    let res = client.send_and_confirm_transaction(&tx);
    res.unwrap();

    println!("User A: {:?}", get_token_balance(&client, &ata_a));
    println!("User B: {:?}", get_token_balance(&client, &ata_b));
    println!("Pool A: {:?}", get_token_balance(&client, &pool_a));
    println!("Pool B: {:?}", get_token_balance(&client, &pool_b));
}

fn get_ata(mint: &Pubkey, owner: &Pubkey) -> Pubkey {
    let ata_addr = spl_associated_token_account_interface::address::get_associated_token_address(
        &Address::from(owner.to_bytes()),
        &Address::from(mint.to_bytes()),
    );
    Pubkey::from(ata_addr.to_bytes())
}

fn get_token_balance(client: &RpcClient, token_account: &Pubkey) -> u64 {
    let data = client.get_account_data(token_account).unwrap();
    let account = spl_token_interface::state::Account::unpack(&data).unwrap();
    account.amount
}

fn create_swap_ix(
    program_id: Pubkey,
    payer: Pubkey,
    fee: u16,
    mint_a: Pubkey,
    mint_b: Pubkey,
    pool: Pubkey,
    pool_bump: u8,
    pool_a: Pubkey,
    pool_b: Pubkey,
    payer_a: Pubkey,
    payer_b: Pubkey,
    a_for_b: bool,
    amount_in: u64,
    min_amount_out: u64,
) -> Instruction {
    let cmd = Cmd::Swap {
        fee,
        pool_bump,
        a_for_b,
        amount_in,
        min_amount_out,
    };

    Instruction::new_with_borsh(
        program_id,
        &cmd,
        vec![
            AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: mint_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: payer_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Pubkey::from(spl_token_interface::ID.to_bytes()),
                is_signer: false,
                is_writable: true,
            },
        ],
    )
}
