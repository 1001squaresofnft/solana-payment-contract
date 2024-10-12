import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as fs from "fs";
import idl from "../target/idl/mintedgem.json";
import { Mintedgem } from "../target/types/mintedgem";
import { randomInt } from "crypto";
import { getDoneTokenMint, programId } from "./utils/config";
import {
  getItemPayment,
  getMaster,
  getSenderTokenAccount,
  getTokenAccountOwner,
  getVaultToken,
} from "./utils/pda";

(async function main() {
  const connection = new anchor.web3.Connection(
    anchor.web3.clusterApiUrl("devnet"),
    "confirmed"
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
  const doneTokenMint = await getDoneTokenMint();

  // ================== CREATE PROGRAM ==================
  const program = new anchor.Program(idl as Mintedgem);

  // ================== GET ENTIRE THE PDAs ==================
  const [master] = getMaster();

  const [tokenAccountOwner] = getTokenAccountOwner();
  const [vaultToken] = getVaultToken(doneTokenMint);
  const [senderTokenAccount] = getSenderTokenAccount(doneTokenMint);
  //
  //
  const randomNumber = randomInt(0, 999999);
  console.log("randomNumber: ", randomNumber);
  const itemId = new anchor.BN(randomNumber);
  const amountDoneTokenCreatePayment = new anchor.BN(
    1.222 * 10 ** doneTokenMint.decimals
  );
  const [itemPayment] = getItemPayment(itemId);
  //

  // ================== create INSTRUCTIONs ==================
  //
  // ===== 1. Init tx DONE Token volume
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
    .accountsStrict({
      transactionDoneTokenVolume,
      signer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();
  //
  // ===== 2. Create payment by Done Token
  const createPaymentByDoneTokenIx = await program.methods
    .createPaymentByDone(itemId, amountDoneTokenCreatePayment)
    .accountsStrict({
      master,
      itemPayment,
      transactionDoneTokenVolume,
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

  //   ================== SEND TX ==================
  try {
    const tx = new anchor.web3.Transaction().add(
      // ===== CREATE PAYMENT BY DONE
      initTxDoneTokenvolumeIx,
      createPaymentByDoneTokenIx
    );
    const txLog = await anchor.web3.sendAndConfirmTransaction(connection, tx, [
      wallet.payer,
    ]);
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
