import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import { AnchorProvider } from "@coral-xyz/anchor";

export const initWalletWithSols = async (sols: number, connection) => {
  const wallet = anchor.web3.Keypair.generate();

  const signature = await connection.requestAirdrop(
    wallet.publicKey,
    sols * anchor.web3.LAMPORTS_PER_SOL
  );

  await connection.confirmTransaction(signature);

  return wallet;
};

export async function printAccountBalance(account: anchor.web3.PublicKey) {
  const balance = await anchor.getProvider().connection.getBalance(account);
  console.log(`${account} has ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
}

export function lamportsToSol(lamports: number): number {
  return lamports / 1_000_000_000;
}

export const mintTokenForWallet = async (
  wallet: anchor.web3.Signer,
  connection: anchor.web3.Connection,
  amount: number
) => {
  const tokenMint = await spl.createMint(
    connection,
    wallet,
    wallet.publicKey,
    wallet.publicKey,
    9
  );

  // associate token_mint to alice account
  const tokenAccount = await spl.getOrCreateAssociatedTokenAccount(
    connection,
    wallet,
    tokenMint,
    wallet.publicKey
  );

  await spl.mintTo(
    connection,
    wallet,
    tokenMint,
    tokenAccount.address,
    wallet,
    amount
  );

  // const disableMintingTx = new anchor.web3.Transaction().add(
  //   spl.createSetAuthorityInstruction(
  //     tokenMint,
  //     wallet.publicKey,
  //     spl.AuthorityType.MintTokens,
  //     null
  //   )
  // );

  // await anchor.web3.sendAndConfirmTransaction(connection, disableMintingTx, [
  //   wallet,
  // ]);

  return { tokenMint, tokenAccount };
};

export async function printTokenBalanceSpl(connection, tokenAccount) {
  const info = await spl.getAccount(connection, tokenAccount);
  const amount = Number(info.amount);
  const mint = await spl.getMint(connection, info.mint);
  // console.log("decimal: ", mint.decimals);
  const balance = amount / 10 ** mint.decimals;
  console.log("Balance (using Solana-Web3.js): ", balance);
  return balance;
}

export async function getAssociatedTokenAccount(mint, owner) {
  const tokenAccount = await spl.getAssociatedTokenAddress(
    mint,
    owner,
    true,
    spl.TOKEN_PROGRAM_ID,
    spl.ASSOCIATED_TOKEN_PROGRAM_ID
  );

  return tokenAccount;
}
