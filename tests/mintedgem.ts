// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { Mintedgem } from "../target/types/mintedgem";
// import {
//   initWalletWithSols,
//   mintTokenForWallet,
//   printAccountBalance,
//   printTokenBalanceSpl,
// } from "./helper";
// import * as spl from "@solana/spl-token";

// describe("mintedgem", () => {
//   // Configure the client to use the local cluster.
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const connection = provider.connection;

//   const program = anchor.workspace.Mintedgem as Program<Mintedgem>;

//   it("Is initialized!", async () => {
//     const user = await initWalletWithSols(10, connection);
//     const user2 = await initWalletWithSols(10, connection);

//     const amountDoneMint = new anchor.BN(1000 * anchor.web3.LAMPORTS_PER_SOL);
//     const { tokenAccount, tokenMint } = await mintTokenForWallet(
//       user,
//       connection,
//       amountDoneMint.toNumber()
//     );

//     // get the PDAs
//     const [masterPda, masterBump] =
//       anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("master")],
//         program.programId
//       );

//     const [vaultSol, vaultSolBump] =
//       anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("vault_sol")],
//         program.programId
//       );

//     const [tokenAccountOwnerPda, tokenAccountOwnerBump] =
//       anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("token_account_owner")],
//         program.programId
//       );

//     const [vaultToken, vaultTokenBump] =
//       anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("vault_token"), tokenMint.toBuffer()],
//         program.programId
//       );

//     // ===== init ======
//     const fee = new anchor.BN(10);

//     await program.methods
//       .initialize(fee)
//       .accounts({
//         master: masterPda,
//         tokenAccountOwnerPda,
//         vaultSol,
//         vaultToken,
//         mintOfTokenBeingSent: tokenMint,
//         signer: user.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//         tokenProgram: spl.TOKEN_PROGRAM_ID,
//       })
//       .signers([user])
//       .rpc();

//     console.log("Initialized");

//     // ===== deposit ======
//     console.log("[BEFORE] token balance");
//     console.log("user: ");
//     await printTokenBalanceSpl(connection, tokenAccount.address);
//     console.log("vault");
//     await printTokenBalanceSpl(connection, vaultToken);

//     const amountDone = new anchor.BN(100 * anchor.web3.LAMPORTS_PER_SOL);

//     await program.methods
//       .transferIn(amountDone)
//       .accounts({
//         tokenAccountOwnerPda,
//         vaultToken,
//         senderTokenAccount: tokenAccount.address,
//         mintOfTokenBeingSent: tokenMint,
//         signer: user.publicKey,
//         tokenProgram: spl.TOKEN_PROGRAM_ID,
//         vaultSol,
//         master: masterPda,
//       })
//       .signers([user])
//       .rpc();

//     console.log("[AFTER] token balance");
//     console.log("user: ");
//     await printTokenBalanceSpl(connection, tokenAccount.address);
//     console.log("vault");
//     await printTokenBalanceSpl(connection, vaultToken);

//     // ===== transfer out
//     console.log("BEFORE deposit SOL - TRANSFER OUT DONE TOKEN");
//     await printAccountBalance(user.publicKey);
//     await printAccountBalance(vaultSol);

//     const amountSolIn = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
//     const amountDoneOut = new anchor.BN(3);
//     const solItemId = new anchor.BN(1);

//     const [itemPayment] = anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("item_payment"), solItemId.toArrayLike(Buffer, "le", 8)],
//       program.programId
//     );
//     const [transactionSolVolumePda] =
//       anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("transaction_sol_volume"), user.publicKey.toBuffer()],
//         program.programId
//       );

//     const tx = await program.methods
//       .createPayment(solItemId, amountSolIn)
//       .accounts({
//         tokenAccountOwnerPda,
//         vaultToken,
//         senderTokenAccount: tokenAccount.address,
//         mintOfTokenBeingSent: tokenMint,
//         signer: user.publicKey,
//         tokenProgram: spl.TOKEN_PROGRAM_ID,
//         vaultSol,
//         master: masterPda,
//         itemPayment,
//         transactionSolVolume: transactionSolVolumePda,
//       })
//       .signers([user])
//       .rpc();


//     console.log("[AFTER TRANSFER_OUT] token balance");
//     console.log("user: ");
//     await printTokenBalanceSpl(connection, tokenAccount.address);
//     console.log("vault");
//     await printTokenBalanceSpl(connection, vaultToken);
//     console.log("SOL");
//     await printAccountBalance(user.publicKey);
//     await printAccountBalance(vaultSol);

//     const txSolVol = await program.account.transctionSolVolume.fetch(
//       transactionSolVolumePda
//     );
//     console.log("tx sol amount: ", txSolVol.amount.toNumber());
//     console.log("tx sol creator: ", txSolVol.creator.toBase58());

//     // ===== deposit DONE token
//     const itemId1 = new anchor.BN(1);

//     const [itemPaymentByDone] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("item_payment_by_done"),
//         itemId1.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );

//     const [transactionDoneTokenVolumePda] =
//       anchor.web3.PublicKey.findProgramAddressSync(
//         [
//           Buffer.from("transaction_done_token_volume"),
//           user.publicKey.toBuffer(),
//         ],
//         program.programId
//       );
//     const amountDoneIn = new anchor.BN(10 * anchor.web3.LAMPORTS_PER_SOL);

//     // init
//     // await program.methods
//     //   .initDoneTokenVolCtx()
//     //   .accounts({
//     //     transactionDoneTokenVolume: transactionDoneTokenVolumePda,
//     //     signer: user.publicKey,
//     //   })
//     //   .signers([user])
//     //   .rpc();
//     //

//     //123   // ==========
//     await program.methods
//       .createPaymentByDone(itemId1, amountDoneIn)
//       .accounts({
//         tokenAccountOwnerPda,
//         vaultToken,
//         senderTokenAccount: tokenAccount.address,
//         mintOfTokenBeingSent: tokenMint,
//         signer: user.publicKey,
//         tokenProgram: spl.TOKEN_PROGRAM_ID,
//         itemPayment: itemPaymentByDone,
//         transactionDoneTokenVolume: transactionDoneTokenVolumePda,
//       })
//       .signers([user])
//       .rpc();

//     console.log("[AFTER DEPOSIT DONE_TOKEN] token balance");
//     console.log("user: ");
//     await printTokenBalanceSpl(connection, tokenAccount.address);
//     console.log("vault");
//     await printTokenBalanceSpl(connection, vaultToken);

//     const txDoneTokenVol =
//       await program.account.transactionDoneTokenVolume.fetch(
//         transactionDoneTokenVolumePda
//       );
//     console.log("tx DONE amount: ", txDoneTokenVol.amount.toNumber());
//     console.log("tx DONE creator: ", txDoneTokenVol.creator.toBase58());

//     // payment by done 2
//     const itemId2 = new anchor.BN(2);

//     const [itemPaymentByDone2] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("item_payment_by_done"),
//         itemId2.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );

//     await program.methods
//       .createPaymentByDone(itemId2, amountDoneIn)
//       .accounts({
//         tokenAccountOwnerPda,
//         vaultToken,
//         senderTokenAccount: tokenAccount.address,
//         mintOfTokenBeingSent: tokenMint,
//         signer: user.publicKey,
//         tokenProgram: spl.TOKEN_PROGRAM_ID,
//         itemPayment: itemPaymentByDone2,
//         transactionDoneTokenVolume: transactionDoneTokenVolumePda,
//       })
//       .signers([user])
//       .rpc();

//     const txDoneTokenVol2 =
//       await program.account.transactionDoneTokenVolume.fetch(
//         transactionDoneTokenVolumePda
//       );
//     console.log("tx DONE amount 2: ", txDoneTokenVol2.amount.toNumber());
//     console.log("tx DONE creator 2: ", txDoneTokenVol2.creator.toBase58());

//     // payment by done 3
//     const itemId3 = new anchor.BN(5);

//     const [itemPaymentByDone3] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("item_payment_by_done"),
//         itemId3.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );

//     await program.methods
//       .createPaymentByDone(itemId3, amountDoneIn)
//       .accounts({
//         tokenAccountOwnerPda,
//         vaultToken,
//         senderTokenAccount: tokenAccount.address,
//         mintOfTokenBeingSent: tokenMint,
//         signer: user.publicKey,
//         tokenProgram: spl.TOKEN_PROGRAM_ID,
//         itemPayment: itemPaymentByDone3,
//         transactionDoneTokenVolume: transactionDoneTokenVolumePda,
//       })
//       .signers([user])
//       .rpc();

//     const txDoneTokenVol3 =
//       await program.account.transactionDoneTokenVolume.fetch(
//         transactionDoneTokenVolumePda
//       );
//     console.log("tx DONE amount 3: ", txDoneTokenVol3.amount.toNumber());
//     console.log("tx DONE creator 3: ", txDoneTokenVol3.creator.toBase58());

//         // ==== withdraw SOL from vault
//         console.log("[ADMIN[] BEFORE WITHDRAW] SOL");
//         await printAccountBalance(user.publicKey);
//         await printAccountBalance(vaultSol);

//         const amountSolOut = new anchor.BN(0.5 * anchor.web3.LAMPORTS_PER_SOL);

//         await program.methods
//           .withdrawSol(amountSolOut)
//           .accounts({
//             master: masterPda,
//             signer: user.publicKey,
//             vaultSol,
//             systemProgram: anchor.web3.SystemProgram.programId,
//           })
//           .signers([user])
//           .rpc();

//         console.log("[AFTER WITHDRAW] SOL");
//         await printAccountBalance(user.publicKey);
//         await printAccountBalance(vaultSol);

//         // ====== withdraw DONE token
//         console.log("[ADMIN] BEFORE TRANSFER DONE TOKEN OUT");
//         console.log("user: ");
//         await printTokenBalanceSpl(connection, tokenAccount.address);
//         console.log("vault");
//         await printTokenBalanceSpl(connection, vaultToken);

//         const amountDoneOutAdmin = new anchor.BN(50 * anchor.web3.LAMPORTS_PER_SOL);

//         await program.methods
//           .withdrawDoneToken(amountDoneOutAdmin)
//           .accounts({
//             master: masterPda,
//             mintOfTokenBeingSent: tokenMint,
//             senderTokenAccount: tokenAccount.address,
//             tokenAccountOwnerPda,
//             signer: user.publicKey,
//             tokenProgram: spl.TOKEN_PROGRAM_ID,
//             vaultToken,
//           })
//           .signers([user])
//           .rpc();

//         console.log("[ADMIN] AFTER TRANSFER DONE TOKEN OUT");
//         console.log("user: ");
//         await printTokenBalanceSpl(connection, tokenAccount.address);
//         console.log("vault");
//         await printTokenBalanceSpl(connection, vaultToken);
//   });
// });
