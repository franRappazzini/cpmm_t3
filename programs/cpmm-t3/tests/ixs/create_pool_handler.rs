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

pub fn create_create_pool_ix(
    creator: &Keypair,
    global_config: Address,
    liquidity_pool: Address,
    mint_a: Address,
    mint_b: Address,
    lp_mint: Address,
    mint_a_vault: Address,
    mint_b_vault: Address,
    lp_mint_vault: Address,
    creator_a_ata: Address,
    creator_b_ata: Address,
    creator_lp_ata: Address,
    amount_a: u64,
    amount_b: u64,
) -> Instruction {
    Instruction::new_with_bytes(
        cpmm_t3::id(),
        &cpmm_t3::instruction::CreatePool { amount_a, amount_b }.data(),
        cpmm_t3::accounts::CreatePool {
            creator: creator.pubkey(),
            global_config,
            liquidity_pool,
            mint_a,
            mint_b,
            lp_mint,
            mint_a_vault,
            mint_b_vault,
            lp_mint_vault,
            creator_a_ata,
            creator_b_ata,
            creator_lp_ata,
            associated_token_program: associated_token::ID,
            token_program: token::ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}
