use {
    crate::ixs::{
        create_add_liquidity_ix, create_create_pool_ix, create_initialixe_ix, create_redeem_lp_ix,
    },
    anchor_lang::{prelude::Pubkey, solana_program::instruction::Instruction},
    anchor_spl::associated_token,
    litesvm::LiteSVM,
    litesvm_token::{spl_token, CreateMint},
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
    std::println,
};
mod ixs;

fn send_tx(
    svm: &mut LiteSVM,
    ixs: &[Instruction],
    payer: &Keypair,
    signers: &[&Keypair],
) -> litesvm::types::TransactionResult {
    svm.expire_blockhash();
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(ixs, Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), signers).unwrap();
    svm.send_transaction(tx)
}

// Setup function to initialize LiteSVM and create a payer keypair
fn setup() -> (LiteSVM, Keypair, Keypair, Pubkey) {
    let program_id = cpmm_t3::id();
    let authority = Keypair::new();
    let creator = Keypair::new();
    let mut svm = LiteSVM::new();
    let program_bytes = include_bytes!("../../../target/deploy/cpmm_t3.so");
    svm.add_program(program_id, program_bytes).unwrap();
    svm.airdrop(&authority.pubkey(), 1_000_000_000).unwrap();
    svm.airdrop(&creator.pubkey(), 1_000_000_000).unwrap();

    // PDAs
    let global_config =
        Pubkey::find_program_address(&[cpmm_t3::constants::GLOBAL_CONFIG_SEED], &program_id).0;

    (svm, authority, creator, global_config)
}

fn pdas() -> (Pubkey, Pubkey) {
    let program_id = cpmm_t3::ID;

    let global_config =
        Pubkey::find_program_address(&[cpmm_t3::constants::GLOBAL_CONFIG_SEED], &program_id).0;

    let liquidity_pool = Pubkey::find_program_address(
        &[cpmm_t3::constants::LIQUIDITY_POOL_SEED, &0u64.to_le_bytes()],
        &program_id,
    )
    .0;

    // mint_a: Address,
    // mint_b: Address,
    // lp_mint: Address,
    // mint_a_vault: Address,
    // mint_b_vault: Address,
    // lp_mint_vault: Address,
    // creator_a_ata: Address,
    // creator_b_ata: Address,
    // creator_lp_ata: Address,
    // associated_token_program: Address,

    (global_config, liquidity_pool)
}

fn mints(
    mut svm: &mut LiteSVM,
    creator: &Keypair,
    liquidity_pool: &Pubkey,
) -> (
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
) {
    let mint_a = CreateMint::new(&mut svm, &creator)
        .decimals(6)
        .authority(&creator.pubkey())
        .send()
        .unwrap();

    let mint_b = CreateMint::new(&mut svm, &creator)
        .decimals(6)
        .authority(&creator.pubkey())
        .send()
        .unwrap();

    let lp_mint = CreateMint::new(&mut svm, &creator)
        .decimals(6)
        .authority(&creator.pubkey())
        .send()
        .unwrap();

    let vault_a = associated_token::get_associated_token_address(liquidity_pool, &mint_a);
    let vault_b = associated_token::get_associated_token_address(liquidity_pool, &mint_b);
    let vault_lp = associated_token::get_associated_token_address(liquidity_pool, &lp_mint);

    let creator_ata_a = associated_token::get_associated_token_address(&creator.pubkey(), &mint_a);
    let creator_ata_b = associated_token::get_associated_token_address(&creator.pubkey(), &mint_b);
    let creator_ata_lp =
        associated_token::get_associated_token_address(&creator.pubkey(), &lp_mint);

    (
        mint_a,
        mint_b,
        lp_mint,
        vault_a,
        vault_b,
        vault_lp,
        creator_ata_a,
        creator_ata_b,
        creator_ata_lp,
    )
}

// --- tests ---

#[test]
fn initialize() {
    let (mut svm, authority, _, global_config) = setup();

    let ix = create_initialixe_ix(&authority, global_config);
    let sig = send_tx(&mut svm, &[ix], &authority, &[&authority]);
    println!("{:#?}", sig);
    assert!(sig.is_ok());
}

#[test]
fn create_pool() {
    let (mut svm, authority, creator, global_config) = setup();

    let ix1 = create_initialixe_ix(&authority, global_config);

    let (global_config, liquidity_pool) = pdas();

    let (
        mint_a,
        mint_b,
        lp_mint,
        vault_a,
        vault_b,
        vault_lp,
        creator_ata_a,
        creator_ata_b,
        creator_ata_lp,
    ) = mints(&mut svm, &creator, &liquidity_pool);

    let ix2 = create_create_pool_ix(
        &creator,
        global_config,
        liquidity_pool,
        mint_a,
        mint_b,
        lp_mint,
        vault_a,
        vault_b,
        vault_lp,
        creator_ata_a,
        creator_ata_b,
        creator_ata_lp,
        10_000_000,
        20_000_000,
    );

    let sig = send_tx(&mut svm, &[ix1, ix2], &creator, &[&creator]);
    println!("{:#?}", sig);
    assert!(sig.is_ok());
}

#[test]
fn add_liquidity() {
    let (mut svm, authority, creator, global_config) = setup();

    let ix1 = create_initialixe_ix(&authority, global_config);

    let (global_config, liquidity_pool) = pdas();

    let (
        mint_a,
        mint_b,
        lp_mint,
        vault_a,
        vault_b,
        vault_lp,
        creator_ata_a,
        creator_ata_b,
        creator_ata_lp,
    ) = mints(&mut svm, &creator, &liquidity_pool);

    let ix2 = create_create_pool_ix(
        &creator,
        global_config,
        liquidity_pool,
        mint_a,
        mint_b,
        lp_mint,
        vault_a,
        vault_b,
        vault_lp,
        creator_ata_a,
        creator_ata_b,
        creator_ata_lp,
        10_000_000,
        20_000_000,
    );

    let ix3 = create_add_liquidity_ix(
        &creator,
        liquidity_pool,
        mint_a,
        mint_b,
        lp_mint,
        vault_a,
        vault_b,
        creator_ata_a,
        creator_ata_b,
        creator_ata_lp,
        0,
        500_000_000,
        1_000_000_000,
        0,
    );

    let sig = send_tx(&mut svm, &[ix1, ix2, ix3], &creator, &[&creator]);
    println!("{:#?}", sig);
    assert!(sig.is_ok());
}

#[test]
fn redeem_lp() {
    let (mut svm, authority, creator, global_config) = setup();

    let ix1 = create_initialixe_ix(&authority, global_config);

    let (global_config, liquidity_pool) = pdas();

    let (
        mint_a,
        mint_b,
        lp_mint,
        vault_a,
        vault_b,
        vault_lp,
        creator_ata_a,
        creator_ata_b,
        creator_ata_lp,
    ) = mints(&mut svm, &creator, &liquidity_pool);

    let ix2 = create_create_pool_ix(
        &creator,
        global_config,
        liquidity_pool,
        mint_a,
        mint_b,
        lp_mint,
        vault_a,
        vault_b,
        vault_lp,
        creator_ata_a,
        creator_ata_b,
        creator_ata_lp,
        10_000_000,
        20_000_000,
    );

    let ix3 = create_add_liquidity_ix(
        &creator,
        liquidity_pool,
        mint_a,
        mint_b,
        lp_mint,
        vault_a,
        vault_b,
        creator_ata_a,
        creator_ata_b,
        creator_ata_lp,
        0,
        500_000_000,
        1_000_000_000,
        0,
    );

    let creator_ata_lp_account: spl_token::state::Account =
        litesvm_token::get_spl_account(&svm, &creator_ata_lp).unwrap();

    let ix4 = create_redeem_lp_ix(
        &creator,
        liquidity_pool,
        mint_a,
        mint_b,
        lp_mint,
        vault_a,
        vault_b,
        creator_ata_a,
        creator_ata_b,
        creator_ata_lp,
        0,
        creator_ata_lp_account.amount / 2,
    );

    let sig = send_tx(&mut svm, &[ix1, ix2, ix3, ix4], &creator, &[&creator]);
    println!("{:#?}", sig);
    assert!(sig.is_ok());
}
