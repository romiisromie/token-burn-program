# token-burn-program

Anchor program that demonstrates common SPL Token flows: creating a mint, associated token accounts, minting, burning, and transferring.

## Prerequisites

- [Rust](https://rustup.rs/) (see `rust-toolchain.toml` for the pinned toolchain)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/docs/installation) (0.30.x, matching `Anchor.toml` / `Cargo.toml`)
- Node.js 18+ and [Yarn](https://yarnpkg.com/)

## Quick start

```bash
yarn install
anchor build
anchor test
```

Generated TypeScript types for clients live under `target/types/` after `anchor build`.

Commit a `yarn.lock` (or `package-lock.json`) once you run `yarn install` locally so CI installs the same dependency tree.

## Continuous integration

GitHub Actions (`.github/workflows/ci.yml`) runs `cargo fmt --check`, `anchor build`, and `anchor test` on pushes and pull requests to `main` / `master`.

## Program instructions

| Instruction             | Role |
|-------------------------|------|
| `create_token`          | Initializes a new SPL mint (decimals + mint authority). |
| `create_token_account`  | Creates an ATA for a given owner and mint. |
| `mint_tokens`           | Mints tokens into a destination token account (mint authority signs). |
| `burn_tokens`           | Burns tokens from a token account you own (reduces mint supply). |
| `transfer_tokens`       | Transfers between two token accounts of the same mint. |

### `burn_tokens`

This instruction does not reimplement SPL burn logic in your program. It performs a **cross-program invocation (CPI)** into the SPL Token program’s `Burn` instruction: your program passes the mint, the token account to debit (`token_account`), and the signer (`authority`), and SPL Token updates balances and total supply.

**Accounts:**

| Account          | Description |
|------------------|-------------|
| `mint`           | Mint for the tokens being burned (must match the token account’s mint). |
| `token_account`  | SPL token account whose balance decreases. |
| `authority`      | Must be the **owner** of `token_account` (same pattern as wallet-initiated burns). |
| `token_program`  | SPL Token program (`Tokenkeg...`). |

**Rust handler:** `burn_tokens` in `programs/astana-token/src/lib.rs` builds `anchor_spl::token::Burn` CPI accounts and calls `token::burn`.

## Security

- Never commit wallet keypairs or mainnet program deploy keys. Use CI secrets only where appropriate.
- Review account constraints before deploying to devnet/mainnet; this repo is intended as a learning reference.

## License

MIT — see `package.json`.
