import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SwapperProgram } from "../target/types/swapper_program";


describe("swapper_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SwapperProgram as Program<SwapperProgram>;


  it("Initializes the program", async () => {
    const tx = await program.methods
      .intiialize()
      .accounts({})
      .rpc();

    console.log("Transaction Signature:", tx);
  });


});
