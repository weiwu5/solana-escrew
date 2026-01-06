// Integration tests that run against a real test validator
// This ensures the BPF-compiled program works correctly
#![cfg(feature = "test-bpf")]

use {
    assert_matches::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};

/// Integration test with a local test validator
///
/// This test verifies that the BPF-compiled program can be deployed
/// and executed on a real (local) Solana validator. It:
/// 1. Starts a local test validator
/// 2. Deploys the BPF program to it
/// 3. Sends a transaction to invoke the program
/// 4. Verifies the transaction succeeds
///
/// This is more comprehensive than unit tests because it tests
/// the actual compiled BPF bytecode, not just the Rust code.
#[test]
fn test_validator_transaction() {
    // Set up logging to help debug test failures
    solana_logger::setup_with_default("solana_program_runtime=debug");

    // Generate a unique program ID for this test
    let program_id = Pubkey::new_unique();

    // Start a local test validator with our program deployed
    let (test_validator, payer) = TestValidatorGenesis::default()
        .add_program("bpf_program_template", program_id)
        .start();

    // Get an RPC client to interact with the test validator
    let rpc_client = test_validator.get_rpc_client();

    // Get the latest blockhash (required for transactions)
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    // Create a transaction that invokes our program
    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![AccountMeta::new(payer.pubkey(), false)],
            data: vec![1, 2, 3], // Sample instruction data
        }],
        Some(&payer.pubkey()),
    );

    // Sign the transaction with the payer's keypair
    transaction.sign(&[&payer], blockhash);

    // Send the transaction to the validator and verify it succeeds
    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
}
