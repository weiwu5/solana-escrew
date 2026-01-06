// Import necessary types from the Solana Program SDK
use solana_program::{
    account_info::AccountInfo, // Contains account metadata and data
    entrypoint,                  // Macro to define the program entrypoint
    entrypoint::ProgramResult,   // Result type for program execution
    msg,                          // Logging macro for on-chain messages
    pubkey::Pubkey,              // Public key type for addresses
};

// Define the program entrypoint. This macro sets up the necessary boilerplate
// to make the process_instruction function callable by the Solana runtime.
entrypoint!(process_instruction);

/// Main program entrypoint
///
/// This function is called by the Solana runtime whenever a transaction
/// invokes this program. It receives the program's ID, a list of accounts,
/// and arbitrary instruction data.
///
/// # Arguments
///
/// * `program_id` - The public key of the currently executing program
/// * `accounts` - A slice of accounts that the instruction references
/// * `instruction_data` - The instruction data provided by the caller
///
/// # Returns
///
/// * `ProgramResult` - Ok(()) on success, or a ProgramError on failure
///
/// # Example
///
/// This template implementation simply logs the inputs and returns success.
/// In a real program, you would:
/// 1. Parse the instruction_data to determine what operation to perform
/// 2. Validate the accounts array contains the expected accounts
/// 3. Perform the requested operation (state changes, transfers, etc.)
/// 4. Return Ok(()) on success or an appropriate error
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Log program invocation details to the Solana logs
    // This is useful for debugging and monitoring
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    // TODO: Add your program logic here
    // Example operations you might implement:
    // - Parse instruction_data to determine the operation type
    // - Validate accounts (check signers, ownership, etc.)
    // - Read/write account data
    // - Transfer SOL or tokens
    // - Invoke other programs (Cross-Program Invocation)

    Ok(())
}

// Unit tests module - only compiled when running tests
#[cfg(test)]
mod test {
    use {
        super::*,
        assert_matches::*,
        solana_program::instruction::{AccountMeta, Instruction},
        solana_program_test::*,
        solana_sdk::{signature::Signer, transaction::Transaction},
    };

    /// Test basic program invocation
    ///
    /// This test verifies that the program can be invoked successfully
    /// with a simple instruction. It:
    /// 1. Creates a test environment with the program loaded
    /// 2. Constructs a transaction that calls the program
    /// 3. Submits the transaction and verifies it succeeds
    #[tokio::test]
    async fn test_transaction() {
        // Generate a unique program ID for this test
        let program_id = Pubkey::new_unique();

        // Initialize the test environment with our program
        // This creates an in-memory banks client that simulates the Solana runtime
        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        // Create a transaction that invokes our program
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![1, 2, 3], // Sample instruction data
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        // Submit the transaction and verify it succeeds
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    /// Test program invocation with empty instruction data
    ///
    /// Verifies that the program handles empty instruction data correctly
    #[tokio::test]
    async fn test_empty_instruction_data() {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![], // Empty instruction data
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    /// Test program invocation with no accounts
    ///
    /// Verifies that the program handles instructions with no accounts
    #[tokio::test]
    async fn test_no_accounts() {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![], // No accounts
                data: vec![1, 2, 3],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    /// Test program invocation with multiple accounts
    ///
    /// Verifies that the program can handle multiple account references
    #[tokio::test]
    async fn test_multiple_accounts() {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        // Create additional accounts for testing
        let account1 = Pubkey::new_unique();
        let account2 = Pubkey::new_unique();
        let account3 = Pubkey::new_unique();

        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new(account1, false),
                    AccountMeta::new_readonly(account2, false),
                    AccountMeta::new_readonly(account3, false),
                ],
                data: vec![1, 2, 3],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    /// Test program invocation with large instruction data
    ///
    /// Verifies that the program can handle larger instruction data payloads
    #[tokio::test]
    async fn test_large_instruction_data() {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        // Create a larger instruction data payload (1KB)
        let large_data = vec![42u8; 1024];

        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: large_data,
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    /// Test multiple program invocations in single transaction
    ///
    /// Verifies that the program can be invoked multiple times in one transaction
    #[tokio::test]
    async fn test_multiple_invocations() {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        // Create a transaction with multiple instructions to the same program
        let mut transaction = Transaction::new_with_payer(
            &[
                Instruction {
                    program_id,
                    accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                    data: vec![1],
                },
                Instruction {
                    program_id,
                    accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                    data: vec![2],
                },
                Instruction {
                    program_id,
                    accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                    data: vec![3],
                },
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
}
