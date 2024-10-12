import * as anchor from "@coral-xyz/anchor";
import idl from "../target/idl/mintedgem.json";
import { Mintedgem } from "../target/types/mintedgem";
import { connection, getDoneTokenMint, wallet } from "./utils/config";
import { getMaster } from "./utils/pda";

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

  // ================== create INSTRUCTIONs ==================
  // ===== 1. Set percent
  // 1.2 Pay with Sol
  const newPercent = 9900;
  const setPercentSolIx = await program.methods
    .setPercentPayWSol(newPercent)
    .accountsStrict({
      master,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();
  //
  // 1.2 Pay with DONE token
  const setPercentDoneTokenIx = await program.methods
    .setPercentPayWDoneToken(newPercent)
    .accountsStrict({
      master,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();
  //
  // ================== SEND TX ==================
  try {
    const tx = new anchor.web3.Transaction().add(
      // ===== SET PERCENT
      setPercentSolIx,
      setPercentDoneTokenIx
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
