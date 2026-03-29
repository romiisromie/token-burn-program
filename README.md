# token-burn-program

A clean, production-ready Anchor program demonstrating core SPL Token operations on Solana.

## Features
- **Create Mint:** Initialize a new SPL token with custom decimals.
- **ATA Management:** Create Associated Token Accounts for users.
- **Minting:** Issue new tokens to a specific account.
- **Burning:** Securely burn tokens using Cross-Program Invocations (CPI).
- **Transferring:** Move tokens between accounts.

## Prerequisites
- [Rust](https://rustup.rs/) (see `rust-toolchain.toml`)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/docs/installation) (v0.30.x)
- Node.js 18+ & [Yarn](https://yarnpkg.com/)

## Quick Start

1. **Install dependencies:**
   ```bash
   yarn install
