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

pub fn create_withdraw_treasury_ix(
    authority: &Keypair,
    global_config: Address,
    liquidity_pool: Address,
    mint_a: Address,
    mint_b: Address,
    mint_a_vault: Address,
    mint_b_vault: Address,
    authority_a_ata: Address,
    authority_b_ata: Address,
    pool_id: u64,
) -> Instruction {
    Instruction::new_with_bytes(
        cpmm_t3::id(),
        &cpmm_t3::instruction::WithdrawTreasury { pool_id }.data(),
        cpmm_t3::accounts::WithdrawTreasury {
            authority: authority.pubkey(),
            global_config,
            liquidity_pool,
            mint_a,
            mint_b,
            mint_a_vault,
            mint_b_vault,
            authority_a_ata,
            authority_b_ata,
            associated_token_program: associated_token::ID,
            token_program: token::ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}
