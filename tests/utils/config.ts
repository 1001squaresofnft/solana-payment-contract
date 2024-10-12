import * as anchor from "@coral-xyz/anchor";
import { getMint, Mint } from "@solana/spl-token";
import * as fs from "fs";
import "dotenv/config";

// ENVIROMENT
export const connection = new anchor.web3.Connection(
  process.env.RPC_URL,
  "confirmed"
);
export const keypair = anchor.web3.Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(fs.readFileSync(process.env.WALLET_PATH, "utf-8")))
);
export const wallet = new anchor.Wallet(keypair);

// ADDRESSES
export const PROGRAM_ID = process.env.PROGRAM_ID;
export const DONE_TOKEN_PUBKEY = process.env.DONE_TOKEN_MINT;

export const PERCENT_PAY_W_SOL = Number(process.env.PERCENT_PAY_W_SOL);
export const PERCENT_W_DONE_TOKEN = Number(process.env.PERCENT_W_DONE_TOKEN);

// CONSTS
export const programId = new anchor.web3.PublicKey(PROGRAM_ID);
export const doneTokenPubkey = new anchor.web3.PublicKey(DONE_TOKEN_PUBKEY);

// FUNCTIONS
export async function getDoneTokenMint(): Promise<Mint> {
  return await getMint(
    connection,
    new anchor.web3.PublicKey(DONE_TOKEN_PUBKEY)
  );
}

// ============== RAYDIUM CP SWAP
// CP PROGRAM ADDRESSES
export const DEVNET_CP_SWAP_PROGRAM_ID = new anchor.web3.PublicKey(
  "CPMDWBwJDtYax9qW7AyRuVC19Cc4L4Vcy4n2BHAbHkCW"
);
// export const CP_SWAP_PROGRAM_ID = new anchor.web3.PublicKey("");
//
// CP PROGRAM SEEDS
export const POOL_SEED = Buffer.from(anchor.utils.bytes.utf8.encode("pool"));
export const AMM_CONFIG_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("amm_config")
);
export const POOL_AUTH_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("vault_and_lp_mint_auth_seed")
);
export const POOL_VAULT_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("pool_vault")
);
export const ORACLE_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("observation")
);
//
// CP PROGRAM FUNCTION TOOLS
export function u16ToBytes(num: number) {
  const arr = new ArrayBuffer(2);
  const view = new DataView(arr);
  view.setUint16(0, num, false);
  return new Uint8Array(arr);
}

export async function getAuthAddress(
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [POOL_AUTH_SEED],
    programId
  );
  return [address, bump];
}

export async function getPoolAddress(
  ammConfig: anchor.web3.PublicKey,
  tokenMint0: anchor.web3.PublicKey,
  tokenMint1: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      POOL_SEED,
      ammConfig.toBuffer(),
      tokenMint0.toBuffer(),
      tokenMint1.toBuffer(),
    ],
    programId
  );
  return [address, bump];
}

export async function getPoolVaultAddress(
  pool: anchor.web3.PublicKey,
  vaultTokenMint: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [POOL_VAULT_SEED, pool.toBuffer(), vaultTokenMint.toBuffer()],
    programId
  );
  return [address, bump];
}

export async function getAmmConfigAddress(
  index: number,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [AMM_CONFIG_SEED, u16ToBytes(index)],
    programId
  );
  return [address, bump];
}

export async function getOrcleAccountAddress(
  pool: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  const [address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [ORACLE_SEED, pool.toBuffer()],
    programId
  );
  return [address, bump];
}
