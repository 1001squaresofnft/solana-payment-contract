// import * as anchor from "@coral-xyz/anchor";
// import { ASSOCIATED_TOKEN_PROGRAM_ID, getMint, TOKEN_PROGRAM_ID } from "@solana/spl-token";
// import * as fs from "fs";
// import idl from "../target/idl/mintedgem.json";
// import { Mintedgem } from "../target/types/mintedgem";
// import { randomInt } from "crypto";

// (async function main() {
//   const connection = new anchor.web3.Connection(
//     anchor.web3.clusterApiUrl("devnet"),
//     "confirmed"
//   );

//   const keypair = anchor.web3.Keypair.fromSecretKey(
//     Uint8Array.from(
//       JSON.parse(
//         // ================== NOTE: need to update this path point to your wallet ==================
//         fs.readFileSync("/Users/lainhathoang/.config/solana/id.json", "utf-8")
//       )
//     )
//   );
//   // ================== setup WALLET ==================
//   const wallet = new anchor.Wallet(keypair);
//   console.log("interact using wallet: ", wallet.publicKey.toBase58());
//   // ================== setup PROVIDER ==================
//   const provider = new anchor.AnchorProvider(connection, wallet, {});
//   anchor.setProvider(provider);

//   // ================== DECLARE PROGRAM ID & DONE token==================
//   const programId = new anchor.web3.PublicKey(
//     "GbuZ8rLAvahewUdLDxysgcndv3PP6N9gLZTSWtSQCZdv" // 9
//   );

//   const doneTokenMint = await getMint(
//     connection,
//     new anchor.web3.PublicKey("B7dAybb6wM33GL5d2kuHDnPPre3KTxMWSfd7GwZpr6XX")
//   );

//   // ================== CREATE PROGRAM ==================
//   const program = new anchor.Program(idl as Mintedgem);

//   // ================== GET ENTIRE THE PDAs ==================
//   const [master] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("master")],
//     programId
//   );

//   const [vaultSol] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("vault_sol")],
//     programId
//   );

//   const [tokenAccountOwner] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("token_account_owner")],
//     programId
//   );

//   const [vaultToken] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("vault_token"), doneTokenMint.address.toBuffer()],
//     programId
//   );

//   const [senderTokenAccount] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       wallet.publicKey.toBuffer(),
//       TOKEN_PROGRAM_ID.toBuffer(),
//       doneTokenMint.address.toBuffer(),
//     ],
//     ASSOCIATED_TOKEN_PROGRAM_ID
//   );

//   // ================== create INSTRUCTIONs ==================
//   // ====== 1. hello
//   // const helloIx = await program.methods.hello().instruction();
//   // ====== 2. init MASTER
//   const percent = 9900; // 100_00 ~ 100%, 10_00 ~ 10%
//   const initMasterIx = await program.methods
//     .initialize(percent, percent)
//     .accountsStrict({
//       master,
//       signer: wallet.publicKey,
//       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//       systemProgram: anchor.web3.SystemProgram.programId,
//     })
//     .instruction();
//   // ===== 3. init vault SOL
//   const initVaultSolIx = await program.methods
//     .initVaultSol()
//     .accountsStrict({
//       master,
//       vaultSol,
//       signer: wallet.publicKey,
//       systemProgram: anchor.web3.SystemProgram.programId,
//       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//     })
//     .instruction();
//   // ====== 4. init vault DONE Token
//   const initVaultDoneTokenIx = await program.methods
//     .initVaultDoneToken()
//     .accountsStrict({
//       master,
//       mintOfTokenBeingSent: doneTokenMint.address,
//       tokenAccountOwnerPda: tokenAccountOwner,
//       vaultToken,
//       signer: wallet.publicKey,
//       systemProgram: anchor.web3.SystemProgram.programId,
//       tokenProgram: TOKEN_PROGRAM_ID,
//       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//     })
//     .instruction();
//   // ====== 5. deposit SOL
//   const amountSol = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);
//   console.log("amountSol: ", amountSol.toNumber());
//   const depositSolIx = await program.methods
//     .depositSol(amountSol)
//     .accountsStrict({
//       master,
//       vaultSol,
//       signer: wallet.publicKey,
//       systemProgram: anchor.web3.SystemProgram.programId,
//     })
//     .instruction();
//   // ====== 6. deposit DONE Token
//   const amountDoneToken = new anchor.BN(
//     1_000_000 * 10 ** doneTokenMint.decimals
//   );
//   const depositDoneTokenIx = await program.methods
//     .depositDoneToken(amountDoneToken)
//     .accountsStrict({
//       master,
//       mintOfTokenBeingSent: doneTokenMint.address,
//       tokenAccountOwnerPda: tokenAccountOwner,
//       vaultToken,
//       senderTokenAccount,
//       signer: wallet.publicKey,
//       systemProgram: anchor.web3.SystemProgram.programId,
//       tokenProgram: TOKEN_PROGRAM_ID,
//       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//     })
//     .instruction();
//   // ===== 7. withdraw SOL
//   const amountSolWithdraw = new anchor.BN(0.01 * anchor.web3.LAMPORTS_PER_SOL);
//   const withdrawSolIx = await program.methods
//     .withdrawSol(amountSolWithdraw)
//     .accountsStrict({
//       master,
//       vaultSol,
//       signer: wallet.publicKey,
//       systemProgram: anchor.web3.SystemProgram.programId,
//     })
//     .instruction();
//   // ===== 8. withdraw DONE Token
//   const amountDoneTokenWithdraw = new anchor.BN(
//     500_000 * 10 ** doneTokenMint.decimals
//   );
//   const withdrawDoneTokenIx = await program.methods
//     .withdrawDoneToken(amountDoneTokenWithdraw)
//     .accountsStrict({
//       master,
//       mintOfTokenBeingSent: doneTokenMint.address,
//       tokenAccountOwnerPda: tokenAccountOwner,
//       vaultToken,
//       senderTokenAccount,
//       signer: wallet.publicKey,
//       tokenProgram: TOKEN_PROGRAM_ID,
//       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//     })
//     .instruction();
//   // ===== 9. Init sender Ata
//   const initSenderAta = await program.methods
//     .initSenderAta()
//     .accountsStrict({
//       senderTokenAccount,
//       signer: wallet.publicKey,
//       mintOfTokenBeingSent: doneTokenMint.address,
//       systemProgram: anchor.web3.SystemProgram.programId,
//       tokenProgram: TOKEN_PROGRAM_ID,
//       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
//     })
//     .instruction();
//   // ===== 9.0 Init tx sol volume
//   const [transactionSolVolume] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("transaction_sol_volume"), wallet.publicKey.toBuffer()],
//     programId
//   );
//   const initTxSolVolumeIx = await program.methods
//     .initTxSolVolume()
//     .accountsStrict({
//       transactionSolVolume,
//       signer: wallet.publicKey,
//       systemProgram: anchor.web3.SystemProgram.programId,
//     })
//     .instruction();
//   // ===== 9.1 Create payment by Sol
//   const randomNumber = randomInt(0, 999999);
//   console.log("randomNumber: ", randomNumber);
//   const itemId = new anchor.BN(randomNumber);
//   const amountSolCreatePayment = new anchor.BN(
//     1 * anchor.web3.LAMPORTS_PER_SOL
//   );
//   const [itemPayment] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("item_payment"), itemId.toArrayLike(Buffer, "le", 8)],
//     programId
//   );
//   const createPaymentBySolIx = await program.methods
//     .createPayment(itemId, amountSolCreatePayment)
//     .accountsStrict({
//       master,
//       itemPayment,
//       transactionSolVolume,
//       vaultSol,
//       mintOfTokenBeingSent: doneTokenMint.address,
//       tokenAccountOwnerPda: tokenAccountOwner,
//       vaultToken,
//       senderTokenAccount,
//       signer: wallet.publicKey,
//       systemProgram: anchor.web3.SystemProgram.programId,
//       tokenProgram: TOKEN_PROGRAM_ID,
//       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//     })
//     .instruction();
//   // ===== 10.0 Init tx sol volume
//   const [transactionDoneTokenVolume] =
//     anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("transaction_done_token_volume"),
//         wallet.publicKey.toBuffer(),
//       ],
//       programId
//     );
//   const initTxDoneTokenvolumeIx = await program.methods
//     .initTxDoneTokenVolume()
//     .accountsStrict({
//       transactionDoneTokenVolume,
//       signer: wallet.publicKey,
//       systemProgram: anchor.web3.SystemProgram.programId,
//     })
//     .instruction();
//   // ===== 10.1 Create payment by Done Token
//   const amountDoneTokenCreatePayment = new anchor.BN(
//     10 * 10 ** doneTokenMint.decimals
//   );
//   const [itemPaymentByDone] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("item_payment_by_done"), itemId.toArrayLike(Buffer, "le", 8)],
//     programId
//   );
//   const createPaymentByDoneTokenIx = await program.methods
//     .createPaymentByDone(itemId, amountDoneTokenCreatePayment)
//     .accountsStrict({
//       master,
//       itemPayment: itemPaymentByDone,
//       transactionDoneTokenVolume,
//       mintOfTokenBeingSent: doneTokenMint.address,
//       tokenAccountOwnerPda: tokenAccountOwner,
//       vaultToken,
//       senderTokenAccount,
//       signer: wallet.publicKey,
//       systemProgram: anchor.web3.SystemProgram.programId,
//       tokenProgram: TOKEN_PROGRAM_ID,
//       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//     })
//     .instruction();
//   // ===== 11 Set percent
//   // 11.2 Pay with Sol
//   const newPercent = 9900;
//   const setPercentSolIx = await program.methods
//     .setPercentPayWSol(newPercent)
//     .accountsStrict({
//       master,
//       signer: wallet.publicKey,
//       systemProgram: anchor.web3.SystemProgram.programId,
//     })
//     .instruction();
//   // 11.2 Pay with DONE token
//   const setPercentDoneTokenIx = await program.methods
//     .setPercentPayWDoneToken(newPercent)
//     .accountsStrict({
//       master,
//       signer: wallet.publicKey,
//       systemProgram: anchor.web3.SystemProgram.programId,
//     })
//     .instruction();

//   // ================== SEND TX ==================
//   try {
//     const tx = new anchor.web3.Transaction()
//       .add(
//       // helloIx,

//       // ===== INIT MASTER & VAULTS
//       // initMasterIx,
//       // initVaultSolIx,
//       // initVaultDoneTokenIx,

//       // ===== DEPOSIT
//       // depositSolIx,
//       // depositDoneTokenIx,

//       // ===== WITHDRAW
//       // withdrawSolIx,
//       // withdrawDoneTokenIx,

//       // ===== CREATE PAYMENT 
//       // initSenderAta,
//       // initTxSolVolumeIx,
//       // createPaymentBySolIx

//       // ===== CREATE PAYMENT BY DONE
//       // initTxDoneTokenvolumeIx,
//       // createPaymentByDoneTokenIx,

//       // ===== SET PERCENT
//       // setPercentSolIx,
//       // setPercentDoneTokenIx,
//     );
//     const txLog = await anchor.web3.sendAndConfirmTransaction(connection, tx, [
//       wallet.payer,
//     ]);
//     console.log("=> tx: ", txLog);
//   } catch (error) {
//     console.log("=> error: ", error);
//   }

//   // ===== GET MASTER DATA
//   // const masterPda = await program.account.master.fetch(master);
//   // console.log("masterPda: ", masterPda);
// })();
