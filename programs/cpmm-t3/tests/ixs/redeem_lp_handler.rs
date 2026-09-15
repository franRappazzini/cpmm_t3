use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    anchor_spl::token,
    solana_keypair::Keypair,
    solana_message::Address,
    solana_signer::Signer,
};

pub fn create_redeem_lp_ix(
    redeemer: &Keypair,
    liquidity_pool: Address,
    mint_a: Address,
    mint_b: Address,
    lp_mint: Address,
    mint_a_vault: Address,
    mint_b_vault: Address,
    redeemer_a_ata: Address,
    redeemer_b_ata: Address,
    redeemer_lp_ata: Address,
    pool_id: u64,
    lp_amount: u64,
) -> Instruction {
    Instruction::new_with_bytes(
        cpmm_t3::id(),
        &cpmm_t3::instruction::RedeemLp { pool_id, lp_amount }.data(),
        cpmm_t3::accounts::RedeemLp {
            redeemer: redeemer.pubkey(),
            liquidity_pool,
            mint_a,
            mint_b,
            lp_mint,
            mint_a_vault,
            mint_b_vault,
            redeemer_a_ata,
            redeemer_b_ata,
            redeemer_lp_ata,
            token_program: token::ID,
        }
        .to_account_metas(None),
    )
}
