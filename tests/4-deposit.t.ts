import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import idl from "../target/idl/mintedgem.json";
import { Mintedgem } from "../target/types/mintedgem";
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
  const [vaultToken] = getVaultToken(doneTokenMint);
  const [tokenAccountOwner] = getTokenAccountOwner();
  const [senderTokenAccount] = getSenderTokenAccount(doneTokenMint);

  // ================== create INSTRUCTIONs ==================
  // ====== 1. deposit SOL
  const amountSol = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
  console.log("amountSol: ", amountSol.toNumber());
  const depositSolIx = await program.methods
    .depositSol(amountSol)
    .accountsStrict({
      master,
      vaultSol,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();
  // ====== 2. deposit DONE Token
  const amountDoneToken = new anchor.BN(10 * 10 ** doneTokenMint.decimals);
  const depositDoneTokenIx = await program.methods
    .depositDoneToken(amountDoneToken)
    .accountsStrict({
      master,
      mintOfTokenBeingSent: doneTokenMint.address,
      tokenAccountOwnerPda: tokenAccountOwner,
      vaultToken,
      senderTokenAccount,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .instruction();
  // ================== SEND TX ==================
  try {
    const tx = new anchor.web3.Transaction().add(
      // ===== WITHDRAW
      depositSolIx,
      depositDoneTokenIx
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
