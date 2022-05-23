import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { SwordPlay } from "../target/types/sword_play";

describe("sword-play", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SwordPlay as Program<SwordPlay>;

  it("Is initialized!", async () => {
    const userOne = (program.provider as anchor.AnchorProvider).wallet;
    const userTwo = anchor.web3.Keypair.generate();

    const [battleOneTwo] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("battle"),
        provider.wallet.publicKey.toBuffer(),
        userTwo.publicKey.toBuffer(),
      ],
      program.programId
    );

    const [playerOne] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("player"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    const [equipmentOne] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("equipment"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    const [playerTwo] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("player"), userTwo.publicKey.toBuffer()],
      program.programId
    );

    const [equipmentTwo] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("equipment"),
        userTwo.publicKey.toBuffer(),
      ],
      program.programId
    );

    // Add your test here.
    const init = await program.methods
      .initialize()
      .accounts({
        player: playerOne,
        equipment: equipmentOne,
      })
      .rpc();
    console.log("Your transaction signature", init);

    const init2 = await program.methods
      .initialize()
      .accounts({
        user: userTwo.publicKey,
        player: playerTwo,
        equipment: equipmentTwo,
      })
      .signers([userTwo])
      .rpc();
    console.log("Your transaction signature", init2);

    const game = await program.methods
      .setupBattle()
      .accounts({
        battle: battleOneTwo,
        playerOne: playerOne,
        equipmentOne: equipmentOne,
        userTwo: userTwo.publicKey,
        playerTwo: playerTwo,
        equipmentTwo: equipmentTwo,
      })
      .rpc();
    console.log("Your transaction signature", game);
  });
});
