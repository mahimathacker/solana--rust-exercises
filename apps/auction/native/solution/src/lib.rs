use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub mod instructions;
pub mod state;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Cmd {
    Init {
        start_price: u64,
        end_price: u64,
        start_time: u64,
        end_time: u64,
        sell_amt: u64,
    },
    Buy {
        max_price: u64,
    },
    Cancel,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = Cmd::try_from_slice(instruction_data)?;

    match ix {
        Cmd::Init {
            start_price,
            end_price,
            start_time,
            end_time,
            sell_amt,
        } => {
            //
        }
        Cmd::Buy { max_price } => {}
        Cmd::Cancel => {}
    }

    Ok(())
}
