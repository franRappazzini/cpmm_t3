use {
    anchor_lang::{
        solana_program::{instruction::Instruction, system_program},
        InstructionData, ToAccountMetas,
    },
    solana_keypair::Keypair,
    solana_message::Address,
    solana_signer::Signer,
};

pub fn create_initialixe_ix(authority: &Keypair, global_config: Address) -> Instruction {
    Instruction::new_with_bytes(
        cpmm_t3::id(),
        &cpmm_t3::instruction::Initialize {
            protocol_fee_bps: 500,
            fee_bps: 500,
        }
        .data(),
        cpmm_t3::accounts::Initialize {
            authority: authority.pubkey(),
            global_config,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}
