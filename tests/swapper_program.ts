import * as anchor from "@coral-xyz/anchor";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { Program } from "@coral-xyz/anchor";
import { SwapperProgram } from "../target/types/swapper_program";
import { Keypair, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("swapper_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SwapperProgram as Program<SwapperProgram>;
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;


  // it("Initializes the program", async () => {
  //   const tx = await program.methods
  //     .initialize()
  //     .accounts({})
  //     .rpc();

  //   console.log("Transaction Signature:", tx);
  // });

  it("Calls execute_swap and prints logs", async () => {
    // Replace with the actual Pubkeys you want to test
    const raydiumPoolPubkey = new anchor.web3.PublicKey("5Aab9Fxm7CY3YtGjCS9HyT6trMthsWVnP2j2CEb2xFer");
    const meteoraPoolPubkey = new anchor.web3.PublicKey("LiDoU8ymvYptqxenJ4YpcURBchn4ef63tcbdznBCKJh");

    // Sample inputs, adjust as needed
    const amountIn = new anchor.BN(1000);
    const minAmountOut = new anchor.BN(900);
    const quoteMint = new anchor.web3.PublicKey("So11111111111111111111111111111111111111112"); // WSOL example
    const tokenMint = new anchor.web3.PublicKey("USDCoctVLVnvTXBEuP9s8hntucdJokbo17RwHuNXemT"); // Replace with a real token mint

    // Call execute_swap
    const tx = await program.methods
      .executeSwap(amountIn, minAmountOut, quoteMint, tokenMint)
      .accounts({
        raydiumPool: raydiumPoolPubkey,
        meteoraPool: meteoraPoolPubkey,
        // Add any other required accounts here
      })
      .rpc();

    console.log("Transaction signature:", tx);
  });
});
