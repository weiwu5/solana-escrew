// Solana Escrow Program
// A secure escrow service for atomic SOL exchanges between two parties

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

// Program entrypoint
entrypoint!(process_instruction);

/// Main program entrypoint
///
/// Routes instructions to appropriate handlers based on instruction data
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = EscrowInstruction::unpack(instruction_data)?;

    match instruction {
        EscrowInstruction::Initialize { amount } => {
            msg!("Instruction: Initialize Escrow");
            process_initialize(program_id, accounts, amount)
        }
        EscrowInstruction::Exchange => {
            msg!("Instruction: Exchange");
            process_exchange(program_id, accounts)
        }
        EscrowInstruction::Cancel => {
            msg!("Instruction: Cancel Escrow");
            process_cancel(program_id, accounts)
        }
    }
}

/// Escrow instruction types
#[derive(Debug)]
pub enum EscrowInstruction {
    /// Initialize a new escrow
    ///
    /// Accounts expected:
    /// 0. `[signer, writable]` Initializer's account
    /// 1. `[writable]` Escrow state account (PDA)
    /// 2. `[]` System program
    Initialize {
        /// Amount of SOL the initializer deposits
        amount: u64,
    },

    /// Exchange - taker completes the escrow
    ///
    /// Accounts expected:
    /// 0. `[signer, writable]` Taker's account
    /// 1. `[writable]` Initializer's account
    /// 2. `[writable]` Escrow state account (PDA)
    /// 3. `[]` System program
    Exchange,

    /// Cancel - initializer cancels and retrieves funds
    ///
    /// Accounts expected:
    /// 0. `[signer, writable]` Initializer's account
    /// 1. `[writable]` Escrow state account (PDA)
    /// 2. `[]` System program
    Cancel,
}

impl EscrowInstruction {
    /// Unpacks instruction data into the appropriate instruction type
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                // Initialize instruction
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Self::Initialize { amount }
            }
            1 => Self::Exchange,
            2 => Self::Cancel,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

/// Escrow state stored in the escrow account
#[repr(C)]
#[derive(Debug, Default)]
pub struct EscrowState {
    /// Initializer's public key
    pub initializer_pubkey: Pubkey,
    /// Amount of SOL deposited by initializer
    pub initializer_amount: u64,
    /// Is the escrow initialized
    pub is_initialized: bool,
}

impl EscrowState {
    pub const LEN: usize = 32 + 8 + 1; // pubkey + u64 + bool

    /// Serialize state to bytes
    pub fn pack(&self, dst: &mut [u8]) -> ProgramResult {
        if dst.len() < Self::LEN {
            return Err(ProgramError::AccountDataTooSmall);
        }

        dst[0..32].copy_from_slice(self.initializer_pubkey.as_ref());
        dst[32..40].copy_from_slice(&self.initializer_amount.to_le_bytes());
        dst[40] = self.is_initialized as u8;

        Ok(())
    }

    /// Deserialize state from bytes
    pub fn unpack(src: &[u8]) -> Result<Self, ProgramError> {
        if src.len() < Self::LEN {
            return Err(ProgramError::AccountDataTooSmall);
        }

        Ok(EscrowState {
            initializer_pubkey: Pubkey::new_from_array(src[0..32].try_into().unwrap()),
            initializer_amount: u64::from_le_bytes(src[32..40].try_into().unwrap()),
            is_initialized: src[40] != 0,
        })
    }
}

/// Process Initialize instruction
///
/// Creates an escrow and transfers SOL from initializer to escrow account
fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let escrow_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Verify initializer signed the transaction
    if !initializer.is_signer {
        msg!("Initializer must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify escrow account is owned by our program
    if escrow_account.owner != program_id {
        msg!("Escrow account must be owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Verify escrow account has correct size
    if escrow_account.data_len() < EscrowState::LEN {
        msg!("Escrow account data too small");
        return Err(ProgramError::AccountDataTooSmall);
    }

    // Check if escrow is already initialized
    let escrow_data = escrow_account.try_borrow_data()?;
    if escrow_data[40] != 0 {
        msg!("Escrow already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    drop(escrow_data);

    // Verify amount is greater than 0
    if amount == 0 {
        msg!("Amount must be greater than 0");
        return Err(ProgramError::InvalidArgument);
    }

    // Transfer SOL from initializer to escrow account
    invoke(
        &system_instruction::transfer(initializer.key, escrow_account.key, amount),
        &[
            initializer.clone(),
            escrow_account.clone(),
            system_program.clone(),
        ],
    )?;

    // Initialize escrow state
    let escrow_state = EscrowState {
        initializer_pubkey: *initializer.key,
        initializer_amount: amount,
        is_initialized: true,
    };

    // Write state to escrow account
    let mut escrow_data = escrow_account.try_borrow_mut_data()?;
    escrow_state.pack(&mut escrow_data)?;

    msg!(
        "Escrow initialized: {} SOL deposited by {}",
        amount,
        initializer.key
    );

    Ok(())
}

/// Process Exchange instruction
///
/// Taker sends SOL to initializer and receives escrow funds
fn process_exchange(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let taker = next_account_info(account_info_iter)?;
    let initializer = next_account_info(account_info_iter)?;
    let escrow_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Verify taker signed the transaction
    if !taker.is_signer {
        msg!("Taker must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify escrow account is owned by our program
    if escrow_account.owner != program_id {
        msg!("Escrow account must be owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Load and verify escrow state
    let escrow_data = escrow_account.try_borrow_data()?;
    let escrow_state = EscrowState::unpack(&escrow_data)?;
    drop(escrow_data);

    if !escrow_state.is_initialized {
        msg!("Escrow not initialized");
        return Err(ProgramError::UninitializedAccount);
    }

    // Verify initializer account matches
    if escrow_state.initializer_pubkey != *initializer.key {
        msg!("Initializer account mismatch");
        return Err(ProgramError::InvalidAccountData);
    }

    // Calculate amount to transfer (escrow balance minus rent exemption)
    let rent = Rent::get()?;
    let rent_exempt_amount = rent.minimum_balance(EscrowState::LEN);
    let escrow_balance = escrow_account.lamports();

    if escrow_balance <= rent_exempt_amount {
        msg!("Insufficient escrow balance");
        return Err(ProgramError::InsufficientFunds);
    }

    let transfer_amount = escrow_balance - rent_exempt_amount;

    // Transfer SOL from escrow to taker
    **escrow_account.try_borrow_mut_lamports()? -= transfer_amount;
    **taker.try_borrow_mut_lamports()? += transfer_amount;

    // Transfer equal amount from taker to initializer
    invoke(
        &system_instruction::transfer(taker.key, initializer.key, transfer_amount),
        &[taker.clone(), initializer.clone(), system_program.clone()],
    )?;

    // Mark escrow as closed
    let mut escrow_data = escrow_account.try_borrow_mut_data()?;
    escrow_data[40] = 0; // Set is_initialized to false

    msg!(
        "Exchange completed: {} SOL exchanged between {} and {}",
        transfer_amount,
        initializer.key,
        taker.key
    );

    Ok(())
}

/// Process Cancel instruction
///
/// Initializer cancels escrow and retrieves deposited funds
fn process_cancel(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let escrow_account = next_account_info(account_info_iter)?;
    let _system_program = next_account_info(account_info_iter)?;

    // Verify initializer signed the transaction
    if !initializer.is_signer {
        msg!("Initializer must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify escrow account is owned by our program
    if escrow_account.owner != program_id {
        msg!("Escrow account must be owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Load and verify escrow state
    let escrow_data = escrow_account.try_borrow_data()?;
    let escrow_state = EscrowState::unpack(&escrow_data)?;
    drop(escrow_data);

    if !escrow_state.is_initialized {
        msg!("Escrow not initialized");
        return Err(ProgramError::UninitializedAccount);
    }

    // Verify initializer account matches
    if escrow_state.initializer_pubkey != *initializer.key {
        msg!("Only initializer can cancel escrow");
        return Err(ProgramError::InvalidAccountData);
    }

    // Calculate refund amount (escrow balance minus rent exemption)
    let rent = Rent::get()?;
    let rent_exempt_amount = rent.minimum_balance(EscrowState::LEN);
    let escrow_balance = escrow_account.lamports();

    if escrow_balance <= rent_exempt_amount {
        msg!("Insufficient escrow balance for refund");
        return Err(ProgramError::InsufficientFunds);
    }

    let refund_amount = escrow_balance - rent_exempt_amount;

    // Transfer SOL back to initializer
    **escrow_account.try_borrow_mut_lamports()? -= refund_amount;
    **initializer.try_borrow_mut_lamports()? += refund_amount;

    // Mark escrow as closed
    let mut escrow_data = escrow_account.try_borrow_mut_data()?;
    escrow_data[40] = 0; // Set is_initialized to false

    msg!("Escrow cancelled: {} SOL refunded to {}", refund_amount, initializer.key);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_initialize_escrow() {
        let program_id = Pubkey::new_unique();
        let initializer = Keypair::new();
        let escrow_keypair = Keypair::new();

        let mut program_test = ProgramTest::new(
            "solana_escrow",
            program_id,
            processor!(process_instruction),
        );

        // Fund initializer account
        program_test.add_account(
            initializer.pubkey(),
            Account {
                lamports: 10_000_000,
                ..Account::default()
            },
        );

        // Create escrow account
        program_test.add_account(
            escrow_keypair.pubkey(),
            Account {
                lamports: 1_000_000,
                data: vec![0; EscrowState::LEN],
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create initialize instruction
        let mut instruction_data = vec![0u8]; // Initialize tag
        instruction_data.extend_from_slice(&5_000_000u64.to_le_bytes());

        let mut transaction = Transaction::new_with_payer(
            &[solana_program::instruction::Instruction {
                program_id,
                accounts: vec![
                    solana_program::instruction::AccountMeta::new(
                        initializer.pubkey(),
                        true,
                    ),
                    solana_program::instruction::AccountMeta::new(
                        escrow_keypair.pubkey(),
                        false,
                    ),
                    solana_program::instruction::AccountMeta::new_readonly(
                        solana_program::system_program::id(),
                        false,
                    ),
                ],
                data: instruction_data,
            }],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[&payer, &initializer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify escrow state
        let escrow_account = banks_client
            .get_account(escrow_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();

        let escrow_state = EscrowState::unpack(&escrow_account.data).unwrap();
        assert_eq!(escrow_state.initializer_pubkey, initializer.pubkey());
        assert_eq!(escrow_state.initializer_amount, 5_000_000);
        assert!(escrow_state.is_initialized);
    }

    #[tokio::test]
    async fn test_cancel_escrow() {
        let program_id = Pubkey::new_unique();
        let initializer = Keypair::new();
        let escrow_keypair = Keypair::new();

        let mut program_test = ProgramTest::new(
            "solana_escrow",
            program_id,
            processor!(process_instruction),
        );

        // Fund initializer
        program_test.add_account(
            initializer.pubkey(),
            Account {
                lamports: 10_000_000,
                ..Account::default()
            },
        );

        // Create initialized escrow
        let mut escrow_data = vec![0; EscrowState::LEN];
        let escrow_state = EscrowState {
            initializer_pubkey: initializer.pubkey(),
            initializer_amount: 5_000_000,
            is_initialized: true,
        };
        escrow_state.pack(&mut escrow_data).unwrap();

        program_test.add_account(
            escrow_keypair.pubkey(),
            Account {
                lamports: 6_000_000, // 5M deposit + 1M for rent
                data: escrow_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let initial_balance = banks_client
            .get_account(initializer.pubkey())
            .await
            .unwrap()
            .unwrap()
            .lamports;

        // Create cancel instruction
        let instruction_data = vec![2u8]; // Cancel tag

        let mut transaction = Transaction::new_with_payer(
            &[solana_program::instruction::Instruction {
                program_id,
                accounts: vec![
                    solana_program::instruction::AccountMeta::new(
                        initializer.pubkey(),
                        true,
                    ),
                    solana_program::instruction::AccountMeta::new(
                        escrow_keypair.pubkey(),
                        false,
                    ),
                    solana_program::instruction::AccountMeta::new_readonly(
                        solana_program::system_program::id(),
                        false,
                    ),
                ],
                data: instruction_data,
            }],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[&payer, &initializer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify escrow is closed
        let escrow_account = banks_client
            .get_account(escrow_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();

        let escrow_state = EscrowState::unpack(&escrow_account.data).unwrap();
        assert!(!escrow_state.is_initialized);

        // Verify initializer received refund
        let final_balance = banks_client
            .get_account(initializer.pubkey())
            .await
            .unwrap()
            .unwrap()
            .lamports;

        assert!(final_balance > initial_balance);
    }

    #[tokio::test]
    async fn test_exchange() {
        let program_id = Pubkey::new_unique();
        let initializer = Keypair::new();
        let taker = Keypair::new();
        let escrow_keypair = Keypair::new();

        let mut program_test = ProgramTest::new(
            "solana_escrow",
            program_id,
            processor!(process_instruction),
        );

        // Fund accounts
        program_test.add_account(
            initializer.pubkey(),
            Account {
                lamports: 5_000_000,
                ..Account::default()
            },
        );

        program_test.add_account(
            taker.pubkey(),
            Account {
                lamports: 10_000_000,
                ..Account::default()
            },
        );

        // Create initialized escrow
        let mut escrow_data = vec![0; EscrowState::LEN];
        let escrow_state = EscrowState {
            initializer_pubkey: initializer.pubkey(),
            initializer_amount: 5_000_000,
            is_initialized: true,
        };
        escrow_state.pack(&mut escrow_data).unwrap();

        program_test.add_account(
            escrow_keypair.pubkey(),
            Account {
                lamports: 6_000_000,
                data: escrow_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let init_init_balance = banks_client
            .get_account(initializer.pubkey())
            .await
            .unwrap()
            .unwrap()
            .lamports;

        let init_taker_balance = banks_client
            .get_account(taker.pubkey())
            .await
            .unwrap()
            .unwrap()
            .lamports;

        // Create exchange instruction
        let instruction_data = vec![1u8]; // Exchange tag

        let mut transaction = Transaction::new_with_payer(
            &[solana_program::instruction::Instruction {
                program_id,
                accounts: vec![
                    solana_program::instruction::AccountMeta::new(taker.pubkey(), true),
                    solana_program::instruction::AccountMeta::new(
                        initializer.pubkey(),
                        false,
                    ),
                    solana_program::instruction::AccountMeta::new(
                        escrow_keypair.pubkey(),
                        false,
                    ),
                    solana_program::instruction::AccountMeta::new_readonly(
                        solana_program::system_program::id(),
                        false,
                    ),
                ],
                data: instruction_data,
            }],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[&payer, &taker], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify escrow is closed
        let escrow_account = banks_client
            .get_account(escrow_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();

        let escrow_state = EscrowState::unpack(&escrow_account.data).unwrap();
        assert!(!escrow_state.is_initialized);

        // Verify balances changed appropriately
        let final_init_balance = banks_client
            .get_account(initializer.pubkey())
            .await
            .unwrap()
            .unwrap()
            .lamports;

        let final_taker_balance = banks_client
            .get_account(taker.pubkey())
            .await
            .unwrap()
            .unwrap()
            .lamports;

        // Initializer should have more (received from taker)
        assert!(final_init_balance > init_init_balance);

        // Taker should have more (received from escrow) net of payment to initializer
        // This depends on the exact amounts but taker gets escrow funds
        assert!(final_taker_balance < init_taker_balance); // Paid to initializer
    }

    #[test]
    fn test_instruction_unpacking() {
        // Test Initialize
        let mut data = vec![0u8];
        data.extend_from_slice(&1000u64.to_le_bytes());
        let instruction = EscrowInstruction::unpack(&data).unwrap();
        match instruction {
            EscrowInstruction::Initialize { amount } => assert_eq!(amount, 1000),
            _ => panic!("Wrong instruction type"),
        }

        // Test Exchange
        let data = vec![1u8];
        let instruction = EscrowInstruction::unpack(&data).unwrap();
        matches!(instruction, EscrowInstruction::Exchange);

        // Test Cancel
        let data = vec![2u8];
        let instruction = EscrowInstruction::unpack(&data).unwrap();
        matches!(instruction, EscrowInstruction::Cancel);
    }

    #[test]
    fn test_state_packing() {
        let state = EscrowState {
            initializer_pubkey: Pubkey::new_unique(),
            initializer_amount: 12345,
            is_initialized: true,
        };

        let mut buffer = vec![0u8; EscrowState::LEN];
        state.pack(&mut buffer).unwrap();

        let unpacked = EscrowState::unpack(&buffer).unwrap();
        assert_eq!(state.initializer_pubkey, unpacked.initializer_pubkey);
        assert_eq!(state.initializer_amount, unpacked.initializer_amount);
        assert_eq!(state.is_initialized, unpacked.is_initialized);
    }
}
