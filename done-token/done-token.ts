import {
  percentAmount,
  generateSigner,
  signerIdentity,
  createSignerFromKeypair,
} from "@metaplex-foundation/umi";
import {
  TokenStandard,
  createAndMint,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import * as anchor from "@coral-xyz/anchor";
import fs from "fs";
import "dotenv/config";
import { AuthorityType, setAuthority } from "@solana/spl-token";
import { Connection, PublicKey } from "@solana/web3.js";

// yarn add @metaplex-foundation/umi @metaplex-foundation/mpl-token-metadata @metaplex-foundation/umi-bundle-defaults

const umi = createUmi(process.env.RPC_URL!);
const connection = new Connection(process.env.RPC_URL!);

const keypairWeb3 = anchor.web3.Keypair.fromSecretKey(
  Uint8Array.from(
    JSON.parse(fs.readFileSync(process.env.WALLET_PATH!, "utf-8"))
  )
);
const keypairUmi = umi.eddsa.createKeypairFromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(process.env.WALLET_PATH!, "utf-8")))
);
const userWalletSigner = createSignerFromKeypair(umi, keypairUmi);
const wallet = new anchor.Wallet(keypairWeb3);

const metadata = {
  name: "DONE Token",
  symbol: "DONE",
  description: "$DONE token description",
  uri: "https://gray-capitalist-cephalopod-696.mypinata.cloud/ipfs/QmZ4A1Ym2eSsyx1NrseJe7PwCnhAL19e41Fk2XMXTejg1z",
};

const mint = generateSigner(umi);
umi.use(signerIdentity(userWalletSigner));
umi.use(mplTokenMetadata());

// const mint = publicKey("token_mint");

async function main() {
  // CREATE & MINT
  createAndMint(umi, {
    mint,
    authority: umi.identity,
    name: metadata.name,
    symbol: metadata.symbol,
    uri: metadata.uri,
    sellerFeeBasisPoints: percentAmount(0),
    decimals: 9,
    amount: 1001_000000000,
    tokenOwner: keypairUmi.publicKey,
    tokenStandard: TokenStandard.Fungible,
    isMutable: true,
  })
    .sendAndConfirm(umi)
    .then(async () => {
      console.log(
        "Successfully minted 1001 DONE tokens (",
        mint.publicKey,
        ")"
      );

      setAuthority(
        connection,
        keypairWeb3,
        new PublicKey(mint.publicKey),
        keypairWeb3.publicKey,
        AuthorityType.MintTokens,
        null 
      )
        .then(async (txId) => {
          console.log("Mint authority set to null: ", txId);

          setAuthority(
            connection,
            keypairWeb3,
            new PublicKey(mint.publicKey),
            keypairWeb3.publicKey,
            AuthorityType.FreezeAccount,
            null 
          )
            .then((txId) => {
              console.log("Freeze authority set to null: ", txId);
            })
            .catch((err) => {
              console.error("Error AuthorityType.FreezeAccount tokens: ", err);
            });
        })
        .catch((err) => {
          console.error("Error AuthorityType.MintTokens tokens: ", err);
        });
    })
    .catch((err) => {
      console.error("Error minting tokens: ", err);
    });
}

main();