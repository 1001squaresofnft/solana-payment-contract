// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { Mintedgem } from "../target/types/mintedgem";
// import {
//   initWalletWithSols,
//   mintTokenForWallet,
//   printAccountBalance,
//   printTokenBalanceSpl,
// } from "./helper";
// import * as spl from "@solana/spl-token";

// describe("mintedgem", () => {
//   // Configure the client to use the local cluster.
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const connection = provider.connection;

//   const program = anchor.workspace.Mintedgem as Program<Mintedgem>;

//   it("Is initialized!", async () => {
//     const user = await initWalletWithSols(10, connection);
//     const user2 = await initWalletWithSols(10, connection);

//     const amountDoneMint = new anchor.BN(1000 * anchor.web3.LAMPORTS_PER_SOL);
//     const { tokenAccount, tokenMint } = await mintTokenForWallet(
//       user,
//       connection,
//       amountDoneMint.toNumber()
//     );

//     // get the PDAs
//     const [masterPda, masterBump] =
//       anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("master")],
//         program.programId
//       );

//     const [vaultSol, vaultSolBump] =
//       anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("vault_sol")],
//         program.programId
//       );

//     const [tokenAccountOwnerPda, tokenAccountOwnerBump] =
//       anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("token_account_owner")],
//         program.programId
//       );

//     const [vaultToken, vaultTokenBump] =
//       anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("vault_token"), tokenMint.toBuffer()],
//         program.programId
//       );

//     // ===== hello ======

//     const listenerHelloEvent = program.addEventListener("Hello", (e, s) => {
//       console.log(`slot: ${s}, value: ${e.msg}`);
//     });

//     const txHello = await program.methods
//       .hello()
//       .accounts({})
//       .rpc();

//     console.log("=> ", txHello);

//     await new Promise((resolve) => setTimeout(resolve, 5000));

//     program.removeEventListener(listenerHelloEvent);
//   });
// });
