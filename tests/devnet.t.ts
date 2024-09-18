import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import * as fs from "fs";
import idl from "../target/idl/mintedgem.json";
import { Mintedgem } from "../target/types/mintedgem";
import { randomInt } from "crypto";

(async function main() {
  const connection = new anchor.web3.Connection(
    "https://api.devnet.solana.com"
  );

  const keypair = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(
        // ================== NOTE: need to update this path point to your wallet ==================
        fs.readFileSync("/Users/lainhathoang/.config/solana/id.json", "utf-8")
      )
    )
  );
  // ================== setup WALLET ==================
  const wallet = new anchor.Wallet(keypair);
  console.log("interact using wallet: ", wallet.publicKey.toBase58());
  // ================== setup PROVIDER ==================
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  anchor.setProvider(provider);

  // ================== DECLARE PROGRAM ID & DONE token==================
  const programId = new anchor.web3.PublicKey(
    "7vFW6zPUtfgctuwyDgij12NkC9Cgi2R6YExqgXU5tLCp" // 9
  );

  const doneTokenMint = await spl.getMint(
    connection,
    new anchor.web3.PublicKey("B7dAybb6wM33GL5d2kuHDnPPre3KTxMWSfd7GwZpr6XX") // 9
  );

  // ================== CREATE PROGRAM ==================
  const program = new anchor.Program(idl as Mintedgem, programId);

  // ================== GET ENTIRE THE PDAs ==================
  const [master] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("master")],
    programId
  );

  const [vaultSol] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_sol")],
    programId
  );

  const [tokenAccountOwner] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("token_account_owner")],
    programId
  );

  const [vaultToken] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_token"), doneTokenMint.address.toBuffer()],
    programId
  );

  const [senderTokenAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      wallet.publicKey.toBuffer(),
      spl.TOKEN_PROGRAM_ID.toBuffer(),
      doneTokenMint.address.toBuffer(),
    ],
    spl.ASSOCIATED_TOKEN_PROGRAM_ID
  );

  spl.getOrCreateAssociatedTokenAccount;

  // ================== create INSTRUCTIONs ==================
  // ====== 1. hello
  const helloIx = await program.methods.hello().instruction();
  // ====== 2. init MASTER
  const percent = new anchor.BN(50);
  const initMasterIx = await program.methods
    .initialize(percent)
    .accounts({
      master,
      signer: wallet.publicKey,
    })
    .instruction();
  // ===== 3. init vault SOL
  const initVaultSolIx = await program.methods
    .initVaultSol()
    .accounts({
      master,
      vaultSol,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .instruction();
  // ====== 4. init vault DONE Token
  const initVaultDoneTokenIx = await program.methods
    .initVaultDoneToken()
    .accounts({
      master,
      mintOfTokenBeingSent: doneTokenMint.address,
      tokenAccountOwnerPda: tokenAccountOwner,
      vaultToken,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .instruction();
  // ====== 5. deposit SOL
  const amountSol = new anchor.BN(0.5 * anchor.web3.LAMPORTS_PER_SOL);
  console.log("amountSol: ", amountSol.toNumber());
  const depositSolIx = await program.methods
    .depositSol(amountSol)
    .accounts({
      master,
      vaultSol,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();
  // ====== 6. deposit DONE Token
  const amountDoneToken = new anchor.BN(
    1_000_000 * 10 ** doneTokenMint.decimals
  );
  const depositDoneTokenIx = await program.methods
    .depositDoneToken(amountDoneToken)
    .accounts({
      master,
      mintOfTokenBeingSent: doneTokenMint.address,
      tokenAccountOwnerPda: tokenAccountOwner,
      vaultToken,
      senderTokenAccount,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .instruction();
  // ===== 7. withdraw SOL
  const amountSolWithdraw = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);
  const withdrawSolIx = await program.methods
    .withdrawSol(amountSolWithdraw)
    .accounts({
      master,
      vaultSol,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();
  // ===== 8. withdraw DONE Token
  const amountDoneTokenWithdraw = new anchor.BN(
    1_000_000 * 10 ** doneTokenMint.decimals
  );
  const withdrawDoneTokenIx = await program.methods
    .withdrawDoneToken(amountDoneTokenWithdraw)
    .accounts({
      master,
      mintOfTokenBeingSent: doneTokenMint.address,
      tokenAccountOwnerPda: tokenAccountOwner,
      vaultToken,
      senderTokenAccount,
      signer: wallet.publicKey,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .instruction();
  // ===== 9. Init sender Ata
  const initSenderAta = await program.methods
    .initSenderAta()
    .accounts({
      senderTokenAccount,
      signer: wallet.publicKey,
      mintOfTokenBeingSent: doneTokenMint.address,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .instruction();
  // ===== 9.0 Init tx sol volume
  const [transactionSolVolume] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("transaction_sol_volume"), wallet.publicKey.toBuffer()],
    programId
  );
  const initTxSolVolumeIx = await program.methods
    .initTxSolVolume()
    .accounts({
      transactionSolVolume,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();
  // ===== 9.1 Create payment by Sol
  const randomNumber = randomInt(0, 999999);
  console.log("randomNumber: ", randomNumber);
  const itemId = new anchor.BN(randomNumber);
  const amountSolCreatePayment = new anchor.BN(
    5 * anchor.web3.LAMPORTS_PER_SOL
  );
  const [itemPayment] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("item_payment"), itemId.toArrayLike(Buffer, "le", 8)],
    programId
  );
  const createPaymentBySolIx = await program.methods
    .createPayment(itemId, amountSolCreatePayment)
    .accounts({
      master,
      itemPayment,
      transactionSolVolume,
      vaultSol,
      mintOfTokenBeingSent: doneTokenMint.address,
      tokenAccountOwnerPda: tokenAccountOwner,
      vaultToken,
      senderTokenAccount,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .instruction();
  // ===== 10.0 Init tx sol volume
  const [transactionDoneTokenVolume] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("transaction_done_token_volume"),
        wallet.publicKey.toBuffer(),
      ],
      programId
    );
  const initTxDoneTokenvolumeIx = await program.methods
    .initTxDoneTokenVolume()
    .accounts({
      transactionDoneTokenVolume,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();
  // ===== 10.1 Create payment by Done Token
  const amountDoneTokenCreatePayment = new anchor.BN(
    10 * 10 ** doneTokenMint.decimals
  );
  const [itemPaymentByDone] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("item_payment_by_done"), itemId.toArrayLike(Buffer, "le", 8)],
    programId
  );
  const createPaymentByDoneTokenIx = await program.methods
    .createPaymentByDone(itemId, amountDoneTokenCreatePayment)
    .accounts({
      itemPayment: itemPaymentByDone,
      transactionDoneTokenVolume,
      mintOfTokenBeingSent: doneTokenMint.address,
      tokenAccountOwnerPda: tokenAccountOwner,
      vaultToken,
      senderTokenAccount,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .instruction();

  // ================== SEND TX ==================
  try {
    const tx = new anchor.web3.Transaction().add(
      // helloIx,
      initMasterIx,
      initVaultSolIx,
      initVaultDoneTokenIx,
      // depositSolIx,
      depositDoneTokenIx,
      // initSenderAta,
      // initTxSolVolumeIx,
      // createPaymentBySolIx
      // initTxDoneTokenvolumeIx,
      // createPaymentByDoneTokenIx
    );
    const txLog = await anchor.web3.sendAndConfirmTransaction(connection, tx, [
      wallet.payer,
    ]);
    console.log("=> tx: ", txLog);
  } catch (error) {
    console.log("=> error: ", error);
  }
})();
