use {
    anchor_lang::{
        solana_program::{instruction::Instruction, system_program},
        InstructionData, ToAccountMetas,
    },
    anchor_spl::{associated_token, token},
    solana_keypair::Keypair,
    solana_message::Address,
    solana_signer::Signer,
};

pub fn create_add_liquidity_ix(
    provider: &Keypair,
    liquidity_pool: Address,
    mint_a: Address,
    mint_b: Address,
    lp_mint: Address,
    mint_a_vault: Address,
    mint_b_vault: Address,
    provider_a_ata: Address,
    provider_b_ata: Address,
    provider_lp_ata: Address,
    pool_id: u64,
    amount_a: u64,
    amount_b: u64,
    min_lp_out: u64,
) -> Instruction {
    Instruction::new_with_bytes(
        cpmm_t3::id(),
        &cpmm_t3::instruction::AddLiquidity {
            pool_id,
            amount_a,
            amount_b,
            min_lp_out,
        }
        .data(),
        cpmm_t3::accounts::AddLiquidity {
            provider: provider.pubkey(),
            liquidity_pool,
            mint_a,
            mint_b,
            lp_mint,
            mint_a_vault,
            mint_b_vault,
            provider_a_ata,
            provider_b_ata,
            provider_lp_ata,
            associated_token_program: associated_token::ID,
            token_program: token::ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}
