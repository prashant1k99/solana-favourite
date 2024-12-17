import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as favourites from "../target/types/favourites";

describe("favourites", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Favourites as Program<favourites.Favourites>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.setFavourite(
      new anchor.BN(4),
      "blue",
      ["Coding", "Music"],
    ).rpc();
    console.log("Your transaction signature", tx);
  });
});
