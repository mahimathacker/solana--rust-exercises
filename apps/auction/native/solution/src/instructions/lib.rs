use solana_program::pubkey::{Pubkey, PubkeyError};

pub fn get_pda(
    program_id: &Pubkey,
    seller: &Pubkey,
    mint_sell: &Pubkey,
    mint_buy: &Pubkey,
    bump: u8,
) -> Result<Pubkey, PubkeyError> {
    Pubkey::create_program_address(
        &[
            b"auction",
            seller.as_ref(),
            mint_sell.as_ref(),
            mint_buy.as_ref(),
            &[bump],
        ],
        program_id,
    )
}

use solana_address::Address;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
};
use spl_associated_token_account_interface as spl_ata;

fn create_ata<'a>(
    payer: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
    ata: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    sys_program: &AccountInfo<'a>,
    ata_program: &AccountInfo<'a>,
    rent_sysvar: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    // Added return type
    let spl_ix = spl_ata::instruction::create_associated_token_account(
        &Address::from(payer.key.to_bytes()),
        &Address::from(owner.key.to_bytes()),
        &Address::from(mint.key.to_bytes()),
        &Address::from(token_program.key.to_bytes()),
    );

    let ix = Instruction {
        program_id: Pubkey::from(spl_ix.program_id.to_bytes()),
        accounts: spl_ix
            .accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: Pubkey::from(acc.pubkey.to_bytes()),
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect(),
        data: spl_ix.data,
    };

    invoke(
        &ix,
        &[
            payer.clone(),         // 0. Funding account
            ata.clone(),           // 1. ATA to create
            owner.clone(),         // 2. Wallet owner
            mint.clone(),          // 3. Mint
            sys_program.clone(),   // 4. System program
            token_program.clone(), // 5. Token program
            ata_program.clone(),
            rent_sysvar.clone(),
        ],
    )?;

    Ok(())
}
