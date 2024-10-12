import * as anchor from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  NATIVE_MINT,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
  createSyncNativeInstruction,
  createCloseAccountInstruction,
} from "@solana/spl-token";
import idl from "../target/idl/mintedgem.json";
import { Mintedgem } from "../target/types/mintedgem";
import { randomInt } from "crypto";
import {
  getItemPayment,
  getMaster,
  getSenderTokenAccount,
  getTransactionSolVolume,
  getVaultSol,
} from "./utils/pda";
import {
  connection,
  DEVNET_CP_SWAP_PROGRAM_ID,
  doneTokenPubkey,
  getDoneTokenMint,
  PROGRAM_ID,
  wallet,
  getAuthAddress,
  getPoolAddress,
  getAmmConfigAddress,
  getOrcleAccountAddress,
  getPoolVaultAddress,
} from "./utils/config";

(async function main() {
  // ================== setup WALLET ==================
  //
  console.log("interact using wallet: ", wallet.publicKey.toBase58());
  //
  // ================== setup PROVIDER ==================
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  anchor.setProvider(provider);
  //
  // ================== DECLARE PROGRAM ID & DONE token==================
  const programId = new anchor.web3.PublicKey(PROGRAM_ID);
  const doneTokenMint = await getDoneTokenMint();
  //
  // ================== CREATE PROGRAM ==================
  const program = new anchor.Program(idl as Mintedgem);
  //
  //
  const randomNumber = randomInt(0, 999999);
  console.log("randomNumber: ", randomNumber);
  const itemId = new anchor.BN(randomNumber);
  const amountSolCreatePayment = new anchor.BN(
    0.222 * anchor.web3.LAMPORTS_PER_SOL
  );
  //
  // ================== GET ENTIRE THE PDAs ==================
  const [master] = getMaster();
  const [vaultSol] = getVaultSol();
  const [senderTokenAccount] = getSenderTokenAccount(doneTokenMint);
  const [transactionSolVolume] = getTransactionSolVolume();
  const [itemPayment] = getItemPayment(itemId);
  // CPI
  const [authority] = await getAuthAddress(DEVNET_CP_SWAP_PROGRAM_ID);
  const [ammConfig] = await getAmmConfigAddress(0, DEVNET_CP_SWAP_PROGRAM_ID);
  const [poolState] = await getPoolAddress(
    ammConfig,
    NATIVE_MINT,
    doneTokenPubkey,
    DEVNET_CP_SWAP_PROGRAM_ID
  );
  const [inputVault] = await getPoolVaultAddress(
    poolState,
    NATIVE_MINT,
    DEVNET_CP_SWAP_PROGRAM_ID
  );
  const [outputVault] = await getPoolVaultAddress(
    poolState,
    doneTokenPubkey,
    DEVNET_CP_SWAP_PROGRAM_ID
  );
  const [observationState] = await getOrcleAccountAddress(
    poolState,
    DEVNET_CP_SWAP_PROGRAM_ID
  );
  const wsolAta = getAssociatedTokenAddressSync(NATIVE_MINT, wallet.publicKey);
  const doneTokenAta = getAssociatedTokenAddressSync(
    doneTokenPubkey,
    wallet.publicKey
  );
  //
  // ================== create INSTRUCTIONs ==================
  // Make a payment with SOL
  // ========== 1. Init sender Ata
  const initSenderAta = await program.methods
    .initSenderAta()
    .accountsStrict({
      senderTokenAccount,
      signer: wallet.publicKey,
      mintOfTokenBeingSent: doneTokenMint.address,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .instruction();
  //
  // ========== 2. Init tx SOL volume
  const initTxSolVolumeIx = await program.methods
    .initTxSolVolume()
    .accountsStrict({
      transactionSolVolume,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();
  //
  // ========== 3 Create payment by Sol
  const createPaymentBySolIx = await program.methods
    .createPayment(itemId, amountSolCreatePayment)
    .accountsStrict({
      master,
      itemPayment,
      transactionSolVolume,
      vaultSol,
      wsolMint: NATIVE_MINT,
      doneTokenMint: doneTokenPubkey,
      poolState,
      senderWsolAta: wsolAta,
      senderDoneTokenAta: doneTokenAta,
      authority,
      ammConfig,
      inputVault,
      outputVault,
      observationState,
      cpSwapProgram: DEVNET_CP_SWAP_PROGRAM_ID,

      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .instruction();

  // ================== SEND TX ==================
  try {
    const tx = new anchor.web3.Transaction().add(
      // ===== CREATE PAYMENT
      initSenderAta,
      initTxSolVolumeIx,
      //
      // before create payment
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        wsolAta,
        wallet.publicKey,
        NATIVE_MINT
      ),
      anchor.web3.SystemProgram.transfer({
        fromPubkey: wallet.publicKey,
        toPubkey: wsolAta,
        lamports: amountSolCreatePayment.mul(new anchor.BN(990)).div(new anchor.BN(1000)).toNumber(),
      }),
      createSyncNativeInstruction(wsolAta),
      //
      createPaymentBySolIx,
      //
      // after create payment
      createCloseAccountInstruction(wsolAta, wallet.publicKey, wallet.publicKey)
      // 
    );
    const txLog = await anchor.web3.sendAndConfirmTransaction(
      connection,
      tx,
      [wallet.payer],
      {
        commitment: "confirmed",
        skipPreflight: false,
      }
    );
    console.log("=> tx: ", txLog);
  } catch (error) {
    console.log("=> error: ", error);
  }


  // ====== GET PAYMENT ITEM DATA
  //   const itemPaymentPda = await program.account.itemPayment.fetch(itemPayment);
  //   console.log("itemPaymentPda: ", itemPaymentPda);
  //   console.log("creator: ", itemPaymentPda.creator.toBase58());
  //   console.log("amount: ", itemPaymentPda.amount.toString());
  //   console.log("amountDone: ", itemPaymentPda.amountDone.toString());
})();
