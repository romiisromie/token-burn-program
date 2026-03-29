describe("burn_tokens", () => {
  it("should mint tokens and then burn some of them", async () => {
    const creator = Keypair.generate();
    const user = Keypair.generate();
    await airdrop(provider.wallet.publicKey);

    const mint = await createMint(creator, 9);

    // Создаём токен аккаунт для пользователя
    await program.methods
      .createTokenAccount()
      .accounts({
        payer: provider.wallet.publicKey,
        mint: mint.publicKey,
        owner: user.publicKey,
        tokenAccount: ata(mint.publicKey, user.publicKey),
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const totalMint = new BN(5_000_000_000);
    await program.methods
      .mintTokens(totalMint)
      .accounts({
        mint: mint.publicKey,
        to: ata(mint.publicKey, user.publicKey),
        mintAuthority: creator.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([creator])
      .rpc();

    // Проверяем баланс после mint
    let balance = await readBalance(mint.publicKey, user.publicKey);
    expect(balance).to.equal(BigInt(totalMint.toString()));

    const burnAmount = new BN(2_000_000_000);
    await program.methods
      .burnTokens(burnAmount)
      .accounts({
        mint: mint.publicKey,
        tokenAccount: ata(mint.publicKey, user.publicKey),
        authority: user.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    // Баланс после сжигания
    balance = await readBalance(mint.publicKey, user.publicKey);
    expect(balance).to.equal(BigInt(totalMint.sub(burnAmount).toString()));
  });
});
