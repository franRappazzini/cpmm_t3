use {
    anchor_lang::{
        prelude::system_program, solana_program::instruction::Instruction, InstructionData,
        ToAccountMetas,
    },
    anchor_spl::{associated_token, token},
    solana_keypair::Keypair,
    solana_message::Address,
    solana_signer::Signer,
};

pub fn create_swap_ix(
    signer: &Keypair,
    liquidity_pool: Address,
    input_mint: Address,
    output_mint: Address,
    input_mint_vault: Address,
    output_mint_vault: Address,
    signer_input_ata: Address,
    signer_output_ata: Address,
    pool_id: u64,
    params: cpmm_t3::SwapParams,
) -> Instruction {
    Instruction::new_with_bytes(
        cpmm_t3::id(),
        &cpmm_t3::instruction::Swap { pool_id, params }.data(),
        cpmm_t3::accounts::Swap {
            signer: signer.pubkey(),
            liquidity_pool,
            input_mint,
            output_mint,
            input_mint_vault,
            output_mint_vault,
            signer_input_ata,
            signer_output_ata,
            associated_token_program: associated_token::ID,
            token_program: token::ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}
