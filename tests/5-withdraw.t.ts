import * as anchor from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getMint,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import * as fs from "fs";
import idl from "../target/idl/mintedgem.json";
import { Mintedgem } from "../target/types/mintedgem";
import { randomInt } from "crypto";
import {
  getMaster,
  getSenderTokenAccount,
  getTokenAccountOwner,
  getVaultSol,
  getVaultToken,
} from "./utils/pda";
import { connection, getDoneTokenMint, wallet } from "./utils/config";

(async function main() {
  console.log("interact using wallet: ", wallet.publicKey.toBase58());
  // ================== setup PROVIDER ==================
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  anchor.setProvider(provider);
  //
  // ================== DECLARE PROGRAM ID & DONE token==================
  const doneTokenMint = await getDoneTokenMint();
  //
  // ================== CREATE PROGRAM ==================
  const program = new anchor.Program(idl as Mintedgem);
  //
  // ================== GET ENTIRE THE PDAs ==================
  const [master] = getMaster();
  const [vaultSol] = getVaultSol();
  const [tokenAccountOwner] = getTokenAccountOwner();
  const [vaultToken] = getVaultToken(doneTokenMint);
  const [senderTokenAccount] = getSenderTokenAccount(doneTokenMint);

  // ================== create INSTRUCTIONs ==================
  // ===== 1. withdraw SOL
  const amountSolWithdraw = new anchor.BN(0.2 * anchor.web3.LAMPORTS_PER_SOL);
  const withdrawSolIx = await program.methods
    .withdrawSol(amountSolWithdraw)
    .accountsStrict({
      master,
      vaultSol,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();
  // ===== 2. withdraw DONE Token
  const amountDoneTokenWithdraw = new anchor.BN(
    2.222 * 10 ** doneTokenMint.decimals
  );
  const withdrawDoneTokenIx = await program.methods
    .withdrawDoneToken(amountDoneTokenWithdraw)
    .accountsStrict({
      master,
      mintOfTokenBeingSent: doneTokenMint.address,
      tokenAccountOwnerPda: tokenAccountOwner,
      vaultToken,
      senderTokenAccount,
      signer: wallet.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .instruction();
  //
  // ================== SEND TX ==================
  try {
    const tx = new anchor.web3.Transaction().add(
      // ===== WITHDRAW
      withdrawSolIx,
      withdrawDoneTokenIx
    );
    const txLog = await anchor.web3.sendAndConfirmTransaction(connection, tx, [
      wallet.payer,
    ]);
    console.log("=> tx: ", txLog);
  } catch (error) {
    console.log("=> error: ", error);
  }

  // ===== GET MASTER DATA
  // const masterPda = await program.account.master.fetch(master);
  // console.log("masterPda: ", masterPda);
})();
