# Solana Escrow Program

A secure, trustless escrow service for atomic SOL exchanges between two parties on the Solana blockchain.

## Overview

This program implements a production-ready escrow system that enables safe peer-to-peer exchanges without requiring trust between parties. When the initializer deposits SOL into escrow, a taker can complete the exchange by sending an equal amount, triggering automatic fund distribution.

### Features

- ✅ Trustless SOL exchanges
- ✅ Atomic transactions (all-or-nothing)
- ✅ Cancel and refund functionality
- ✅ Secure state management
- ✅ Comprehensive test coverage
- ✅ Production-ready security checks

## How It Works

### The Escrow Flow

```
1. Initialize
   ├─ Initializer deposits SOL into escrow account
   ├─ Escrow state is created and marked as active
   └─ Funds are locked until exchange or cancellation

2. Exchange (Happy Path)
   ├─ Taker sends equal amount of SOL to initializer
   ├─ Taker receives the escrowed SOL
   ├─ Escrow is marked as completed
   └─ Both parties receive their funds atomically

3. Cancel (Alternative Path)
   ├─ Initializer decides to cancel
   ├─ Escrowed SOL is returned to initializer
   └─ Escrow is closed
```

### Participants

- **Initializer**: Creates the escrow and deposits SOL
- **Taker**: Completes the exchange by matching the deposited amount
- **Escrow Account**: Holds the deposited SOL securely

## Quick Start

### Prerequisites

Before you begin, ensure you have:

#### 1. Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 2. Solana CLI Tools
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

### Build the Program

```bash
# Native build (for testing)
cargo build

# BPF build (for deployment)
cargo build-bpf
```

### Run Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_initialize_escrow
```

## Program Architecture

### Instructions

The program supports three instructions:

#### 1. Initialize
Creates a new escrow and locks SOL.

**Accounts:**
- `[signer, writable]` Initializer's account
- `[writable]` Escrow state account (must be owned by program)
- `[]` System program

**Data:**
- `tag: u8` - Instruction discriminator (0 for Initialize)
- `amount: u64` - Amount of SOL to deposit (in lamports)

#### 2. Exchange
Completes the escrow by exchanging SOL between parties.

**Accounts:**
- `[signer, writable]` Taker's account
- `[writable]` Initializer's account
- `[writable]` Escrow state account
- `[]` System program

**Data:**
- `tag: u8` - Instruction discriminator (1 for Exchange)

#### 3. Cancel
Cancels the escrow and refunds the initializer.

**Accounts:**
- `[signer, writable]` Initializer's account
- `[writable]` Escrow state account
- `[]` System program

**Data:**
- `tag: u8` - Instruction discriminator (2 for Cancel)

### State Structure

```rust
pub struct EscrowState {
    pub initializer_pubkey: Pubkey,  // 32 bytes
    pub initializer_amount: u64,     // 8 bytes
    pub is_initialized: bool,        // 1 byte
}
// Total: 41 bytes
```

### Security Features

1. **Signature Verification**: All instructions verify signer authority
2. **Ownership Checks**: Escrow account ownership is validated
3. **State Validation**: Prevents double-initialization and unauthorized access
4. **Amount Validation**: Ensures positive amounts and sufficient balances
5. **Rent Exemption**: Maintains minimum balance for rent exemption
6. **Atomic Operations**: All transfers succeed or fail together

## Development

### Using the Makefile

```bash
# Show all available commands
make help

# Build and test
make all

# Quick development cycle
make dev

# Format code
make fmt

# Run linter
make lint

# Clean build artifacts
make clean
```

### Project Structure

```
solana-escrew/
├── src/
│   └── lib.rs              # Main escrow program implementation
├── tests/
│   └── integration.rs      # BPF integration tests
├── Cargo.toml              # Rust package manifest
├── Makefile                # Development commands
└── README.md               # This file
```

## Deployment

### Deploy to Devnet

```bash
# Configure Solana CLI for devnet
solana config set --url https://api.devnet.solana.com

# Create a new keypair (if needed)
solana-keygen new

# Airdrop SOL for deployment
solana airdrop 2

# Build the program
cargo build-bpf

# Deploy
solana program deploy target/deploy/bpf_program_template.so
```

The deployment will output your program ID. Save this for interacting with your program!

### Deploy to Mainnet

```bash
# Configure for mainnet-beta
solana config set --url https://api.mainnet-beta.solana.com

# Deploy (costs real SOL!)
solana program deploy target/deploy/bpf_program_template.so
```

**⚠️ Warning**: Deploying to mainnet costs real SOL. Ensure thorough testing on devnet first!

## Usage Examples

### Creating an Escrow (Client-side TypeScript example)

```typescript
import {
  Connection,
  PublicKey,
  Transaction,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';

// Initialize escrow with 1 SOL
const amount = 1 * LAMPORTS_PER_SOL;
const instructionData = Buffer.alloc(9);
instructionData.writeUInt8(0, 0); // Initialize instruction
instructionData.writeBigUInt64LE(BigInt(amount), 1);

const transaction = new Transaction().add({
  keys: [
    { pubkey: initializer.publicKey, isSigner: true, isWritable: true },
    { pubkey: escrowAccount.publicKey, isSigner: false, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ],
  programId: escrowProgramId,
  data: instructionData,
});
```

### Exchange

```typescript
// Complete the exchange
const instructionData = Buffer.alloc(1);
instructionData.writeUInt8(1, 0); // Exchange instruction

const transaction = new Transaction().add({
  keys: [
    { pubkey: taker.publicKey, isSigner: true, isWritable: true },
    { pubkey: initializer.publicKey, isSigner: false, isWritable: true },
    { pubkey: escrowAccount.publicKey, isSigner: false, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ],
  programId: escrowProgramId,
  data: instructionData,
});
```

### Cancel

```typescript
// Cancel escrow and get refund
const instructionData = Buffer.alloc(1);
instructionData.writeUInt8(2, 0); // Cancel instruction

const transaction = new Transaction().add({
  keys: [
    { pubkey: initializer.publicKey, isSigner: true, isWritable: true },
    { pubkey: escrowAccount.publicKey, isSigner: false, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ],
  programId: escrowProgramId,
  data: instructionData,
});
```

## Testing

The program includes comprehensive tests:

### Unit Tests
- Instruction unpacking
- State serialization/deserialization
- Business logic validation

### Integration Tests
- Full initialize-exchange flow
- Cancel and refund flow
- Error cases and edge conditions

Run tests with:
```bash
cargo test
```

## Common Commands

```bash
# Build
cargo build                  # Native build
cargo build-bpf             # BPF build
cargo build --release       # Release build

# Test
cargo test                   # All tests
cargo test --nocapture      # With output
cargo test test_initialize  # Specific test

# Development
cargo check                  # Fast compile check
cargo fmt                    # Format code
cargo clippy                 # Lint code

# Using Makefile
make build                   # Build native
make build-bpf              # Build BPF
make test                    # Run tests
make all                     # Format, lint, build, test
```

## Troubleshooting

### Build Errors

**"solana-program version mismatch"**
- Ensure Solana CLI version matches SDK version in Cargo.toml
- Update dependencies: `cargo update`

**"cargo: command not found"**
- Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Add to PATH: `source $HOME/.cargo/env`

### Runtime Errors

**"Insufficient funds"**
- Escrow balance must exceed rent-exempt minimum
- Ensure enough SOL for transaction fees

**"Account already initialized"**
- Escrow account is already in use
- Use a new account or cancel existing escrow

**"Only initializer can cancel"**
- Only the account that created the escrow can cancel it
- Verify signer matches initializer

## Security Considerations

### Auditing

This program demonstrates Solana development practices but should undergo professional security auditing before mainnet deployment with significant funds.

### Best Practices Implemented

1. ✅ Input validation on all instructions
2. ✅ Ownership and signer verification
3. ✅ Proper error handling
4. ✅ Rent-exemption maintenance
5. ✅ State machine integrity
6. ✅ No arithmetic overflows (using checked operations where needed)

### Known Limitations

- **Fixed exchange rate**: Currently 1:1 exchange ratio
- **No partial fills**: Must exchange entire amount
- **No timeout**: Escrows don't automatically expire
- **SOL only**: Doesn't support SPL tokens yet

## Future Enhancements

Potential improvements for production use:

- [ ] SPL token support
- [ ] Custom exchange ratios
- [ ] Time-locked escrows with automatic expiration
- [ ] Partial exchange amounts
- [ ] Multi-party escrows (more than 2 participants)
- [ ] Escrow marketplace UI
- [ ] Program Derived Address (PDA) for escrow accounts

## Resources

### Solana Documentation
- [Solana Docs](https://docs.solana.com/)
- [Solana Program Library](https://spl.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Anchor Framework](https://www.anchor-lang.com/)

### Learning
- [Solana Development Course](https://www.soldev.app/)
- [Program Examples](https://github.com/solana-labs/solana-program-library)
- [Escrow Tutorial](https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/)

### Community
- [Solana Stack Exchange](https://solana.stackexchange.com/)
- [Solana Discord](https://discord.gg/solana)
- [Solana Forum](https://forum.solana.com/)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

WTFPL (Do What The Fuck You Want To Public License)

This is free and unencumbered software released into the public domain.

## Acknowledgments

This implementation is inspired by:
- [Paul X's Escrow Tutorial](https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/)
- Solana Program Library patterns
- Community feedback and best practices

---

**Built with ❤️ for the Solana ecosystem**

For questions or issues, please open a GitHub issue or reach out on Discord.
