import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LotteryPlatform } from "../target/types/lottery_platform";
import {
  Account,
  createAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("lottery_platform", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.LotteryPlatform as Program<LotteryPlatform>;
  const adminOwner = anchor.web3.Keypair.generate();
  const lotteryGameOwner = anchor.web3.Keypair.generate();
  const depositAccount = anchor.web3.Keypair.generate();
  /* const usdcMint = new anchor.web3.PublicKey(
    "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
  ); // USDC devnet */

  const payer = wallet.payer;
  const associateTokenProgram = new anchor.web3.PublicKey(
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
  );
  const mintToken = anchor.web3.Keypair.generate(); // dummy usdc token created for test purposes
  const tokenAccount = anchor.utils.token.associatedAddress({
    mint: mintToken.publicKey,
    owner: payer.publicKey,
  });

  let firstParticipantOwner = anchor.web3.Keypair.generate();
  let firstParticipantOwnerATA = anchor.web3.Keypair.generate();

  let secondParticipantOwner = anchor.web3.Keypair.generate();
  let secondParticipantOwnerATA = anchor.web3.Keypair.generate();

  let treasuryVaultATA: Account;

  // pdaAuth
  let [pdaAuth, adminPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("auth"),
      depositAccount.publicKey.toBuffer(),
    ],
    program.programId
  );
  let [treasuryVault, adminTreasuryBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("treasury-vault"), pdaAuth.toBuffer()],
      program.programId
    );

  let [lotteryGameConfigs] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("lottery-game-configs")],
    program.programId
  );

  let [lotteryGame] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("lottery-game"),
      lotteryGameOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [firstParticipant] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("participant"),
      firstParticipantOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [secondParticipant] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("participant"),
      secondParticipantOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  // admin owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      adminOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // first participant owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      firstParticipantOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // second participant owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      secondParticipantOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // lottery game owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      lotteryGameOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  it("Is initialized!", async () => {
    try {
      const tx = await program.methods
        .init()
        .accounts({
          owner: adminOwner.publicKey,
          lotteryGameConfigs: lotteryGameConfigs,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([adminOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.lotteryGameConfigs.fetch(
        lotteryGameConfigs
      );
      console.log("lotteryGameConfigs: ", result);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register lottery game!", async () => {
    try {
      let operator = {
        operator: "Company Lotto Ke",
      };

      let initParams = {
        operator: operator,
        country: "KE",
        lotteryGameName: "Lotto Ke 100",
        lotteryWinningPercentage: 90, // 90 %
        unitCostOfLotteryTicket: 1, // unit cost of lottery ticket
        decimals: 9, // token mint in smallest unit i.e 9 decimals
        valueDate: "28-09-2024",
      };

      const tx = await program.methods
        .registerLotteryGame(initParams)
        .accounts({
          owner: lotteryGameOwner.publicKey,
          lotteryGameConfigs: lotteryGameConfigs,
          lotteryGame: lotteryGame,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([lotteryGameOwner, depositAccount])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.lotteryGame.fetch(lotteryGame);
      let result2 = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      let result3 = await program.account.lotteryGameConfigs.fetch(
        lotteryGameConfigs
      );
      console.log("lottery game: ", result);
      console.log("deposit account: ", result2);
      console.log("lottery game configs: ", result3);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is create token!", async () => {
    console.log("mint token: ", mintToken.publicKey.toBase58());
    console.log("token account: ", tokenAccount.toBase58());

    try {
      let initParams = {
        amount: new anchor.BN(200),
      };

      const tx = await program.methods
        .createToken(initParams)
        .accounts({
          owner: payer.publicKey,
          lotteryGame: lotteryGame,
          mintToken: mintToken.publicKey,
          tokenAccount: tokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintToken])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is token transfer - first participant", async () => {
    console.log(
      "participant owner token account: ",
      firstParticipantOwnerATA.publicKey.toBase58()
    );

    try {
      await createAccount(
        provider.connection,
        firstParticipantOwner,
        mintToken.publicKey,
        firstParticipantOwner.publicKey,
        firstParticipantOwnerATA
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let initParams = {
        amount: new anchor.BN(70),
      };
      const tx = await program.methods
        .transferToken(initParams)
        .accounts({
          owner: payer.publicKey,
          lotteryGame: lotteryGame,
          mintToken: mintToken.publicKey,
          fromAccount: tokenAccount,
          toAccount: firstParticipantOwnerATA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintToken])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is token transfer - second participant", async () => {
    console.log(
      "participant owner token account: ",
      secondParticipantOwnerATA.publicKey.toBase58()
    );

    try {
      await createAccount(
        provider.connection,
        secondParticipantOwner,
        mintToken.publicKey,
        secondParticipantOwner.publicKey,
        secondParticipantOwnerATA
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let initParams = {
        amount: new anchor.BN(100),
      };
      const tx = await program.methods
        .transferToken(initParams)
        .accounts({
          owner: payer.publicKey,
          lotteryGame: lotteryGame,
          mintToken: mintToken.publicKey,
          fromAccount: tokenAccount,
          toAccount: secondParticipantOwnerATA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintToken])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register first participant!", async () => {
    try {
      let initParams = {
        fullNames: "paul john",
        country: "KE",
      };

      const tx = await program.methods
        .registerParticipant(initParams)
        .accounts({
          owner: firstParticipantOwner.publicKey,
          participant: firstParticipant,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstParticipantOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.participant.fetch(firstParticipant);
      console.log("participant: ", result);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register second participant!", async () => {
    try {
      let initParams = {
        fullNames: "philip samuel",
        country: "KE",
      };

      const tx = await program.methods
        .registerParticipant(initParams)
        .accounts({
          owner: secondParticipantOwner.publicKey,
          participant: secondParticipant,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondParticipantOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.participant.fetch(secondParticipant);
      console.log("participant: ", result);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is buy lottery ticket - first participant!", async () => {
    try {
      treasuryVaultATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mintToken.publicKey,
        treasuryVault,
        true
      );
      console.log(
        "treasuryVaultATA address: " + treasuryVaultATA.address.toBase58()
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let initParams = {
        // 1 amount of token to transfer (in smallest unit i.e 9 decimals)
        amount: new anchor.BN(1),
      };

      const tx = await program.methods
        .buyLotteryTicket(initParams)
        .accounts({
          owner: firstParticipantOwner.publicKey,
          lotteryGame: lotteryGame,
          participant: firstParticipant,
          senderTokens: firstParticipantOwnerATA.publicKey,
          recipientTokens: treasuryVaultATA.address,
          mintToken: mintToken.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstParticipantOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.participant.fetch(firstParticipant);
      console.log("participant: ", result);

      let result2 = await program.account.lotteryGame.fetch(lotteryGame);
      console.log("lottery game: ", result2);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is buy lottery ticket - second participant!", async () => {
    try {
      treasuryVaultATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mintToken.publicKey,
        treasuryVault,
        true
      );
      console.log(
        "treasuryVaultATA address: " + treasuryVaultATA.address.toBase58()
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let initParams = {
        // 1 amount of token to transfer (in smallest unit i.e 9 decimals)
        amount: new anchor.BN(1),
      };

      const tx = await program.methods
        .buyLotteryTicket(initParams)
        .accounts({
          owner: secondParticipantOwner.publicKey,
          lotteryGame: lotteryGame,
          participant: secondParticipant,
          senderTokens: secondParticipantOwnerATA.publicKey,
          recipientTokens: treasuryVaultATA.address,
          mintToken: mintToken.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondParticipantOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.participant.fetch(secondParticipant);
      console.log("participant: ", result);

      let result2 = await program.account.lotteryGame.fetch(lotteryGame);
      console.log("lottery game: ", result2);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is get lottery game winner!", async () => {
    try {
      let operator = {
        operator: "Company Lotto Ke",
      };

      let initParams = {
        operator: operator,
        country: "KE",
        lotteryGameName: "Lotto Ke 100",
        /*
        lotteryWinningPercentage: 90, // 90 %
        unitCostOfLotteryTicket: 1, // unit cost of lottery ticket
        decimals: 9, // token mint in smallest unit i.e 9 decimals
        valueDate: "28-09-2024",
        */
      };

      const tx = await program.methods
        .getLotteryGameWinner(initParams)
        .accounts({
          owner: lotteryGameOwner.publicKey,
          lotteryGameConfigs: lotteryGameConfigs,
          lotteryGame: lotteryGame,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([lotteryGameOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.lotteryGame.fetch(lotteryGame);
      let result2 = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      let result3 = await program.account.lotteryGameConfigs.fetch(
        lotteryGameConfigs
      );
      console.log("lottery game: ", result);
      console.log("deposit account: ", result2);
      console.log("lottery game configs: ", result3);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is withdraw lottery game winnings - first participant!", async () => {
    try {
      let initParams = {
        // 2 amount of token to withdraw (in smallest unit i.e 9 decimals)
        amount: new anchor.BN(2),
      };
      const tx = await program.methods
        .withdrawLotteryGameWinnings(initParams)
        .accounts({
          owner: firstParticipantOwner.publicKey,
          lotteryGame: lotteryGame,
          participant: firstParticipant,
          senderTokens: treasuryVaultATA.address,
          recipientTokens: firstParticipantOwnerATA.publicKey,
          mintToken: mintToken.publicKey,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstParticipantOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      console.log("deposit account: ", result);

      let result2 = await program.account.participant.fetch(firstParticipant);
      console.log("participant: ", result2);

      let result3 = await program.account.lotteryGame.fetch(lotteryGame);
      console.log("lottery game: ", result3);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is withdraw lottery game winnings - second participant!", async () => {
    try {
      let initParams = {
        // 2 amount of token to withdraw (in smallest unit i.e 9 decimals)
        amount: new anchor.BN(2),
      };
      const tx = await program.methods
        .withdrawLotteryGameWinnings(initParams)
        .accounts({
          owner: secondParticipantOwner.publicKey,
          lotteryGame: lotteryGame,
          participant: secondParticipant,
          senderTokens: treasuryVaultATA.address,
          recipientTokens: secondParticipantOwnerATA.publicKey,
          mintToken: mintToken.publicKey,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondParticipantOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      console.log("deposit account: ", result);

      let result2 = await program.account.participant.fetch(secondParticipant);
      console.log("participant: ", result2);

      let result3 = await program.account.lotteryGame.fetch(lotteryGame);
      console.log("lottery game: ", result3);
    } catch (error) {
      console.log(error);
    }
  });
});
