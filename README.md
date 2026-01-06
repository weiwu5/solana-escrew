# Solana BPF Program Template

A minimal, production-ready template for building Solana blockchain programs (smart contracts) using Rust and the Solana Program SDK.

## Overview

This template provides a clean starting point for developing Solana programs that run on the Berkley Packet Filter (BPF) runtime. It includes:

- ✅ Basic program structure with entrypoint
- ✅ Unit and integration tests
- ✅ BPF compilation support
- ✅ Development utilities and scripts

## Prerequisites

Before you begin, ensure you have the following installed:

### 1. Rust
Install Rust using rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

### 2. Solana CLI Tools
Install Solana CLI tools:
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

Add Solana to your PATH (add this to your `~/.bashrc` or `~/.zshrc`):
```bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

Verify installation:
```bash
solana --version
solana-test-validator --version
```

### 3. Additional Tools (Optional but Recommended)
```bash
# For better development experience
cargo install cargo-watch  # Auto-rebuild on file changes
```

## Quick Start

### Clone and Setup
```bash
git clone <your-repo-url>
cd solana-escrew
```

### Build the Program

#### Native Build (for testing)
```bash
cargo build
```

#### BPF Build (for deployment)
```bash
cargo build-bpf
```

The compiled BPF program will be located at:
```
target/deploy/bpf_program_template.so
```

### Run Tests

#### Unit Tests
```bash
cargo test
```

#### BPF Integration Tests
```bash
cargo test-bpf
```

This will:
1. Build the program for BPF
2. Start a local test validator
3. Deploy the program
4. Run integration tests

## Project Structure

```
solana-escrew/
├── src/
│   └── lib.rs              # Main program entrypoint and logic
├── tests/
│   └── integration.rs      # BPF integration tests
├── scripts/
│   ├── patch.crates-io.sh  # Local Solana monorepo development
│   └── update-solana-dependencies.sh  # Update Solana versions
├── Cargo.toml              # Rust package manifest
├── README.md               # This file
└── .gitignore             # Git ignore rules
```

## Development Workflow

### 1. Develop Your Program
Edit `src/lib.rs` to implement your program logic. The entrypoint function receives:
- `program_id`: Your program's public key
- `accounts`: Array of accounts passed to the program
- `instruction_data`: Arbitrary instruction data

### 2. Write Tests
- Add unit tests in `src/lib.rs` under `#[cfg(test)]`
- Add integration tests in `tests/integration.rs`

### 3. Build and Test
```bash
# Quick development cycle
cargo build && cargo test

# Full BPF test
cargo build-bpf && cargo test-bpf
```

### 4. Deploy to Devnet (Optional)
```bash
# Configure Solana CLI for devnet
solana config set --url https://api.devnet.solana.com

# Airdrop SOL for deployment
solana airdrop 2

# Deploy your program
solana program deploy target/deploy/bpf_program_template.so
```

## Program Architecture

This template uses Solana's `entrypoint!` macro to define the program's entry point. When a transaction calls your program, Solana executes `process_instruction()`.

### Current Implementation
The template includes a minimal implementation that:
- Logs the program ID, number of accounts, and instruction data
- Returns `Ok(())` without performing any operations

### Extending the Template

To build a real program, you'll typically:

1. **Define instruction types** (e.g., Initialize, Transfer, Update)
2. **Parse instruction data** to determine which instruction to execute
3. **Validate accounts** to ensure they meet your requirements
4. **Perform operations** (read/write account data, transfer SOL, etc.)
5. **Return results** or error codes

Example structure:
```rust
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Parse instruction
    let instruction = MyInstruction::unpack(instruction_data)?;

    // Route to handler
    match instruction {
        MyInstruction::Initialize { .. } => process_initialize(accounts, ...),
        MyInstruction::Transfer { .. } => process_transfer(accounts, ...),
    }
}
```

## Common Commands

```bash
# Build for native testing
cargo build

# Run native tests
cargo test

# Build for BPF (blockchain deployment)
cargo build-bpf

# Run BPF integration tests
cargo test-bpf

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Watch mode (requires cargo-watch)
cargo watch -x build
```

## Troubleshooting

### Build Errors

**Error: "solana-program version mismatch"**
- Ensure your Solana CLI version matches the SDK version in `Cargo.toml`
- Run `scripts/update-solana-dependencies.sh` to update dependencies

**Error: "cargo: command not found"**
- Ensure Rust is installed and in your PATH
- Run `source $HOME/.cargo/env`

### Test Errors

**Error: "test-bpf requires BPF build"**
- Run `cargo build-bpf` before `cargo test-bpf`

**Error: "Connection refused" during tests**
- The test validator may not have started properly
- Check if port 8899 is available
- Try running tests again

## Resources

### Solana Documentation
- [Solana Documentation](https://docs.solana.com/)
- [Solana Program Library](https://spl.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Anchor Framework](https://www.anchor-lang.com/) - Higher-level framework

### Learning Resources
- [Solana Development Course](https://www.soldev.app/)
- [Solana Program Examples](https://github.com/solana-labs/solana-program-library)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

WTFPL (Do What The Fuck You Want To Public License)

This is free and unencumbered software released into the public domain.

## Next Steps

1. **Customize this template** for your use case
2. **Implement your program logic** in `src/lib.rs`
3. **Add comprehensive tests** in `tests/`
4. **Update this README** with your program's specific documentation
5. **Deploy to devnet** for testing
6. **Audit and deploy to mainnet** when ready

---

**Need help?** Check the [Solana Stack Exchange](https://solana.stackexchange.com/) or [Solana Discord](https://discord.gg/solana).
