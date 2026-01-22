// use solana_program::{
//     account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
// };

// // Defines where execution of the program begins
// entrypoint!(process_instruction);

// pub fn process_instruction(
//     _program_id: &Pubkey,
//     _accounts: &[AccountInfo],
//     _instruction_data: &[u8],
// ) -> ProgramResult {
//     msg!("Hello Solana!");
//     Ok(())
// }

// #[cfg(test)]
// mod test {
//     use litesvm::LiteSVM;
//     use solana_sdk::{
//         instruction::Instruction,
//         message::Message,
//         signature::{Keypair, Signer},
//         transaction::Transaction,
//     };

//     #[test]
//     fn test_hello() {
//         let mut svm = LiteSVM::new();

//         let payer = Keypair::new();

//         svm.airdrop(&payer.pubkey(), 1e9 as u64).unwrap();

//         let program_keypair = Keypair::new();
//         let program_id = program_keypair.pubkey();
//         svm.add_program_from_file(program_id, "target/deploy/hello.so")
//             .unwrap();

//         let instruction = Instruction {
//             program_id,
//             accounts: vec![],
//             data: vec![],
//         };

//         let message = Message::new(&[instruction], Some(&payer.pubkey()));
//         let tx = Transaction::new(&[&payer], message, svm.latest_blockhash());

//         let res = svm.send_transaction(tx);
//         assert!(res.is_ok(), "Transaction should succeed");
//         let logs = res.unwrap().logs;
//         println!("Logs: {logs:#?}");
//     }
// }

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    log::sol_log_compute_units,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    // Get values from instruction data (runtime, not compile-time)
    let x: u64 = 1000000;
    let y: u64 = if instruction_data.is_empty() { 3 } else { instruction_data[0] as u64 };
    
    msg!("--- Unsigned Division (runtime divisor) ---");
    sol_log_compute_units();
    
    let mut unsigned_sum: u64 = 0;
    for _ in 0..100 {
        unsigned_sum += x / y;
        // Prevent compiler optimization
        core::hint::black_box(&unsigned_sum);
    }
    
    sol_log_compute_units();
    msg!("Unsigned sum: {}", unsigned_sum);

    msg!("--- Signed Division (runtime divisor) ---");
    sol_log_compute_units();
    
    let a: i64 = -1000000;
    let b: i64 = y as i64;
    let mut signed_sum: i64 = 0;
    for _ in 0..100 {
        signed_sum += a / b;
        core::hint::black_box(&signed_sum);
    }
    
    sol_log_compute_units();
    msg!("Signed sum: {}", signed_sum);

    Ok(())
}


let a: i64 = -10;
let b: i64 = 3;
let result = a / b;