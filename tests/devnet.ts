import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import * as fs from "fs";
import idl from "../target/idl/mintedgem.json";
import { Mintedgem } from "../target/types/mintedgem";

(async function main() {
  const connection = new anchor.web3.Connection(
    "https://api.devnet.solana.com"
  );

  const keypair = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(
        fs.readFileSync("/Users/lainhathoang/.config/solana/id.json", "utf-8")
      )
    )
  );
  // ===== setup WALLET
  const wallet = new anchor.Wallet(keypair);
  console.log("interact using wallet: ", wallet.publicKey.toBase58());
  // ==== setup PROVIDER
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  anchor.setProvider(provider);

  // DECLARE PROGRAM ID & DONE token
  const programId = new anchor.web3.PublicKey(
    "p4oawzVcmB9BSTKtDjRNhgbR4qFdHufnz5Zft2wNBF5"
  );

  const doneTokenMint = new anchor.web3.PublicKey(
    "HETVuKo7hyJXkukR3viMJm9uh7P2RJD8Zg1sdGJQjCtm"
  );

  // CREATE PROGRAM
  const program = new anchor.Program(idl as Mintedgem, programId);

  // GET ENTIRE THE PDAs
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
    [Buffer.from("vault_token"), doneTokenMint.toBuffer()],
    programId
  );

  const [senderTokenAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      wallet.publicKey.toBuffer(),
      spl.TOKEN_PROGRAM_ID.toBuffer(),
      doneTokenMint.toBuffer(),
    ],
    spl.ASSOCIATED_TOKEN_PROGRAM_ID
  );

  spl.getOrCreateAssociatedTokenAccount;

  // create INSTRUCTIONs
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
      mintOfTokenBeingSent: doneTokenMint,
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
  const amountDoneToken = new anchor.BN(1_000_000 * 10 ** 6);
  const depositDoneTokenIx = await program.methods
    .depositDoneToken(amountDoneToken)
    .accounts({
      master,
      mintOfTokenBeingSent: doneTokenMint,
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

  // ======== SEND TX
  try {
    const tx = new anchor.web3.Transaction().add(depositDoneTokenIx);
    const txLog = await anchor.web3.sendAndConfirmTransaction(connection, tx, [
      wallet.payer,
    ]);
    console.log("=> tx: ", txLog);
  } catch (error) {
    console.log("=> error: ", error);
  }

  // ======== FETCH data
  console.log("vaultSol: ", vaultSol.toBase58());
  console.log("vaultToken: ", vaultToken.toBase58());
  const masterFetched = await program.account.master.fetch(master);
  console.log(masterFetched);
})();
