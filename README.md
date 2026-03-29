 token-burn-program

Solana Anchor program demonstrating common SPL Token flows: creating a mint, associated token accounts, minting, burning, and transferring tokens.

⸻

 Prerequisites
	•	Rust￼ (see rust-toolchain.toml for pinned toolchain)
	•	Solana CLI￼
	•	Anchor￼ (0.30.x, matches Anchor.toml)
	•	Node.js 18+ and Yarn￼

⸻

 Getting Started
	1.	Clone the repository:

git clone https://github.com/your-username/astana-token.git
cd astana-token

	2.	Install dependencies:

yarn install

	3.	Build the program:

anchor build

	4.	Run local tests:

anchor test

	5.	Generated TypeScript types live under target/types/ after anchor build.

⸻

 Project Structure

programs/astana-token/src/lib.rs  # main Rust program
tests/astana-token.ts             # Anchor TypeScript tests
target/types/                     # generated TypeScript types
Anchor.toml                       # Anchor config
Cargo.toml                        # Rust dependencies


⸻

 Program Instructions

Instruction	Description
create_token	Initializes a new SPL mint (decimals + mint authority).
create_token_account	Creates an associated token account (ATA) for a given owner and mint.
mint_tokens	Mints tokens to a destination token account (signed by mint authority).
burn_tokens	Burns tokens from a token account you own, reducing the mint supply.
transfer_tokens	Transfers tokens between two accounts of the same mint.


⸻

 Examples

Create a mint

const creator = Keypair.generate();
const mint = await program.methods
  .createToken(9) // decimals
  .accounts({
    mint: mintPubkey,
    payer: provider.wallet.publicKey,
    mintAuthority: creator.publicKey,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .signers([creator])
  .rpc();

Create an associated token account

await program.methods
  .createTokenAccount()
  .accounts({
    payer: provider.wallet.publicKey,
    mint: mintPubkey,
    owner: user.publicKey,
    tokenAccount: userTokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

Mint tokens

await program.methods
  .mintTokens(new BN(1_000_000))
  .accounts({
    mint: mintPubkey,
    to: userTokenAccount,
    mintAuthority: creator.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .signers([creator])
  .rpc();

Burn tokens

await program.methods
  .burnTokens(new BN(500_000))
  .accounts({
    mint: mintPubkey,
    tokenAccount: userTokenAccount,
    authority: user.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .signers([user])
  .rpc();

Transfer tokens

await program.methods
  .transferTokens(new BN(200_000))
  .accounts({
    from: userATAMint,
    to: userBTAMint,
    authority: user.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .signers([user])
  .rpc();


⸻

 Testing

Run tests locally:

anchor test

	•	Tests cover creating mints, ATAs, minting, burning, and transfers.
	•	Includes happy paths and failure cases (invalid authority, insufficient funds, mint mismatch).

⸻

 Security
	•	Never commit wallet keypairs or mainnet deploy keys.
	•	Verify account constraints before deploying to devnet/mainnet.
	•	This repository is for learning and reference purposes.

⸻

 License

MIT — see package.json.
