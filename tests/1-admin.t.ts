import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import idl from "../target/idl/mintedgem.json";
import { Mintedgem } from "../target/types/mintedgem";
import {
  connection,
  getDoneTokenMint,
  PERCENT_PAY_W_SOL,
  PERCENT_W_DONE_TOKEN,
  wallet,
} from "./utils/config";
import {
  getMaster,
  getTokenAccountOwner,
  getVaultSol,
  getVaultToken,
} from "./utils/pda";

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
  //
  // ================== create INSTRUCTIONs ==================
  // ============ 1. init MASTER
  const initMasterIx = await program.methods
    .initialize(PERCENT_PAY_W_SOL, PERCENT_W_DONE_TOKEN)
    .accountsStrict({
      master,
      signer: wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();
  //
  // ========== 2. init vault SOL
  const initVaultSolIx = await program.methods
    .initVaultSol()
    .accountsStrict({
      master,
      vaultSol,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .instruction();
  // ============ 3. init vault DONE Token
  const initVaultDoneTokenIx = await program.methods
    .initVaultDoneToken()
    .accountsStrict({
      master,
      mintOfTokenBeingSent: doneTokenMint.address,
      tokenAccountOwnerPda: tokenAccountOwner,
      vaultToken,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .instruction();

  // ================== SEND TX ==================
  try {
    const tx = new anchor.web3.Transaction().add(
      // ===== INIT MASTER & VAULTS
      initMasterIx,
      initVaultSolIx,
      initVaultDoneTokenIx
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

  // ===== GET MASTER DATA
  // const masterPda = await program.account.master.fetch(master);
  // console.log("masterPda: ", masterPda);
})();
