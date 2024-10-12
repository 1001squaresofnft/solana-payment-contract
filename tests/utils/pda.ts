import * as anchor from "@coral-xyz/anchor";
import { Mint } from "@solana/spl-token";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import "dotenv/config";
import { programId, wallet } from "./config";

// ========= PROGRAMS =========
// SEEDS
export const MASTER_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("master")
);

export const VAULT_SOL_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("vault_sol")
);

export const VAULT_TOKEN_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("vault_token")
);

export const TOKEN_ACCOUNT_OWNER_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("token_account_owner")
);

export const TRANSACTION_SOL_VOLUME = Buffer.from(
  anchor.utils.bytes.utf8.encode("transaction_sol_volume")
);

export const ITEM_PAYMENT = Buffer.from(
  anchor.utils.bytes.utf8.encode("item_payment")
);

// get PDAs
export function getMaster(): [anchor.web3.PublicKey, number] {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [MASTER_SEED],
    programId
  );
  return [address, bump];
}

export function getVaultSol(): [anchor.web3.PublicKey, number] {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [VAULT_SOL_SEED],
    programId
  );
  return [address, bump];
}

export function getAccountOwner(): [anchor.web3.PublicKey, number] {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [TOKEN_ACCOUNT_OWNER_SEED],
    programId
  );
  return [address, bump];
}

export function getVaultToken(
  doneTokenMint: Mint
): [anchor.web3.PublicKey, number] {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [VAULT_TOKEN_SEED, doneTokenMint.address.toBuffer()],
    programId
  );
  return [address, bump];
}

export function getTokenAccountOwner(): [anchor.web3.PublicKey, number] {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [TOKEN_ACCOUNT_OWNER_SEED],
    programId
  );
  return [address, bump];
}

export function getSenderTokenAccount(
  doneTokenMint: Mint
): [anchor.web3.PublicKey, number] {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      wallet.publicKey.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      doneTokenMint.address.toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID
  );
  return [address, bump];
}

export function getTransactionSolVolume(): [anchor.web3.PublicKey, number] {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [TRANSACTION_SOL_VOLUME, wallet.publicKey.toBuffer()],
    programId
  );
  return [address, bump];
}

export function getItemPayment(
  itemId: anchor.BN
): [anchor.web3.PublicKey, number] {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [ITEM_PAYMENT, itemId.toArrayLike(Buffer, "le", 8)],
    programId
  );
  return [address, bump];
}

// ========= For CPI to Raydium program =========
